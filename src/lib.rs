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
//!
//! Another formatted text example:

pub mod format;
pub mod markupsth;
pub mod syntax;

pub use crate::{
    format::generic::*, format::Formatter, markupsth::MarkupSth, syntax::MarkupLanguage,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn testfile(name: &str) -> String {
        let mut s = std::fs::read_to_string(name).unwrap();
        s.pop();
        s
    }

    #[test]
    fn formatted_html_always_indent() {
        let mut document = String::new();
        let mut markup = MarkupSth::new(&mut document, MarkupLanguage::Html).unwrap();
        markup.set_formatter(Box::new(AlwaysIndentAlwaysLf::new()));
        markup.open("head").unwrap();
        markup.self_closing("meta").unwrap();
        markup.properties(&properties!["charset", "utf-8"]).unwrap();
        markup.close().unwrap();
        markup.open("body").unwrap();
        markup.open("section").unwrap();
        markup.open("div").unwrap();
        markup.open("p").unwrap();
        markup.text("Text").unwrap();
        markup.close_all().unwrap();
        markup.finalize().unwrap();
        assert_eq!(
            document,
            testfile("tests/formatted_html_always_indent.html"),
        );
    }

    #[test]
    fn formatted_html_auto_indent() {
        let mut document = String::new();
        let mut markup = MarkupSth::new(&mut document, MarkupLanguage::Html).unwrap();
        let mut formatter = AutoIndent::new();
        formatter.set_always_filter(always_filter!["head", "body", "section"]);
        markup.set_formatter(Box::new(formatter));
        markup.open("head").unwrap();
        markup.self_closing("meta").unwrap();
        markup.properties(&properties!["charset", "utf-8"]).unwrap();
        markup.close().unwrap();
        markup.open("body").unwrap();
        markup.open("section").unwrap();
        markup.open("div").unwrap();
        markup.new_line().unwrap();
        markup.open("div").unwrap();
        markup.open("p").unwrap();
        markup.text("Text").unwrap();
        markup.close_all().unwrap();
        markup.finalize().unwrap();
        assert_eq!(document, testfile("tests/formatted_html_auto_indent.html"),);
    }
}
