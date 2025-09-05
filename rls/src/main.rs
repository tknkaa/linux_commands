use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let path_str = if args.len() > 1 { &args[1] } else { "." };
    let path = Path::new(path_str);

    if !path.is_dir() {
        let error_message = format!("'{}' is not a valid directory.", path.display());
        let custom_error = io::Error::new(io::ErrorKind::NotFound, error_message);
        return Err(custom_error);
    }
    let entries = fs::read_dir(path)?;
    println!("Entries in '{}'", path.display());

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        println!("{}", file_name.to_string_lossy());
    }
    Ok(())
}
