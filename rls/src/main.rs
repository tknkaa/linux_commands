use clap::Parser;
use std::fs;
use std::fs::ReadDir;
use std::io;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(default_value = ".")]
    path: PathBuf,

    #[arg(short = 'a', long = "all")]
    all: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let path = &args.path;

    if !path.is_dir() {
        let error_message = format!("'{}' is not a valid directory.", path.display());
        let custom_error = io::Error::new(io::ErrorKind::NotFound, error_message);
        return Err(custom_error);
    }
    let entries = fs::read_dir(path)?;
    list_directory(entries, &args)?;

    Ok(())
}

fn list_directory(entries: ReadDir, args: &Args) -> io::Result<()> {
    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().to_string();
        if !args.all && file_name.starts_with('.') {
            continue;
        }
        println!("{}", file_name);
    }
    Ok(())
}
