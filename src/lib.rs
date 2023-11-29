//! # notesth
//!
//! A very simple Rust library for writing almost all kinds of formatted text files, escpially 
//! Markup files, such as HTML and XML. Its original purpose was to develop a Text-to-HTML 
//! converter, but got slowly extended step-by-step. HTML and XML support is default, but you can
//! also define your own Markup language syntax. There are also some pre-defined formatters to
//! either non-format the Markup output or have some out-of-the-box formatting styles for a
//! beautiful and readable HTML code.
//!
//! For an individual syntax style (own Markup Language), have a look at the `syntax` module. For
//! an own formatting style (write your own formatter to be used with this crate), have a look at
//! the `format` module.
//!
//! In case of interest, I am also willing to extend this crate any time by new Markup Languages or
//! formatting styles. Feel free to contact me via Email.
//!
//! ## Examples from Scratch
//!
//! By using the an implemented Markup Language such as HTML or XML, and a pre-defined `Formatter`,
//! you can quickly write some converter or HTML-generator. Such a quick-start guide can be seen in
//! the following example.
//! 
//! ### Simple Readable HTML
//!
//! To generate the following HTML code:
//! ```html
//! <!DOCTYPE html>
//! <html>
//! <head>
//!     <title>New Website</title>
//!     <link href="css/style.css" rel="stylesheet">
//! </head>
//! <body>
//!     <section>
//!         <img src="image.jpg">
//!         <p>This is HTML</p>
//!     </section>
//! </body>
//! </html>
//! ```
//! you need to implement:
//! ```
//! use markupsth::{AutoIndent, MarkupLanguage, MarkupSth, properties};
//!
//! // Setup a document (String), MarkupSth and a default formatter.
//! let mut document = String::new();
//! let mut markup = MarkupSth::new(&mut document, MarkupLanguage::Html).unwrap();
//! 
//! // Generate the content of example shown above.
//! markup.open("html").unwrap();
//! markup.open("head").unwrap();
//! markup.open("title").unwrap();
//! markup.text("New Website").unwrap();
//! markup.close().unwrap();
//! markup.new_line().unwrap();
//! markup.self_closing("link").unwrap();
//! properties!(markup, "href", "css/style.css", "rel", "stylesheet").unwrap();
//! markup.close();
//! markup.open("body").unwrap();
//! markup.open("section").unwrap();
//! markup.self_closing("img").unwrap();
//! properties!(markup, "src", "image.jpg").unwrap();
//! markup.open("p").unwrap();
//! markup.text("This is HTML").unwrap();
//! markup.close_all().unwrap();
//! markup.finalize().unwrap();
//! ```

//! optional configuration for formatting and syntax description.
//!
//! It is a basis
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
    fn simple_unformatted_html() {
        let mut document = String::new();
        let mut markup = MarkupSth::new(&mut document, MarkupLanguage::Html).unwrap();
        markup.set_formatter(Box::new(NoFormatting::new()));
        markup.open("html").unwrap();
        markup.text("This is HTML").unwrap();
        markup.close().unwrap();
        markup.finalize().unwrap();
        assert_eq!(document, "<!DOCTYPE html><html>This is HTML</html>");
    }

    #[test]
    fn unformatted_html_with_properties() {
        let mut document = String::new();
        let mut markup = MarkupSth::new(&mut document, MarkupLanguage::Html).unwrap();
        markup.set_formatter(Box::new(NoFormatting::new()));
        markup.open("body").unwrap();
        markup.open("section").unwrap();
        markup.properties(&[("class", "class")]).unwrap();
        markup.open("div").unwrap();
        markup
            .properties(&[("keya", "value1"), ("keyb", "value2")])
            .unwrap();
        markup.text("Text").unwrap();
        markup.self_closing("img").unwrap();
        properties!(markup, "src", "img.jpg").unwrap();
        markup.close_all().unwrap();
        markup.finalize().unwrap();
        assert_eq!(
            document,
            concat![
                r#"<!DOCTYPE html><body><section class="class">"#,
                r#"<div keya="value1" keyb="value2">"#,
                r#"Text<img src="img.jpg"></div></section></body>"#
            ]
        );
    }

    #[test]
    fn formatted_html_always_indent() {
        let mut document = String::new();
        let mut markup = MarkupSth::new(&mut document, MarkupLanguage::Html).unwrap();
        markup.set_formatter(Box::new(AlwaysIndentAlwaysLf::new()));
        markup.open("head").unwrap();
        markup.self_closing("meta").unwrap();
        properties!(markup, "charset", "utf-8").unwrap();
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
        markup.open("head").unwrap();
        markup.self_closing("meta").unwrap();
        properties!(markup, "charset", "utf-8").unwrap();
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
