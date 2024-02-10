//! This module is home of `MarkupSth`, the core and 'writer' of this crate. `MarkupSth` owns the
//! syntax configuration and a `Formatter`, which can be configured individually.

use crate::{
    format::{FormatChanges, Formatter, Sequence, SequenceState, TagSequence},
    syntax::{Language, SyntaxConfig},
};
use std::fmt::Write;

/// Internal `Result` definition to make it more easy to write our default return type.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// The core and 'writer' of this crate. Configure and use one instance of `MarkupSth` to generate
/// your Markup-Language content. Configurable sub-items are about syntax of used Markup Language
/// and about formatting. This crate provides some pre-defined configurations, which can be used
/// out of the box. Available and pre-defined syntaxes are HTML and XML, but you can also configure
/// your own syntax of an individual Markup Language. For formatting there are currently three
/// pre-defined formatters available (`NoFormatting` or `AutoIndent` may suit your needs). For more
/// informations have a look at the documentation of the two modules `format` and `syntax`.
///
/// # Examples
///
/// To generate the following very simple HTML output:
/// ```html
/// <!DOCTYPE html>
/// <html>
/// <body>
///     <section>
///         <img src="image.jpg">
///         <p>This is HTML</p>
///     </section>
/// </body>
/// </html>
/// ```
/// you can use the following Rust code snippet:
/// ```
/// use markupsth::{MarkupSth, Language, AutoIndent, properties};
///
/// let mut document = String::new();
/// let mut markup = MarkupSth::new(&mut document, Language::Html).unwrap();
/// markup.open("html").unwrap();
/// markup.open("body").unwrap();
/// markup.open("section").unwrap();
/// markup.self_closing("img").unwrap();
/// properties!(markup, "src", "image.jpg").unwrap();
/// markup.open("p").unwrap();
/// markup.text("This is HTML").unwrap();
/// markup.close_all().unwrap();
/// markup.finalize().unwrap();
/// ```
#[derive(Debug)]
pub struct MarkupSth<'d> {
    /// Syntax configuration of `MarkupSth`.
    pub syntax: SyntaxConfig,
    /// Formatting configuration of `MarkupSth`.
    pub formatter: Box<dyn Formatter>,
    /// Sequence state stored interally.
    seq_state: SequenceState,
    /// Simple optimization.
    indent_str: String,
    /// Reference to a Document.
    document: &'d mut String,
}

/// Do not repeat yourself!
macro_rules! final_op_arm {
    (selfclosing $self:expr) => {{
        $self.document.write_fmt(format_args!(
            "{}",
            $self.syntax.self_closing.as_ref().unwrap().after
        ))?;
    }};
    (opening $self:expr) => {{
        $self.document.write_fmt(format_args!(
            "{}",
            $self.syntax.tag_pairs.as_ref().unwrap().opening_after
        ))?;
    }};
    (closing $self:expr) => {{
        $self.document.write_fmt(format_args!(
            "{}",
            $self.syntax.tag_pairs.as_ref().unwrap().closing_after
        ))?;
    }};
}

pub(crate) use final_op_arm;

