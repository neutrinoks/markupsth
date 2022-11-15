//! # notesth
//! 
//! A very simple Rust library for writing any kind of text files (note something). It is a basis
//! for various kinds of writer-types, written in Rust. The library assists in formatting, escpially 
//! indenting and adding line-feeds when writing. If you want to generate some kind of code, e.g. HTML,
//! Rust etc. this is very useful.
//! 
//! ### Examples

use std::fmt;


/// All Writer-types have some similarities, e.g. adding a line-feed or increment and decrement
/// the current indent in the document under edit. That's why all this common functionality is
/// encapsuled in the NoteSth struct. This struct holds:
/// - the **content**-String, which holds the markup-content under edit
/// - the indent_step_size, as a number of whitespaces to be added at current line
/// - the block_stack, for closing HTML-tags automatically without specifying again which one
/// - other useful data for internal usage
/// This struct is used as a composition in the WriterTypes: HTMLWriter, XMLWriter and JSONWriter
#[derive(Debug)]
pub struct NoteSth<'t, W: fmt::Write> {
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
where W: fmt::Write {
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

    pub fn line_feed(&mut self, n: usize) -> Result<(), fmt::Error> {
        for _i in 0..n {
            self.drain.write_char('\n')?;
        }
        self.drain.write_str(&self.indent)?;
        Ok(())
    }

    pub fn line_feed_inc(&mut self) -> Result<(), fmt::Error> {
        self.inc_indent_step();
        self.line_feed(1)?;
        Ok(())
    }

    pub fn line_feed_dec(&mut self) -> Result<(), fmt::Error> {
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

    pub fn open_element(&mut self, opening: &str, closing: &str) -> Result<(), fmt::Error> {
        self.drain.write_str(opening)?;
        self.block_stack.push(String::from(closing));
        Ok(())
    }

    pub fn close_element(&mut self) -> Result<(), fmt::Error> {
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


impl<W> fmt::Write for NoteSth<'_, W>
where W: fmt::Write {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.drain.write_str(s)?;
        Ok(())
    }

    fn write_char(&mut self, c: char) -> Result<(), fmt::Error> {
        self.drain.write_char(c)?;
        Ok(())
    }

    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> Result<(), fmt::Error> {
        self.drain.write_fmt(args)?;
        Ok(())
    }
}


impl<W> fmt::Display for NoteSth<'_, W> 
where W: fmt::Write + fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.drain)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;

    #[test]
    fn indent_methods() {
        let mut content = String::new();
        let mut note = NoteSth::new(&mut content);
        assert_eq!(note.indent, "".to_string());

        note.set_indent_step(2);
        assert_eq!(note.indent, "        ".to_string());

        note.dec_indent_step();
        assert_eq!(note.indent, "    ".to_string());

        note.inc_indent_step();
        assert_eq!(note.indent, "        ".to_string());

        note.set_indent_step_size(3);
        note.set_indent_step(1);
        assert_eq!(note.indent, "   ");
    }

    #[test]
    fn write_trait() {
        let mut content = String::new();
        let mut note = NoteSth::new(&mut content);
        assert!(note.write_str(&"This is a test string".to_string()).is_ok());
        assert!(note.write_char('\n').is_ok());
    }

    #[test]
    fn test_element_functionality() {
        let mut content = String::new();
        let mut note = NoteSth::new(&mut content);
        assert!(note.open_element("Begin", "End").is_ok());
        assert!(note.line_feed_inc().is_ok());
        assert!(note.write_str("...of a very nice story. And, happy...").is_ok());
        assert!(note.line_feed_dec().is_ok());
        assert!(note.close_element().is_ok());
        assert_eq!(content, String::from("Begin\n    ...of a very nice story. And, happy...\nEnd"));
    }
}
