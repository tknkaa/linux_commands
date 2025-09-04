use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new(".");
    let entries = fs::read_dir(path)?;
    println!("Entries in '{}'", path.display());

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        println!("{}", file_name.to_string_lossy());
    }
    Ok(())
}
