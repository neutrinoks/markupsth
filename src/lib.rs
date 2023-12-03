//! # notesth
//!
//! A very simple Rust library for writing almost all kinds of formatted text files, escpially
//! Markup files, such as HTML and XML. Its original purpose was to develop a Text-to-HTML
//! converter, but got slowly extended step-by-step. HTML and XML support is default and
//! implemented, but you can also define your own Markup Language by configuring your individual
//! syntax. There are also some pre-defined formatters to either no formatting at all the Markup
//! output or have some out-of-the-box formatting styles for a beautiful and readable HTML code.
//!
//! For an individual syntax style (own Markup Language), have a look at the `syntax` module. For
//! an own formatting style (implement your own `Formatter` to be used with this crate), have a
//! look at the `format` module.
//!
//! In case of interest, I am also willing to extend this crate any time by new Markup Languages,
//! formatting styles, or any other kind of meaningful modifications. Feel free to contact me via
//! Email provided in the manifest file.
//!
//! ## Examples from Scratch
//!
//! By using an implemented Markup Language such as HTML or XML, and a pre-defined `Formatter`, you
//! can quickly write some converter or HTML-generator. Such a quick-start guide can be seen in the
//! following example.
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
//!         <div>
//!             <div><img src="image.jpg"></div>
//!             <p>This is HTML</p>
//!         </div>
//!     </section>
//! </body>
//! </html>
//! ```
//! you need to implement:
//! ```
//! use markupsth::{AutoIndent, Language, MarkupSth, properties};
//!
//! // Setup a document (String), MarkupSth and a default formatter.
//! let mut document = String::new();
//! let mut mus = MarkupSth::new(&mut document, Language::Html).unwrap();
//!
//! // Generate the content of example shown above.
//! mus.open("html").unwrap();
//! mus.open("head").unwrap();
//! mus.open_close_w("title", "New Website").unwrap();
//! mus.self_closing("link").unwrap();
//! properties!(mus, "href", "css/style.css", "rel", "stylesheet").unwrap();
//! mus.close().unwrap();
//! mus.open("body").unwrap();
//! mus.open("section").unwrap();
//! mus.open("div").unwrap();
//! mus.new_line().unwrap();
//! mus.open("div").unwrap();
//! mus.self_closing("img").unwrap();
//! properties!(mus, "src", "image.jpg").unwrap();
//! mus.close().unwrap();
//! mus.open_close_w("p", "This is HTML").unwrap();
//! mus.close_all().unwrap();
//! mus.finalize().unwrap();
//! # assert_eq!(document, markupsth::testfile("formatted_html_auto_indent.html"));
//! ```
//!
//! ### Simple XML Example
//!
//! To generate the following output:
//! ```xml
//! <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
//! <directory>
//!     <title>Wikipedia List of Cities</title>
//!     <entry>
//!         <keyword>Hamburg</keyword>
//!         <entrystext>Hamburg is the residence of ...</entrystext>
//!     </entry>
//!     <entry>
//!         <keyword>Munich</keyword>
//!         <entrystext>Munich is the residence of ...</entrystext>
//!     </entry>
//! </directory>
//! ```
//! you have to implement:
//! ```
//! use markupsth::{AutoIndent, Language, MarkupSth, properties};
//!
//! // Setup a document (String), MarkupSth and a default formatter.
//! let mut document = String::new();
//! let mut mus = MarkupSth::new(&mut document, Language::Html).unwrap();
//!
//! // Generate the content of example shown above.
//! todo!();
//! ```

pub mod format;
pub mod markupsth;
pub mod syntax;

pub use crate::{format::generic::*, format::Formatter, markupsth::MarkupSth, syntax::Language};

