use std::{env, path::PathBuf};

use itertools::Itertools;
pub fn get_directory() -> Option<String> {
    //! TODO: Refactor this unsafe path code.

    let args = env::args().skip(1).tuple_windows();
    let mut directory: Option<String> = None;

    for (elem, next) in args {
        if elem == "--directory" {
            directory = Some(next);
            break;
        }
    }

    let mut path = PathBuf::new();
    path = path.join(std::env::current_dir().unwrap());
    path = path.join(
        &directory
            .clone()
            .unwrap_or(String::from("public/"))
            .trim_start_matches(r"\") // WINDOWS HATES HAVING '\' or '/' AT START!
            .trim_start_matches(r"/"),
    );

    return Some(String::from(path.to_str().unwrap()));
}
