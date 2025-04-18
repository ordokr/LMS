use super::models::{Quiz, Question, Answer, QuestionContent, AnswerType, StudyMode, QuizVisibility};
use super::storage::HybridQuizStore;
use uuid::Uuid;
use std::sync::Arc;
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// Supported export formats
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ExportFormat {
    /// JSON format (default)
    Json,
    /// CSV format (for simple quizzes)
    Csv,
    /// Markdown format (for documentation)
    Markdown,
    /// Anki-compatible format
    Anki,
    /// Quizlet-compatible format
    Quizlet,
}

/// Quiz export options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportOptions {
    /// Export format
    pub format: ExportFormat,
    /// Whether to include explanations
    pub include_explanations: bool,
    /// Whether to include metadata
    pub include_metadata: bool,
    /// Whether to include statistics
    pub include_statistics: bool,
    /// Whether to include images
    pub include_images: bool,
    /// Whether to include audio
    pub include_audio: bool,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            format: ExportFormat::Json,
            include_explanations: true,
            include_metadata: true,
            include_statistics: false,
            include_images: true,
            include_audio: true,
        }
    }
}

/// Quiz export data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizExport {
    /// Quiz data
    pub quiz: Quiz,
    /// Export format
    pub format: ExportFormat,
    /// Export timestamp
    pub timestamp: DateTime<Utc>,
    /// Export version
    pub version: String,
    /// Statistics (if included)
    pub statistics: Option<QuizStatistics>,
}

/// Quiz statistics for export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizStatistics {
    /// Number of times the quiz was taken
    pub times_taken: usize,
    /// Average score
    pub average_score: f32,
    /// Question difficulty ratings
    pub question_difficulties: Vec<(Uuid, f32)>,
}

/// Quiz import/export engine
pub struct QuizExportEngine {
    store: Arc<HybridQuizStore>,
}

impl QuizExportEngine {
    pub fn new(store: Arc<HybridQuizStore>) -> Self {
        Self { store }
    }
    
    /// Export a quiz to a file
    pub async fn export_quiz_to_file(
        &self,
        quiz_id: Uuid,
        path: &Path,
        options: ExportOptions,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let export_data = self.prepare_quiz_export(quiz_id, options).await?;
        
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        
        match options.format {
            ExportFormat::Json => {
                serde_json::to_writer_pretty(writer, &export_data)?;
            }
            ExportFormat::Csv => {
                self.export_to_csv(writer, &export_data)?;
            }
            ExportFormat::Markdown => {
                self.export_to_markdown(writer, &export_data)?;
            }
            ExportFormat::Anki => {
                self.export_to_anki(writer, &export_data)?;
            }
            ExportFormat::Quizlet => {
                self.export_to_quizlet(writer, &export_data)?;
            }
        }
        
        Ok(())
    }
    
    /// Export a quiz to a byte array
    pub async fn export_quiz(
        &self,
        quiz_id: Uuid,
        options: ExportOptions,
    ) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
        let export_data = self.prepare_quiz_export(quiz_id, options).await?;
        
        let mut buffer = Vec::new();
        let writer = BufWriter::new(&mut buffer);
        
        match options.format {
            ExportFormat::Json => {
                serde_json::to_writer_pretty(writer, &export_data)?;
            }
            ExportFormat::Csv => {
                self.export_to_csv(writer, &export_data)?;
            }
            ExportFormat::Markdown => {
                self.export_to_markdown(writer, &export_data)?;
            }
            ExportFormat::Anki => {
                self.export_to_anki(writer, &export_data)?;
            }
            ExportFormat::Quizlet => {
                self.export_to_quizlet(writer, &export_data)?;
            }
        }
        
