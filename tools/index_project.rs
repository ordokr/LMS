use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let project_dir = PathBuf::from(".");
    let output_file = File::create("project_index.md")?;
    
    println!("Creating project index...");
    generate_index(&project_dir, output_file)?;
    println!("Project index created at: project_index.md");
    
    Ok(())
}

fn generate_index<W: Write>(dir: &Path, mut writer: W) -> io::Result<()> {
    writeln!(writer, "# LMS Project Index\n")?;
    writeln!(writer, "Generated on: {}\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"))?;
    
    // Index backend files
    writeln!(writer, "## Backend (Rust/Tauri)\n")?;
    index_directory(&dir.join("src-tauri"), &mut writer, 0, "")?;
    
    // Index frontend files
    writeln!(writer, "\n## Frontend\n")?;
    index_directory(&dir.join("src"), &mut writer, 0, "")?;
    
    // Create a status report
    writeln!(writer, "\n## Component Status\n")?;
    
    let components_to_check = [
        ("Auth API", "src-tauri/src/api/auth.rs"),
        ("User Repository", "src-tauri/src/database/repositories/user.rs"),
        ("Auth Service", "src-tauri/src/core/auth.rs"),
        ("Forum Repository", "src-tauri/src/database/repositories/forum.rs"),
        ("Course Repository", "src-tauri/src/database/repositories/course.rs"),
        ("Main Entry", "src-tauri/src/main.rs"),
        ("Login Component", "src/components/auth/login.rs"),
        ("Register Component", "src/components/auth/register.rs"),
        ("Forum Components", "src/components/forum"),
        ("Course Components", "src/components/courses"),
    ];
    
    writeln!(writer, "| Component | Status | Path |")?;
    writeln!(writer, "|-----------|--------|------|")?;
    
    for (name, path) in components_to_check.iter() {
        let status = if Path::new(path).exists() { "✅ Complete" } else { "❌ Missing" };
        writeln!(writer, "| {} | {} | `{}` |", name, status, path)?;
    }
    
    Ok(())
}

fn index_directory<W: Write>(dir: &Path, writer: &mut W, depth: usize, prefix: &str) -> io::Result<()> {
    if !dir.exists() || !dir.is_dir() {
        return Ok(());
    }
    
    let entries = fs::read_dir(dir)?
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    
    for (i, entry) in entries.iter().enumerate() {
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy();
        
        // Skip hidden files and directories
        if name.starts_with('.') {
            continue;
        }
        
        // Skip target directories and node_modules
        if name == "target" || name == "node_modules" {
            continue;
        }
        
        let is_last = i == entries.len() - 1;
        let indent = if depth == 0 { "" } else { prefix };
        let new_prefix = if depth == 0 {
            "  ".to_string()
        } else {
            format!("{}  ", prefix)
        };
        
        if path.is_dir() {
            writeln!(writer, "{}{} **{}/**", indent, if is_last { "└──" } else { "├──" }, name)?;
            index_directory(&path, writer, depth + 1, &new_prefix)?;
        } else {
            writeln!(writer, "{}{} {}", indent, if is_last { "└──" } else { "├──" }, name)?;
        }
    }
    
    Ok(())
}