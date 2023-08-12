use filewalker::FileWalker;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let path = "/etc";
    let walker = FileWalker::open(Path::new(path))?
        .file_only(true)
        .filter_hidden_items(true);

    for dir_entry in walker.take(50).flatten() {
        println!("{}", dir_entry.path().display());
    }
    Ok(())
}
