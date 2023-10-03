use std::{env, path::PathBuf};

use itertools::Itertools;
pub fn get_directory() -> Option<String> {
    let args = env::args().skip(1).tuple_windows();
    // .position(|(elem, next)| elem == "--directory" && Path::new(&next).is_dir());
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
            .expect("--directory argument is missing")
            .trim_start_matches(r"\") // WINDOWS HATES HAVING '\' or '/' AT START!
            .trim_start_matches(r"/"),
    );

    println!("{:?}", path);

    if path.is_dir() {
        return Some(String::from(path.to_str().unwrap()));
    }

    return None;
}
