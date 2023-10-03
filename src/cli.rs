use std::{env, path::PathBuf};

use itertools::Itertools;
pub fn get_directory() -> PathBuf {
    //! TODO: Refactor this unsafe path code.

    let args = env::args().skip(1).tuple_windows();
    let mut directory: String = String::new();

    for (elem, next) in args {
        if elem == "--directory" {
            directory = next;
            break;
        }
    }

    PathBuf::from(directory)
}
