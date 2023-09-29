
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum MyError {
    CreateTableError(String),
    EmbedFileError(String),
    EmbedDirectoryError(String),
    // ... todo
}
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::CreateTableError(msg) => write!(f, "Create Table Error: {}", msg),
            MyError::EmbedFileError(msg) => write!(f, "Embed File Error: {}", msg),
            MyError::EmbedDirectoryError(msg) => write!(f, "Embed Directory Error: {}", msg),
        }
    }
}

impl Error for MyError {}
