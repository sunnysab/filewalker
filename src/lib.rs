use std::fs::{DirEntry, ReadDir};
use std::path::Path;
use std::io;
use std::os::unix::ffi::OsStrExt;


/// Iterator over the entries in a directory and its sub-directories.
pub struct FileWalker {
    /// `stack` stores file scanning iterator for each layer, and `stack.last()` always refers to
    /// current directory iterator.
    stack: Vec<ReadDir>,

    /// Hidden file flag
    filter_hidden_files: bool,
}

impl FileWalker {
    pub fn open(root: &Path) -> io::Result<Self> {
        let iter = root.read_dir()?;

        let stack = vec![iter];
        Ok(FileWalker { stack, filter_hidden_files: false })
    }

    /// Filter out hidden files and directories.
    /// Note: this function may cause performance loss.
    pub fn filter_hidden_items(mut self, flag: bool) -> Self {
        self.filter_hidden_files = flag;
        self
    }

    fn next_result(&mut self) -> io::Result<Option<DirEntry>> {
        // Jump to current directory

        'iter_dir: while let Some(lowest_dir_iter) = self.stack.last_mut() {
            // If current layer's iterator is available, read next file
            for next_entry in lowest_dir_iter {

                    // If current entrance is not available, consider that there are still some files to
                    // iterate and `lowest_dir_iter` has been moved, return error and wait for a next call.
                    let entry = next_entry?;

                    if self.filter_hidden_files {
                        let file_name = &entry.file_name();
                        if let [0x2E, _rest @ ..] = file_name.as_bytes() {
                            continue;
                        }
                    }

                    if entry.file_type()?.is_dir() {
                        // Once we found a new directory, push its iterator back to the stack
                        // If error occures, maybe for permission denied the sub-directory can not be accessed,
                        // just return that error and ignore, wait for a next call
                        let new_subdirectory = entry.path().read_dir()?;
                        self.stack.push(new_subdirectory);
                        break 'iter_dir;
                    }
            }
            // Return to up level and try again
            self.stack.pop();
        }
        Ok(None)
    }
}

impl Iterator for FileWalker {
    type Item = io::Result<DirEntry>;

    /// Get next file in the directory and sub-directories.
    /// Any error will make a None returned.
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_result() {
            Ok(Some(r)) => Some(Ok(r)),
            Err(e) => Some(Err(e)),
            Ok(None) => None,
        }
    }
}