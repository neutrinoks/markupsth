//! This module implements a Document, to which tags and content can be written.

use std::{
    fmt::{self, Write as FmtWrite},
    io::{BufWriter, Write as IoWrite},
    fs::{self, File},
};

/// Result definition.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// An implementation of a document, where text and tags can be written to.
#[derive(Debug)]
pub enum Document {
    /// In case the 'Document' is stored as String in memory.
    String(String),
    /// In case the 'Document' will be written to a file via BufWriter.
    File(BufWriter<File>),
}

impl Document {
    fn new_file(name: &str) -> Result<Document> {
        let file = fs::File::open(name)?;
        Ok(Document::File(BufWriter::new(file)))
    }

    fn new_buffer() -> Result<Document> {
        Ok(Document::String(String::new()))
    }

    fn write_str(&mut self, s: &str) -> Result<()> {
        match self {
            Document::String(snk) => snk.push_str(s),
            Document::File(writer) => writer.write_all(s.as_ref())?,
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> Result<()> {
        match self {
            Document::String(buf) => buf.push(c),
            Document::File(writer) => { writer.write(&vec![c as u8])?; },
        }
        Ok(())
    }

    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> Result<()> {
        match self {
            Document::String(buf) => buf.write_fmt(args)?,
            Document::File(writer) => writer.write_fmt(args)?,
        }
        Ok(())
    }
}