        Ok(buffer)
    }
    
    /// Import a quiz from a file
    pub async fn import_quiz_from_file(
        &self,
        path: &Path,
    ) -> Result<Uuid, Box<dyn Error + Send + Sync>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        
        let quiz = match extension {
            "json" => {
                let export_data: QuizExport = serde_json::from_reader(reader)?;
                export_data.quiz
            }
            "csv" => {
                self.import_from_csv(reader)?
            }
            "md" => {
                self.import_from_markdown(reader)?
            }
            "apkg" => {
                self.import_from_anki(reader)?
            }
            "txt" => {
                // Try to detect format
                let mut content = String::new();
                reader.get_ref().read_to_string(&mut content)?;
                
                if content.contains("CLOZE") || content.contains("BASIC") {
                    // Likely Anki format
                    self.import_from_anki_text(&content)?
                } else if content.contains("\t") {
                    // Likely Quizlet format
                    self.import_from_quizlet_text(&content)?
                } else {
                    // Default to CSV-like format
                    self.import_from_csv(BufReader::new(content.as_bytes()))?
                }
            }
            _ => {
                return Err(format!("Unsupported file format: {}", extension).into());
            }
        };
        
        // Store the imported quiz
        self.store.store_quiz(&quiz).await?;
        
        Ok(quiz.id)
    }
    
    /// Import a quiz from a byte array
    pub async fn import_quiz(
        &self,
        data: &[u8],
        format: ExportFormat,
    ) -> Result<Uuid, Box<dyn Error + Send + Sync>> {
        let reader = BufReader::new(data);
        
        let quiz = match format {
            ExportFormat::Json => {
                let export_data: QuizExport = serde_json::from_reader(reader)?;
                export_data.quiz
            }
            ExportFormat::Csv => {
                self.import_from_csv(reader)?
            }
            ExportFormat::Markdown => {
                self.import_from_markdown(reader)?
            }
            ExportFormat::Anki => {
                self.import_from_anki(reader)?
            }
            ExportFormat::Quizlet => {
                self.import_from_quizlet(reader)?
            }
        };
        
        // Store the imported quiz
        self.store.store_quiz(&quiz).await?;
        
        Ok(quiz.id)
    }
    
    /// Prepare quiz export data
    async fn prepare_quiz_export(
        &self,
        quiz_id: Uuid,
        options: ExportOptions,
    ) -> Result<QuizExport, Box<dyn Error + Send + Sync>> {
        // Get the quiz
        let mut quiz = self.store.get_quiz(quiz_id).await?;
        
        // Filter out data based on options
        if !options.include_explanations {
            for question in &mut quiz.questions {
                question.explanation = None;
            }
        }
        
        if !options.include_metadata {
            quiz.created_at = None;
            quiz.updated_at = None;
            quiz.author_id = None;
        }
        
        if !options.include_images {
            for question in &mut quiz.questions {
                question.content.image_url = None;
                
                for choice in &mut question.choices {
                    choice.image_url = None;
                }
            }
        }
        
        if !options.include_audio {
            for question in &mut quiz.questions {
                question.content.audio_url = None;
            }
        }
        
        // Get statistics if requested
        let statistics = if options.include_statistics {
            Some(self.get_quiz_statistics(quiz_id).await?)
        } else {
            None
        };
        
        Ok(QuizExport {
            quiz,
            format: options.format,
            timestamp: Utc::now(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            statistics,
        })
    }
    
    /// Get quiz statistics
    async fn get_quiz_statistics(
        &self,
        quiz_id: Uuid,
    ) -> Result<QuizStatistics, Box<dyn Error + Send + Sync>> {
        // In a real implementation, this would query the database for statistics
        // For now, we'll return dummy data
        Ok(QuizStatistics {
            times_taken: 0,
            average_score: 0.0,
            question_difficulties: Vec::new(),
        })
    }
    
    /// Export to CSV format
    fn export_to_csv<W: Write>(
        &self,
        mut writer: W,
        export_data: &QuizExport,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Write header
        writeln!(writer, "Question,Answer,Explanation")?;
        
        // Write questions
        for question in &export_data.quiz.questions {
            let question_text = &question.content.text;
            
            // Get correct answer text
            let answer_text = match &question.correct_answer {
                Answer::Choice(choice_id) => {
                    let choice = question.choices.iter()
                        .find(|c| &c.id == choice_id)
                        .ok_or_else(|| format!("Choice not found: {}", choice_id))?;
                    
                    choice.text.clone()
                }
                Answer::Text(text) => text.clone(),
                _ => "".to_string(),
            };
            
            // Get explanation
            let explanation = question.explanation.as_deref().unwrap_or("");
            
            // Write CSV row
            writeln!(writer, "\"{}\",\"{}\",\"{}\"", 
                question_text.replace("\"", "\"\""), 
                answer_text.replace("\"", "\"\""), 
                explanation.replace("\"", "\"\"")
            )?;
        }
        
        Ok(())
    }
    
    /// Export to Markdown format
    fn export_to_markdown<W: Write>(
        &self,
        mut writer: W,
        export_data: &QuizExport,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Write title
        writeln!(writer, "# {}", export_data.quiz.title)?;
        
        // Write description if available
        if let Some(description) = &export_data.quiz.description {
            writeln!(writer, "\n{}\n", description)?;
        }
        
        // Write metadata
        writeln!(writer, "## Metadata\n")?;
        writeln!(writer, "- **Format**: {:?}", export_data.format)?;
        writeln!(writer, "- **Exported**: {}", export_data.timestamp)?;
        writeln!(writer, "- **Version**: {}", export_data.version)?;
        
        // Write statistics if available
        if let Some(stats) = &export_data.statistics {
            writeln!(writer, "\n## Statistics\n")?;
            writeln!(writer, "- **Times Taken**: {}", stats.times_taken)?;
            writeln!(writer, "- **Average Score**: {:.1}%", stats.average_score * 100.0)?;
        }
        
        // Write questions
        writeln!(writer, "\n## Questions\n")?;
        
        for (i, question) in export_data.quiz.questions.iter().enumerate() {
            writeln!(writer, "### Question {}\n", i + 1)?;
            writeln!(writer, "{}\n", question.content.text)?;
            
            // Write choices for multiple choice questions
            if matches!(question.answer_type, AnswerType::MultipleChoice | AnswerType::TrueFalse) {
                for (j, choice) in question.choices.iter().enumerate() {
                    let is_correct = match &question.correct_answer {
                        Answer::Choice(id) => *id == choice.id,
                        _ => false,
                    };
                    
                    let marker = if is_correct { "- [x]" } else { "- [ ]" };
                    writeln!(writer, "{} {}", marker, choice.text)?;
                }
                writeln!(writer)?;
            }
            
            // Write correct answer for other question types
            if !matches!(question.answer_type, AnswerType::MultipleChoice | AnswerType::TrueFalse) {
                writeln!(writer, "**Answer**: ")?;
                
                match &question.correct_answer {
                    Answer::Text(text) => writeln!(writer, "{}\n", text)?,
                    _ => writeln!(writer, "\n")?,
                }
            }
            
            // Write explanation if available
            if let Some(explanation) = &question.explanation {
                writeln!(writer, "**Explanation**: {}\n", explanation)?;
            }
        }
        
        Ok(())
    }
    
    /// Export to Anki format
    fn export_to_anki<W: Write>(
        &self,
        mut writer: W,
        export_data: &QuizExport,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Anki expects a specific format for import
        // We'll create a simple text file that can be imported into Anki
        
        // Write header
        writeln!(writer, "#separator:tab")?;
        writeln!(writer, "#html:true")?;
        writeln!(writer, "#deck:{}", export_data.quiz.title)?;
        writeln!(writer, "#columns:Front\tBack")?;
        
        // Write questions
        for question in &export_data.quiz.questions {
            let front = &question.content.text;
            
            // Build the back (answer) side
            let mut back = String::new();
            
            match &question.answer_type {
                AnswerType::MultipleChoice | AnswerType::TrueFalse => {
                    if let Answer::Choice(choice_id) = &question.correct_answer {
                        let choice = question.choices.iter()
                            .find(|c| &c.id == choice_id)
                            .ok_or_else(|| format!("Choice not found: {}", choice_id))?;
                        
                        back.push_str(&choice.text);
                    }
                }
                _ => {
                    if let Answer::Text(text) = &question.correct_answer {
                        back.push_str(text);
                    }
                }
            }
            
            // Add explanation if available
            if let Some(explanation) = &question.explanation {
                back.push_str("<br><br><i>");
                back.push_str(explanation);
                back.push_str("</i>");
            }
            
            // Write the card
            writeln!(writer, "{}\t{}", front, back)?;
        }
        
        Ok(())
    }
    
    /// Export to Quizlet format
    fn export_to_quizlet<W: Write>(
        &self,
        mut writer: W,
        export_data: &QuizExport,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Quizlet expects a tab-separated format
        // term TAB definition
        
        for question in &export_data.quiz.questions {
            let term = &question.content.text;
            
            // Get the definition (answer)
            let mut definition = String::new();
            
            match &question.answer_type {
                AnswerType::MultipleChoice | AnswerType::TrueFalse => {
                    if let Answer::Choice(choice_id) = &question.correct_answer {
                        let choice = question.choices.iter()
                            .find(|c| &c.id == choice_id)
                            .ok_or_else(|| format!("Choice not found: {}", choice_id))?;
                        
                        definition.push_str(&choice.text);
                    }
                }
                _ => {
                    if let Answer::Text(text) = &question.correct_answer {
                        definition.push_str(text);
                    }
                }
            }
            
            // Add explanation if available
            if let Some(explanation) = &question.explanation {
                definition.push_str(" (");
                definition.push_str(explanation);
                definition.push_str(")");
            }
            
            // Write the term-definition pair
            writeln!(writer, "{}\t{}", term, definition)?;
        }
        
        Ok(())
    }
    
    /// Import from CSV format
    fn import_from_csv<R: Read>(
        &self,
        reader: R,
    ) -> Result<Quiz, Box<dyn Error + Send + Sync>> {
        let mut csv_reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(reader);
        
        let mut quiz = Quiz::new("Imported Quiz".to_string(), None);
        quiz.study_mode = StudyMode::Flashcards;
        
        for result in csv_reader.records() {
            let record = result?;
            
            if record.len() < 2 {
                continue;
            }
            
            let question_text = record.get(0).unwrap_or("").to_string();
            let answer_text = record.get(1).unwrap_or("").to_string();
            let explanation = record.get(2).unwrap_or("").to_string();
            
            let content = QuestionContent {
                text: question_text,
                rich_text: None,
                image_url: None,
                audio_url: None,
            };
            
            let mut question = Question::new(quiz.id, content, AnswerType::ShortAnswer);
            question.correct_answer = Answer::Text(answer_text);
            
            if !explanation.is_empty() {
                question.explanation = Some(explanation);
            }
            
            quiz.add_question(question);
        }
        
        Ok(quiz)
    }
    
    /// Import from Markdown format
    fn import_from_markdown<R: Read>(
        &self,
        reader: R,
    ) -> Result<Quiz, Box<dyn Error + Send + Sync>> {
        let mut content = String::new();
        BufReader::new(reader).read_to_string(&mut content)?;
        
        let lines: Vec<&str> = content.lines().collect();
        
        // Extract title
        let title = if !lines.is_empty() && lines[0].starts_with("# ") {
            lines[0][2..].to_string()
        } else {
            "Imported Quiz".to_string()
        };
        
        let mut quiz = Quiz::new(title, None);
        quiz.study_mode = StudyMode::Mixed;
        
        let mut current_question: Option<Question> = None;
        let mut in_choices = false;
        
        for line in lines.iter().skip(1) {
            let trimmed = line.trim();
            
            if trimmed.starts_with("### Question ") {
                // Save previous question if exists
                if let Some(q) = current_question.take() {
                    quiz.add_question(q);
                }
                
                // Start new question
                in_choices = false;
                continue;
            }
            
            if let Some(ref mut question) = current_question {
                if trimmed.starts_with("- [") {
                    // This is a choice
                    in_choices = true;
                    
                    let is_correct = trimmed.starts_with("- [x]");
                    let choice_text = trimmed[5..].trim().to_string();
                    
                    let choice_id = question.add_choice(choice_text);
                    
                    if is_correct {
                        question.correct_answer = Answer::Choice(choice_id);
                    }
                } else if trimmed.starts_with("**Answer**:") {
                    // This is the answer for non-multiple choice questions
                    let answer_text = trimmed["**Answer**:".len()..].trim().to_string();
                    question.correct_answer = Answer::Text(answer_text);
                } else if trimmed.starts_with("**Explanation**:") {
                    // This is the explanation
                    let explanation = trimmed["**Explanation**:".len()..].trim().to_string();
                    question.explanation = Some(explanation);
                } else if !trimmed.is_empty() && !in_choices && question.content.text.is_empty() {
                    // This is the question text
                    question.content.text = trimmed.to_string();
                }
            } else if !trimmed.is_empty() && !trimmed.starts_with("#") {
                // Start a new question
                let content = QuestionContent {
                    text: trimmed.to_string(),
                    rich_text: None,
                    image_url: None,
                    audio_url: None,
                };
                
                current_question = Some(Question::new(quiz.id, content, AnswerType::MultipleChoice));
            }
        }
        
        // Add the last question
        if let Some(q) = current_question {
            quiz.add_question(q);
        }
        
        Ok(quiz)
    }
    
    /// Import from Anki format
    fn import_from_anki<R: Read>(
        &self,
        reader: R,
    ) -> Result<Quiz, Box<dyn Error + Send + Sync>> {
        // For simplicity, we'll assume this is a text export from Anki
        // In a real implementation, this would handle .apkg files
        
        let mut content = String::new();
        BufReader::new(reader).read_to_string(&mut content)?;
        
        self.import_from_anki_text(&content)
    }
    
    /// Import from Anki text format
    fn import_from_anki_text(
        &self,
        content: &str,
    ) -> Result<Quiz, Box<dyn Error + Send + Sync>> {
        let mut quiz = Quiz::new("Imported from Anki".to_string(), None);
        quiz.study_mode = StudyMode::Flashcards;
        
        let lines: Vec<&str> = content.lines().collect();
        
        // Skip header lines
        let mut i = 0;
        while i < lines.len() && lines[i].starts_with("#") {
            if lines[i].starts_with("#deck:") {
                quiz.title = lines[i]["#deck:".len()..].trim().to_string();
            }
            i += 1;
        }
        
        // Process cards
        while i < lines.len() {
            let line = lines[i].trim();
            
            if line.is_empty() {
                i += 1;
                continue;
            }
            
            let parts: Vec<&str> = line.split('\t').collect();
            
            if parts.len() >= 2 {
                let front = parts[0].to_string();
                let back = parts[1].to_string();
                
                // Extract explanation if present
                let (answer, explanation) = if back.contains("<br><br><i>") {
                    let parts: Vec<&str> = back.split("<br><br><i>").collect();
                    (parts[0].to_string(), Some(parts[1].replace("</i>", "")))
                } else {
                    (back, None)
                };
                
                let content = QuestionContent {
                    text: front,
                    rich_text: None,
                    image_url: None,
                    audio_url: None,
                };
                
                let mut question = Question::new(quiz.id, content, AnswerType::ShortAnswer);
                question.correct_answer = Answer::Text(answer);
                question.explanation = explanation;
                
                quiz.add_question(question);
            }
            
            i += 1;
        }
        
        Ok(quiz)
    }
    
    /// Import from Quizlet format
    fn import_from_quizlet<R: Read>(
        &self,
        reader: R,
    ) -> Result<Quiz, Box<dyn Error + Send + Sync>> {
        let mut content = String::new();
        BufReader::new(reader).read_to_string(&mut content)?;
        
        self.import_from_quizlet_text(&content)
    }
    
    /// Import from Quizlet text format
    fn import_from_quizlet_text(
        &self,
        content: &str,
    ) -> Result<Quiz, Box<dyn Error + Send + Sync>> {
        let mut quiz = Quiz::new("Imported from Quizlet".to_string(), None);
        quiz.study_mode = StudyMode::Flashcards;
        
        for line in content.lines() {
            let line = line.trim();
            
            if line.is_empty() {
                continue;
            }
            
            let parts: Vec<&str> = line.split('\t').collect();
            
            if parts.len() >= 2 {
                let term = parts[0].to_string();
                let definition = parts[1].to_string();
                
                // Extract explanation if present
                let (answer, explanation) = if definition.contains(" (") && definition.ends_with(")") {
                    let idx = definition.rfind(" (").unwrap();
                    (
                        definition[..idx].to_string(),
                        Some(definition[idx + 2..definition.len() - 1].to_string())
                    )
                } else {
                    (definition, None)
                };
                
                let content = QuestionContent {
                    text: term,
                    rich_text: None,
                    image_url: None,
                    audio_url: None,
                };
                
                let mut question = Question::new(quiz.id, content, AnswerType::ShortAnswer);
                question.correct_answer = Answer::Text(answer);
                question.explanation = explanation;
                
                quiz.add_question(question);
            }
        }
        
        Ok(quiz)
    }
}
