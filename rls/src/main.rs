use clap::Parser;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(default_value = ".")]
    path: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let path = args.path;

    if !path.is_dir() {
        let error_message = format!("'{}' is not a valid directory.", path.display());
        let custom_error = io::Error::new(io::ErrorKind::NotFound, error_message);
        return Err(custom_error);
    }
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        println!("{}", file_name.to_string_lossy());
    }
    Ok(())
}
