use std::path::Path;
use filewalker::FileWalker;

fn main() -> std::io::Result<()> {
    let path = "/etc";
    let walker = FileWalker::open(Path::new(path))?;

    for file in walker.take(50) {
        if let Ok(file_entry) = file {
            println!("{}", file_entry.path().display());
        }
    }
    Ok(())
}