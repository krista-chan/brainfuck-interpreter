pub struct Errors;
use std::path::PathBuf;

use colored::Colorize;

impl Errors {
    pub fn invalid_token(token: &str, extra: &str, pos: (u32, u32)) {
        println!("Found invalid token \"{}\" at position {}:{}. {}", token.red().bold(), pos.1.to_string().yellow(), pos.0.to_string().yellow(), extra);
    }

    pub fn file_not_exists(path: PathBuf) {
        println!("Path \"{}\" is invalid or does not exist.", path.display().to_string().red().bold())
    }
}