impl<'d> MarkupSth<'d> {
    /// New type pattern for creating a new MarkupSth.
    pub fn new(document: &'d mut String, ml: Language) -> Result<MarkupSth<'d>> {
        Ok(MarkupSth {
            syntax: SyntaxConfig::from(ml),
            formatter: Box::new(crate::formatters::AutoIndent::new()),
            seq_state: SequenceState::new(),
            indent_str: String::new(),
            document,
        })
    }

    /// Set a new `Formatter`.
    pub fn set_formatter(&mut self, formatter: Box<dyn Formatter>) {
        self.formatter = formatter;
    }

    /// Inserts a single tag.
    pub fn self_closing(&mut self, tag: &str) -> Result<()> {
        self.finalize_last_op(TagSequence::self_closing(tag))?;
        if let Some(cfg) = &self.syntax.self_closing {
            self.document
                .write_fmt(format_args!("{}{}", cfg.before, tag))?;
            Ok(())
        } else {
            Err("MarkupSth: in this syntaxuration are no self-closing tag elements allowed".into())
        }
    }

    pub fn open(&mut self, tag: &str) -> Result<()> {
        self.finalize_last_op(TagSequence::opening(tag))?;
        if let Some(cfg) = &self.syntax.tag_pairs {
            self.document
                .write_fmt(format_args!("{}{}", cfg.opening_before, tag))?;
            self.seq_state.tag_stack.push(tag.to_string());
            Ok(())
        } else {
            Err("MarkupSth: in this syntaxuration are no tag-pair element allowed".into())
        }
    }

    pub fn close(&mut self) -> Result<()> {
        if self.syntax.tag_pairs.is_none() {
            return Err("MarkupSth: in this syntaxuration are no tag-pair element allowed".into());
        }
        if self.seq_state.tag_stack.is_empty() {
            return Err("MarkupSth: tag-pair tag_stack error".into());
        }

        let tag = self.seq_state.tag_stack.pop().unwrap();
        self.finalize_last_op(TagSequence::closing(&tag))?;
        let cfg = self.syntax.tag_pairs.as_ref().unwrap();
        self.document
            .write_fmt(format_args!("{}{}", cfg.closing_before, &tag))?;
        Ok(())
    }

    /// TODO
    pub fn open_close_w(&mut self, tag: &str, content: &str) -> Result<()> {
        self.open(tag)?;
        self.text(content)?;
        self.close()?;
        Ok(())
    }

    /// Inserts a single tag with properties.
    pub fn properties(&mut self, properties: &[(&str, &str)]) -> Result<()> {
        if !matches!(
            self.seq_state.last.0,
            Sequence::SelfClosing | Sequence::Opening
        ) {
            return Err(
                "MarkupSth: properties can only be added to self-closing or opening tags".into(),
            );
        }

        if let Some(cfg) = &self.syntax.properties {
            self.document.write_fmt(format_args!("{}", cfg.initiator))?;
            let len = properties.len();
            for property in properties[..len - 1].iter() {
                self.document.write_fmt(format_args!(
                    "{}{}{}{}{}{}{}{}",
                    cfg.name_before,
                    property.0,
                    cfg.name_after,
                    cfg.name_separator,
                    cfg.value_before,
                    property.1,
                    cfg.value_after,
                    cfg.value_separator
                ))?;
            }
            let len = len - 1;
            self.document.write_fmt(format_args!(
                "{}{}{}{}{}{}{}",
                cfg.name_before,
                properties[len].0,
                cfg.name_after,
                cfg.name_separator,
                cfg.value_before,
                properties[len].1,
                cfg.value_after,
            ))?;
            Ok(())
        } else {
            Err("MarkupSth: in this syntaxuration are no properties in tag elements allowed".into())
        }
    }

    pub fn text(&mut self, text: &str) -> Result<()> {
        self.finalize_last_op(TagSequence::text())?;
        self.document.write_str(text)?;
        Ok(())
    }

    pub fn new_line(&mut self) -> Result<()> {
        self.finalize_last_op(TagSequence::linefeed())?;
        // self.seq_state.last.0 = Sequence::LineFeed;
        Ok(())
    }

    pub fn indent_more(&mut self) -> Result<()> {
        self.apply_format_changes(FormatChanges::indent_more(
            self.seq_state.indent,
            self.formatter.get_indent_step_size(),
        ))?;
        Ok(())
    }

    pub fn indent_less(&mut self) -> Result<()> {
        self.apply_format_changes(FormatChanges::indent_less(
            self.seq_state.indent,
            self.formatter.get_indent_step_size(),
        ))?;
        Ok(())
    }

    fn new_line_internal(&mut self) -> Result<()> {
        self.document
            .write_fmt(format_args!("\n{}", self.indent_str))?;
        Ok(())
    }

    pub fn close_all(&mut self) -> Result<()> {
        for _ in 0..self.seq_state.tag_stack.len() {
            self.close()?;
        }
        Ok(())
    }

    pub fn finalize(self) -> Result<()> {
        match self.seq_state.last.0 {
            Sequence::SelfClosing => final_op_arm!(selfclosing self),
            Sequence::Opening => final_op_arm!(opening self),
            Sequence::Closing => final_op_arm!(closing self),
            _ => {}
        }
        Ok(())
    }

    /// This internal method finalizes the last operation, e.g. close the tag. Because the tag
    /// elements will never be closed when inserting them, it has to be done later due to optional
    /// properties, which can be added afterwards.
    fn finalize_last_op(&mut self, next: TagSequence) -> Result<()> {
        // Close last tag (maybe after we have added properties).
        match self.seq_state.last.0 {
            Sequence::Initial => {
                if let Some(dt) = self.syntax.doctype.as_ref() {
                    self.document.write_str(dt)?;
                }
            }
            Sequence::SelfClosing => final_op_arm!(selfclosing self),
            Sequence::Opening => final_op_arm!(opening self),
            Sequence::Closing => final_op_arm!(closing self),
            Sequence::Text | Sequence::LineFeed => {}
        }
        self.seq_state.next = next.clone();
        let check = self.formatter.check(&self.seq_state);
        self.apply_format_changes(check)?;
        self.seq_state.last = next;
        Ok(())
    }

    fn apply_format_changes(&mut self, changes: FormatChanges) -> Result<()> {
        if let Some(indent) = changes.new_indent {
            self.indent_str = " ".repeat(indent);
            self.seq_state.indent = indent;
        }
        if changes.new_line {
            self.new_line_internal()?;
        }
        Ok(())
    }
}

/// Simplifies using `MarkupSth::properties()` and calls this method internally.
#[macro_export]
macro_rules! properties {
    ($markup:expr, $($name:literal, $value:expr),*) => {{
        $markup.properties(&[$(($name, $value)),*])
    }};
}
