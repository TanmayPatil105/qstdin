//! qstdin
//!
//! `qstdin` is a simple interface for querying stdin

use std::fs::File;
use std::io::stdin;
use std::os::fd::AsFd;

// possible stdin sources
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Stdin {
    Input,
    File,
    Directory,
}

/// returns true if current stdin is
/// equal to the stream passed in the argument
///
/// # Examples
/// ```
/// use qstdin::{is, Stdin};
///
/// println!("file? {}", is(Stdin::File));
/// ```
#[cfg(unix)]
pub fn is(stream: Stdin) -> bool {
    let fd = stdin().as_fd().try_clone_to_owned().unwrap();
    let file = File::from(fd);
    let meta = file.metadata().unwrap();

    if meta.is_dir() {
        if stream == Stdin::Directory {
            return true;
        } else {
            return false;
        }
    }

    if meta.is_file() {
        if stream == Stdin::File {
            return true;
        } else {
            return false;
        }
    }

    stream != Stdin::Directory && stream != Stdin::File
}
