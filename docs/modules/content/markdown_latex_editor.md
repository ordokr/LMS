# Markdown/LaTeX Editor Implementation

_Last updated: 2025-04-18_

## Overview

This document outlines the implementation details for the Markdown/LaTeX editor component in the Ordo project. The editor is part of the Content Creation & Management module and provides advanced content editing capabilities for educational materials.

## Framework and Crate Selection

For the Ordo project's requirements—offline-first, modular, performant, and Rust/Haskell/Tauri/Leptos stack—the following frameworks and crates have been selected:

### Markdown Editor

#### Rust Crates

**comrak**

- **Purpose**: Full CommonMark-compliant Markdown parser and renderer in pure Rust
- **Features**:
  - Converts Markdown to HTML or AST
  - Supports extensions (tables, autolinks, etc.)
  - Fast, safe, and actively maintained
- **Integration**:
  - Easily called from Rust backend (Tauri commands) or directly in Leptos components
  - Can be used for both preview and export

**Example**:

```rust
use comrak::{markdown_to_html, ComrakOptions};
let html = markdown_to_html("# Hello, **world**!", &ComrakOptions::default());
```

#### Frontend Integration

- **Leptos**: Use textarea or contenteditable for input; render preview using comrak output
- **Tauri**: Use IPC to call Rust-side parsing/rendering if needed for security or advanced features
- **Offline**: All parsing/rendering is local, no dependency on external services

### LaTeX Editor

#### Rust Crates

**rusttex**

- **Purpose**: Programmatic generation of LaTeX documents in Rust
- **Features**:
  - Build LaTeX documents using Rust code
  - Suitable for exporting quizzes, assignments, or notes as .tex files
  - Early stage, but promising for integration with Rust-centric apps

**Example**:

```rust
use rusttex::{ContentBuilder, DocumentClass, ClassOptions};
let mut doc = ContentBuilder::new();
doc.document_class(DocumentClass::Article, ClassOptions::A4);
doc.title("Quiz Export");
// ... add content
println!("{}", doc.build_document());
```

**Note**: For rendering LaTeX to HTML or PDF locally, you may need to bundle a WASM-based renderer (e.g., KaTeX for HTML math rendering in the frontend) or shell out to a local TeX engine for PDF export.

### Haskell Options (for backend or advanced processing)

- **pandoc**: The gold standard for Markdown/LaTeX conversion, supports many formats and is scriptable from Haskell.
- **cmark**: Haskell bindings to the CommonMark reference parser (fast, robust).
- **cheapskate**: Pure Haskell Markdown processor, forgiving and XSS-safe.
- **markdown**: Customizable Haskell parser for Markdown.

## Recommended Architecture for Ordo

| Layer | Framework/Crate | Role |
|-------|----------------|------|
| Frontend | Leptos | UI, input, live preview |
| Backend | comrak (Rust) | Markdown parsing/rendering |
| Backend | rusttex (Rust) | LaTeX document generation/export |
| Backend | pandoc (Haskell) | Advanced conversion/export (optional) |
| Frontend | KaTeX/WASM | Math rendering in UI (LaTeX preview) |

## Integration Plan

### Markdown Input

```rust
// src/components/content/markdown_editor.rs
#[component]
pub fn MarkdownEditor() -> impl IntoView {
    let (markdown, set_markdown) = create_signal(String::new());
    let (html_preview, set_html_preview) = create_signal(String::new());
    
    create_effect(move |_| {
        let md = markdown.get();
        spawn_local(async move {
            // Call Tauri command to parse markdown
            match invoke("parse_markdown", to_value(&md).unwrap()).await {
                Ok(result) => {
                    let html: String = from_value(result).unwrap();
                    set_html_preview.set(html);
                },
                Err(e) => console_error!("Failed to parse markdown: {}", e),
            }
        });
    });
    
    view! {
        <div class="markdown-editor">
            <div class="editor-pane">
                <textarea 
                    on:input=move |ev| {
                        set_markdown.set(event_target_value(&ev));
                    }
                    prop:value=markdown
                />
            </div>
            <div class="preview-pane">
                <div inner_html=html_preview />
            </div>
        </div>
    }
}
```

```rust
// src-tauri/src/commands.rs
#[tauri::command]
fn parse_markdown(markdown: String) -> Result<String, String> {
    let options = ComrakOptions::default();
    Ok(markdown_to_html(&markdown, &options))
}
```

### LaTeX Input/Rendering

- For math blocks, use KaTeX (via WASM or JS) in the frontend for live rendering.
- For export, use rusttex to generate .tex files or shell out to a TeX engine for PDF.

```rust
// src/components/content/latex_editor.rs
#[component]
pub fn LaTeXEditor() -> impl IntoView {
    let (latex, set_latex) = create_signal(String::new());
    let math_ref = create_node_ref::<html::Div>();
    
    create_effect(move |_| {
        let tex = latex.get();
        if let Some(el) = math_ref.get() {
            // Use KaTeX to render the LaTeX
            window().eval(&format!("renderMathInElement({}, {{delimiters: [
                {{left: '$$', right: '$$', display: true}},
                {{left: '$', right: '$', display: false}}
            ]}});", el)).unwrap();
        }
    });
    
    view! {
        <div class="latex-editor">
            <div class="editor-pane">
                <textarea 
                    on:input=move |ev| {
                        set_latex.set(event_target_value(&ev));
                    }
                    prop:value=latex
                />
            </div>
            <div class="preview-pane" node_ref=math_ref>
                {move || latex.get()}
            </div>
            <button on:click=move |_| {
                spawn_local(async move {
                    invoke("export_latex", to_value(&latex.get()).unwrap()).await.unwrap();
                });
            }>
                "Export as PDF"
            </button>
        </div>
    }
}
```

