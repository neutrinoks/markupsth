//! # notesth
//!
//! A very simple Rust library for writing any kind of text files (note something). It is a basis
//! for various kinds of writer-types, written in Rust. The library assists in formatting, escpially
//! indenting and adding line-feeds when writing. If you want to generate some kind of code, e.g. HTML,
//! Rust etc. this is very useful.
//!
//! ### Rust-boundaries
//!
//! At the moment there is only a version available, that implements ```std::fmt::Write```. If requested
//! a version supporting ```std::io::Write``` could also be implemented. Just contact me.
//!
//! ### Examples
//!
//! To generate the following HTML output:
//! ```html
//! <div>
//!     <p>Some interesting written here</p>
//! </div>
//! ```
//! an implementation would look like:
//! ```
//! use notesth::MarkupSth;
//! use std::fmt::{Error, Write};
//!
//! fn main() -> Result<(), Error> {
//!     let mut text = String::new();
//!     let mut notes = MarkupSth::new(&mut text);
//!     
//!     notes.open_element("<div>", "</div>")?;
//!     notes.line_feed_inc()?;
//!     notes.open_element("<p>", "</p>")?;
//!     notes.write_str("Some interesting written here")?;
//!     notes.close_element()?;
//!     notes.line_feed_dec()?;
//!     notes.close_element()?;
//!     println!("{}", text);
//!     
//!     Ok(())
//! }
//! ```
//!
//! Another formatted text example:
//! ```
//! use notesth::MarkupSth;
//! use std::fmt::{Error, Write};
//!
//! fn main() -> Result<(), Error> {
//!     let mut text = String::new();
//!     let mut notes = MarkupSth::new(&mut text);
//!     
//!     notes.open_element("Begin", "End")?;
//!     notes.line_feed_inc()?;
//!     notes.write_str("...of a very nice story. And, happy...")?;
//!     notes.line_feed_dec()?;
//!     notes.close_element()?;
//!     println!("{}", text);
//!     
//!     Ok(())
//! }
//! ```

pub mod syntax;
pub mod markupsth;

pub use crate::{syntax::MarkupLanguage, markupsth::MarkupSth};


#[cfg(test)]
mod tests {
    // use super::*;
    // use std::fmt::Write;

    // #[test]
    // fn indent_methods() {
    //     let mut content = String::new();
    //     let mut note = MarkupSth::new(&mut content);
    //     assert_eq!(note.indent, "".to_string());

    //     note.set_indent_step(2);
    //     assert_eq!(note.indent, "        ".to_string());

    //     note.dec_indent_step();
    //     assert_eq!(note.indent, "    ".to_string());

    //     note.inc_indent_step();
    //     assert_eq!(note.indent, "        ".to_string());

    //     note.set_indent_step_size(3);
    //     note.set_indent_step(1);
    //     assert_eq!(note.indent, "   ");
    // }

    // #[test]
    // fn write_trait() {
    //     let mut content = String::new();
    //     let mut note = MarkupSth::new(&mut content);
    //     assert!(note.write_str(&"This is a test string".to_string()).is_ok());
    //     assert!(note.write_char('\n').is_ok());
    // }

    // #[test]
    // fn test_element_functionality() {
    //     let mut content = String::new();
    //     let mut note = MarkupSth::new(&mut content);
    //     assert!(note.open_element("Begin", "End").is_ok());
    //     assert!(note.line_feed_inc().is_ok());
    //     assert!(note.write_str("...of a very nice story. And, happy...").is_ok());
    //     assert!(note.line_feed_dec().is_ok());
    //     assert!(note.close_element().is_ok());
    //     assert_eq!(content, String::from("Begin\n    ...of a very nice story. And, happy...\nEnd"));
    // }
}
