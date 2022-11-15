//! # notesth
//! 
//! A very simple Rust library for writing any kind of text files (note something). It is a basis
//! for various kinds of writer-types, written in Rust. The library assists in formatting, escpially 
//! indenting and adding line-feeds when writing. If you want to generate some kind of code, e.g. HTML,
//! Rust etc. this is very useful.
//! 
//! ### Examples

use std::fmt::{Write, Error};


/// All Writer-types have some similarities, e.g. adding a line-feed or increment and decrement
/// the current indent in the document under edit. That's why all this common functionality is
/// encapsuled in the NoteSth struct. This struct holds:
/// - the **content**-String, which holds the markup-content under edit
/// - the indent_step_size, as a number of whitespaces to be added at current line
/// - the block_stack, for closing HTML-tags automatically without specifying again which one
/// - other useful data for internal usage
/// This struct is used as a composition in the WriterTypes: HTMLWriter, XMLWriter and JSONWriter
#[derive(Debug)]
pub struct NoteSth<'t, W: Write> {
    /// Drain, thing where text will be written
    drain: &'t mut W,
    /// number of whitespaces one indent-step means
    indent_step_size: usize,
    /// holds the current indent as a string for quick adding into content
    indent: String,
    /// holds a stack with opened/unclosed block-tags
    block_stack: Vec<String>
}

impl<'t, W> NoteSth<'t, W> 
where W: Write {
    /// New-pattern, creates a new instance of NoteSth, default step-size: 4
    pub fn new(drain: &'t mut W) -> NoteSth<W> {
        NoteSth {
            drain,
            indent_step_size: 4,
            indent: String::new(),
            block_stack: Vec::new(),
        }
    }

    // Not sure, if this would be good. Possible misunderstanding of its meaning
    // pub fn clear(&mut self) {
    //     self.indent_step_size = 4;
    //     self.indent.clear();
    //     self.block_stack.clear();
    // }

    pub fn line_feed(&mut self, n: usize) -> Result<(), Error> {
        for _i in 0..n {
            self.drain.write_char('\n')?;
        }
        self.drain.write_str(&self.indent)?;
        Ok(())
    }

    pub fn line_feed_inc(&mut self) -> Result<(), Error> {
        self.inc_indent_step();
        self.line_feed(1)?;
        Ok(())
    }

    pub fn line_feed_dec(&mut self) -> Result<(), Error> {
        self.dec_indent_step();
        self.line_feed(1)?;
        Ok(())
    }

    pub fn inc_indent_step(&mut self) {
        self.indent.push_str(" ".repeat(self.indent_step_size).as_str());
    }

    pub fn dec_indent_step(&mut self) {
        let len = self.indent.len();
        if self.indent_step_size > len {
            self.indent = String::new();
            // this could also be a panic-case, but can also be a weird user-taste
        } else {
            self.indent.truncate(len - self.indent_step_size);
        }
    }

    pub fn open_element(&mut self, opening: &str, closing: &str) -> Result<(), Error> {
        self.drain.write_str(opening)?;
        self.block_stack.push(String::from(closing));
        Ok(())
    }

    pub fn close_element(&mut self) -> Result<(), Error> {
        if let Some(closing) = self.block_stack.pop() {
            self.drain.write_str(&closing)?;
            Ok(())
        } else {
            panic!("block stack of NoteSth is empty, check your code");
        }
    }

    pub fn set_indent_step(&mut self, indent_step: usize) {
        self.indent = " ".repeat(indent_step * self.indent_step_size);
    }

    pub fn set_indent_step_size(&mut self, indent_step_size: usize) {
        self.indent_step_size = indent_step_size;
    }
}


impl<W> Write for NoteSth<'_, W>
where W: Write {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        self.drain.write_str(s)?;
        Ok(())
    }

    fn write_char(&mut self, c: char) -> Result<(), Error> {
        self.drain.write_char(c)?;
        Ok(())
    }

    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> Result<(), Error> {
        self.drain.write_fmt(args)?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