```rust
// src-tauri/src/commands.rs
#[tauri::command]
fn export_latex(latex: String) -> Result<(), String> {
    let mut doc = ContentBuilder::new();
    doc.document_class(DocumentClass::Article, ClassOptions::A4);
    doc.title("LaTeX Export");
    doc.begin("document");
    doc.add_content(&latex);
    doc.end("document");
    
    let tex_content = doc.build_document();
    
    // Write to file and convert to PDF using local TeX engine
    std::fs::write("export.tex", tex_content).map_err(|e| e.to_string())?;
    
    // Shell out to pdflatex or similar
    let output = std::process::Command::new("pdflatex")
        .arg("export.tex")
        .output()
        .map_err(|e| e.to_string())?;
    
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    
    Ok(())
}
```

### Export/Import

- Allow users to export notes/quizzes as Markdown, HTML, or PDF (via rusttex/pandoc).
- Support import of Markdown/LaTeX files using comrak/pandoc.

```rust
// src-tauri/src/commands.rs
#[tauri::command]
fn export_document(content: String, format: String) -> Result<String, String> {
    match format.as_str() {
        "markdown" => Ok(content),
        "html" => Ok(markdown_to_html(&content, &ComrakOptions::default())),
        "pdf" => {
            // Use pandoc for advanced conversion
            let output = std::process::Command::new("pandoc")
                .arg("-f").arg("markdown")
                .arg("-t").arg("pdf")
                .arg("-o").arg("export.pdf")
                .arg("--standalone")
                .stdin(std::process::Stdio::piped())
                .output()
                .map_err(|e| e.to_string())?;
            
            if !output.status.success() {
                return Err(String::from_utf8_lossy(&output.stderr).to_string());
            }
            
            Ok("export.pdf".to_string())
        },
        "latex" => {
            // Convert markdown to LaTeX using pandoc
            let output = std::process::Command::new("pandoc")
                .arg("-f").arg("markdown")
                .arg("-t").arg("latex")
                .stdin(std::process::Stdio::piped())
                .output()
                .map_err(|e| e.to_string())?;
            
            if !output.status.success() {
                return Err(String::from_utf8_lossy(&output.stderr).to_string());
            }
            
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        },
        _ => Err(format!("Unsupported format: {}", format)),
    }
}

#[tauri::command]
fn import_document(path: String) -> Result<String, String> {
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    
    // Detect format and convert if necessary
    if path.ends_with(".tex") {
        // Convert LaTeX to Markdown using pandoc
        let output = std::process::Command::new("pandoc")
            .arg("-f").arg("latex")
            .arg("-t").arg("markdown")
            .stdin(std::process::Stdio::piped())
            .output()
            .map_err(|e| e.to_string())?;
        
        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }
        
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        // Assume it's already markdown
        Ok(content)
    }
}
```

## Offline Support

All parsing/rendering is local; no online dependency. The editor works fully offline with the following considerations:

- Markdown parsing is done using comrak, which is a pure Rust library
- LaTeX rendering in the UI uses KaTeX, which is bundled with the application
- PDF export requires a local TeX installation, which can be bundled with the application or installed separately

## Modularity

The editor is designed as a self-contained component that can be easily integrated into the Content Creation & Management module:

```rust
// src/modules/content/mod.rs
#[cfg(feature = "content-module")]
pub struct ContentModule {
    // ...other components
    advanced_editor: AdvancedEditor,
}

#[cfg(feature = "content-module")]
impl ContentModule {
    pub fn new(config: &Config) -> Self {
        // ...other initializations
        let advanced_editor = AdvancedEditor::new();
        
        Self {
            // ...other components
            advanced_editor,
        }
    }
}

// src/modules/content/advanced_editor.rs
pub struct AdvancedEditor {
    markdown_enabled: bool,
    latex_enabled: bool,
}

impl AdvancedEditor {
    pub fn new() -> Self {
        Self {
            markdown_enabled: true,
            latex_enabled: true,
        }
    }
    
    pub fn extension(&self) -> Box<dyn Extension> {
        Box::new(AdvancedEditorExtension {
            markdown_enabled: self.markdown_enabled,
            latex_enabled: self.latex_enabled,
        })
    }
}
```

## Feature Flags

```toml
# Cargo.toml
[features]
advanced-editor = ["markdown-editor", "latex-editor"]
markdown-editor = ["dep:comrak"]
latex-editor = ["dep:rusttex"]
```

## Summary Table

| Feature | Markdown | LaTeX | Crate/Framework |
|---------|----------|-------|----------------|
| Parsing | Yes | Yes (export) | comrak, rusttex |
| Rendering | Yes (HTML) | Yes (KaTeX/WASM) | comrak, KaTeX |
| Export | Yes (HTML/MD) | Yes (.tex/.pdf) | rusttex, pandoc |
| Offline | Yes | Yes | All local |
| Modular | Yes | Yes | Leptos components |

## Conclusion

The Markdown/LaTeX editor implementation for the Ordo project provides a robust, offline-first solution for creating and editing educational content. By leveraging the power of Rust crates like comrak and rusttex, along with optional Haskell tools like pandoc for advanced processing, the editor offers a seamless experience for users while maintaining the project's commitment to performance, modularity, and offline capabilities.

The implementation is designed to be fully integrated with the Ordo architecture, using Leptos for the frontend UI components and Tauri for the backend processing. This approach ensures that the editor can be easily enabled or disabled as part of the Content Creation & Management module, while still providing a rich set of features for users who need advanced content editing capabilities.
