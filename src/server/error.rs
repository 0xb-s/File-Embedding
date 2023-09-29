
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum MyError {
    CreateTableError(String),
    EmbedFileError(String),
    EmbedDirectoryError(String),
    // ... todo
}