/// Crate internal support method for some unittests with external reference files.
pub fn testfile(name: &str) -> String {
    let mut s = std::fs::read_to_string(&format!("tests/{}", name)).unwrap();
    s.pop();
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_unformatted_html() {
        let mut document = String::new();
        let mut mus = MarkupSth::new(&mut document, Language::Html).unwrap();
        mus.set_formatter(Box::new(NoFormatting::new()));
        mus.open("html").unwrap();
        mus.text("This is HTML").unwrap();
        mus.close().unwrap();
        mus.finalize().unwrap();
        assert_eq!(document, "<!DOCTYPE html><html>This is HTML</html>");
    }

    #[test]
    fn unformatted_html_with_properties() {
        let mut document = String::new();
        let mut mus = MarkupSth::new(&mut document, Language::Html).unwrap();
        mus.set_formatter(Box::new(NoFormatting::new()));
        mus.open("body").unwrap();
        mus.open("section").unwrap();
        mus.properties(&[("class", "class")]).unwrap();
        mus.open("div").unwrap();
        mus.properties(&[("keya", "value1"), ("keyb", "value2")])
            .unwrap();
        mus.text("Text").unwrap();
        mus.self_closing("img").unwrap();
        properties!(mus, "src", "img.jpg").unwrap();
        mus.close_all().unwrap();
        mus.finalize().unwrap();
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
        let mut mus = MarkupSth::new(&mut document, Language::Html).unwrap();
        mus.set_formatter(Box::new(AlwaysIndentAlwaysLf::new()));
        mus.open("head").unwrap();
        mus.self_closing("meta").unwrap();
        properties!(mus, "charset", "utf-8").unwrap();
        mus.close().unwrap();
        mus.open("body").unwrap();
        mus.open("section").unwrap();
        mus.open("div").unwrap();
        mus.open("p").unwrap();
        mus.text("Text").unwrap();
        mus.close_all().unwrap();
        mus.finalize().unwrap();
        assert_eq!(document, testfile("formatted_html_always_indent.html"),);
    }

    #[test]
    fn formatted_html_auto_indent() {
        let mut document = String::new();
        let mut mus = MarkupSth::new(&mut document, Language::Html).unwrap();
        mus.open("html").unwrap();
        mus.open("head").unwrap();
        mus.open_close_w("title", "New Website").unwrap();
        mus.self_closing("link").unwrap();
        properties!(mus, "href", "css/style.css", "rel", "stylesheet").unwrap();
        mus.close().unwrap();
        mus.open("body").unwrap();
        mus.open("section").unwrap();
        mus.open("div").unwrap();
        mus.new_line().unwrap();
        mus.open("div").unwrap();
        mus.self_closing("img").unwrap();
        properties!(mus, "src", "image.jpg").unwrap();
        mus.close().unwrap();
        mus.open_close_w("p", "This is HTML").unwrap();
        mus.close_all().unwrap();
        mus.finalize().unwrap();
        assert_eq!(document, testfile("formatted_html_auto_indent.html"),);
    }

    #[test]
    fn formatted_xml_auto_indent() {
        let do_entry = |mus: &mut MarkupSth, name: &str| {
            mus.open("entry").unwrap();
            mus.open("keyword").unwrap();
            mus.text(name).unwrap();
            mus.close().unwrap();
            mus.open("entrystext").unwrap();
            mus.text(&format!("{} is the residence of ...", name))
                .unwrap();
            mus.close().unwrap();
            mus.close().unwrap();
        };

        let mut document = String::new();
        let mut mus = MarkupSth::new(&mut document, Language::Xml).unwrap();
        todo!();
        mus.formatter
            .set_filter_indent_always(&["directory", "entry"])
            .unwrap();
        mus.formatter
            .set_filter_lf_closing(&["title", "keyword", "entrystext"])
            .unwrap();

        mus.open("directory").unwrap();
        mus.open("title").unwrap();
        mus.text("Wikipedia List of Cities").unwrap();
        mus.close().unwrap();
        do_entry(&mut mus, "Hamburg");
        do_entry(&mut mus, "Munich");
        mus.close_all().unwrap();
        mus.finalize().unwrap();

        assert_eq!(document, testfile("formatted_xml_auto_indent.xml"));
    }
}
