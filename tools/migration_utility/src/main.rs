use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(Debug, StructOpt)]
#[structopt(name = "migration-utility", about = "Utility for managing JavaScript to Rust migration")]
enum Command {
    Discover {
        #[structopt(short, long, default_value = ".")]
        path: String,
        #[structopt(short, long, default_value = "node_modules,coverage,dist,build")]
        exclude: String,
    },
    Migrate {
        #[structopt(short, long)]
        file: String,
        #[structopt(short, long)]
        target: String,
    },
    Clean {
        #[structopt(short, long, default_value = "JavaScript to Rust Migration Tracking.md")]
        tracking_doc: String,
        #[structopt(short, long)]
        dry_run: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Command::from_args();

    match opt {
        Command::Discover { path, exclude } => {
            println!("Discovering files in {} excluding {}", path, exclude);
            for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                if !exclude.split(',').any(|ex| entry.path().to_string_lossy().contains(ex)) {
                    println!("{}", entry.path().display());
                }
            }
        }
        Command::Migrate { file, target } => {
            println!("Migrating {} to {}", file, target);
        }
        Command::Clean { tracking_doc, dry_run } => {
            if dry_run {
                println!("Dry run: Cleaning up tracking document {}", tracking_doc);
            } else {
                println!("Cleaning up tracking document {}", tracking_doc);
            }
        }
    }

    Ok(())
}