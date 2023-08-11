use std::fs::{self, DirEntry, ReadDir};
use std::path::{Path, PathBuf};
use std::io;


/// Iterator over the entries in a directory and its sub-directories.
pub struct FileWalker {
    /// `stack` stores file scanning iterator for each layer, and `stack.last()` always refers to
    /// current directory iterator.
    stack: Vec<ReadDir>,
}

impl FileWalker {
    pub fn open(root: &Path) -> io::Result<Self> {
        let iter = root.read_dir()?;

        let stack = vec![iter];
        Ok(FileWalker { stack })
    }
}

impl Iterator for FileWalker {
    type Item = io::Result<DirEntry>;

    /// Get next file in the directory and sub-directories.
    /// Any error will make a None returned.
    fn next(&mut self) -> Option<Self::Item> {
        // Jump to current directory
        while let Some(lowest_dir_iter) = self.stack.last_mut() {
            // If current layer's iterator is available, read next file
            if let Some(next_entry) = lowest_dir_iter.next() {
                // If current entrance is not available, consider that there are still some files to
                // iterate and `lowest_dir_iter` has been moved, return error and wait for a next call.
                let entry = match next_entry {
                    Ok(entry) => { entry },
                    Err(e) => return Some(Err(e)),
                };

                match entry.file_type() {
                    Ok(entry_type) => {
                        if entry_type.is_dir() {
                            // Once we found a new directory, push its iterator back to the stack
                            match fs::read_dir(&entry.path()) {
                                Ok(new_subdirectory) => {
                                    self.stack.push(new_subdirectory);
                                }
                                Err(e) => {
                                    // Maybe for permission denied the sub-directory can not be accessed,
                                    // just return that error and ignore, wait for a next call
                                    return Some(Err(e));
                                }
                            }
                        } else  {
                            // Some pipes, symbol links may be returned.
                            return Some(Ok(entry));
                        }
                    } ,
                    Err(e) => return  Some(Err(e)),
                }
            } else {
                // Return to up level and try again
                self.stack.pop();
            }
        }
        None
    }
}