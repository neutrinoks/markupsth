//!

use crate::{
    formatting::{FormatChanges, Formatting, LanguageFormatting},
    syntax::{MarkupLanguage, SyntaxConfig},
    tagpattern::{TagPattern, TagType},
};
use std::fmt::Write;

/// Result definition.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Internal state of the MarkupSth for tracking operations.
#[derive(Debug, PartialEq, Eq)]
enum State {
    /// Initial state, no operations so far.
    Initial,
    /// A self-clsoing tag was inserted last.
    SelfClosing,
    /// A opening tag was inserted last.
    Opening,
    /// A closing tag was inserted last.
    Closing,
    /// Some regular text was inserted last.
    Text,
}

#[derive(Debug)]
pub struct MarkupSth<'r> {
    /// Syntax configuration of `MarkupSth`.
    pub syntax: SyntaxConfig,
    /// Formatting configuration of `MarkupSth`.
    pub formatting: Formatting,
    /// Stack of open tags.
    stack: Vec<String>,
    /// Simple optimization.
    indent_str: String,
    /// Internal state of the markup.
    state: State,
    /// Internal log of the last tag to connect to Formatting.
    log: String,
    /// Reference to a Document.
    document: &'r mut String,
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
            $self.syntax.tag_pairs.as_ref().unwrap().opener_after
        ))?;
    }};
    (closing $self:expr) => {{
        $self.document.write_fmt(format_args!(
            "{}",
            $self.syntax.tag_pairs.as_ref().unwrap().closer_after
        ))?;
    }};
}

pub(crate) use final_op_arm;

impl<'d> MarkupSth<'d> {
    /// New type pattern for creating a new MarkupSth.
    pub fn new(document: &'d mut String, cfg: MarkupLanguage) -> Result<MarkupSth<'d>> {
        Ok(MarkupSth {
            syntax: SyntaxConfig::from(cfg.clone()),
            formatting: Formatting::from(cfg),
            stack: Vec::new(),
            indent_str: String::new(),
            state: State::Initial,
            log: String::new(),
            document,
        })
    }

    pub fn set_formatting(&mut self, f: LanguageFormatting) {
        self.formatting = Formatting::from(f);
    }

    /// Inserts a single tag.
    pub fn self_closing(&mut self, tag: &str) -> Result<()> {
        self.finalize_last_op(Some(TagPattern::self_closing(tag)))?;
        if let Some(cfg) = &self.syntax.self_closing {
            self.document
                .write_fmt(format_args!("{}{}", cfg.before, tag))?;
            self.state = State::SelfClosing;
            self.log = tag.to_string();
            Ok(())
        } else {
            Err("MarkupSth: in this syntaxuration are no self-closing tag elements allowed".into())
        }
    }

    pub fn open(&mut self, tag: &str) -> Result<()> {
        self.finalize_last_op(Some(TagPattern::opening(tag)))?;
        if let Some(cfg) = &self.syntax.tag_pairs {
            self.document
                .write_fmt(format_args!("{}{}", cfg.opener_before, tag))?;
            self.stack.push(tag.to_string());
            self.state = State::Opening;
            self.log = tag.to_string();
            Ok(())
        } else {
            Err("MarkupSth: in this syntaxuration are no tag-pair element allowed".into())
        }
    }

    pub fn close(&mut self) -> Result<()> {
        if self.syntax.tag_pairs.is_none() {
            return Err("MarkupSth: in this syntaxuration are no tag-pair element allowed".into())
        }
        if self.stack.is_empty() {
            return Err("MarkupSth: tag-pair stack error".into())
        }

        let tag = self.stack.pop().unwrap();
        self.finalize_last_op(Some(TagPattern::closing(&tag)))?;
        let cfg = self.syntax.tag_pairs.as_ref().unwrap();
        self.document
            .write_fmt(format_args!("{}{}", cfg.closer_before, &tag))?;
        self.state = State::Closing;
        self.log = tag;
        Ok(())
    }

    /// Inserts a single tag with properties.
    pub fn properties(&mut self, properties: &[(String, String)]) -> Result<()> {
        if !matches!(self.state, State::SelfClosing | State::Opening) {
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
        self.finalize_last_op(None)?;
        self.document.write_str(text)?;
        self.state = State::Text;
        Ok(())
    }

    pub fn new_line(&mut self) -> Result<()> {
        self.finalize_last_op(None)?;
        self.new_line_internal()
    }

    fn new_line_internal(&mut self) -> Result<()> {
        self.document.write_fmt(format_args!("\n{}", self.indent_str))?;
        self.state = State::Text;
        Ok(())
    }

    pub fn close_all(&mut self) -> Result<()> {
        for _ in 0..self.stack.len() {
            self.close()?;
        }
        Ok(())
    }

    pub fn finalize(self) -> Result<()> {
        match self.state {
            State::SelfClosing => final_op_arm!(selfclosing self),
            State::Opening => final_op_arm!(opening self),
            State::Closing => final_op_arm!(closing self),
            _ => {}
        }
        Ok(())
    }

    fn finalize_last_op(&mut self, next: Option<TagPattern>) -> Result<()> {
        println!("finalize_last_op({:?})/{:?}/{:?}", self.log, self.state, next);
        let indent = self.indent_str.len();
        match self.state {
            State::Initial => {
                self.state = State::Text; // because nothing happens
                if let Some(dt) = self.syntax.doctype.as_ref() {
                    self.document.write_str(dt)?;
                    self.apply_format_changes(FormatChanges::may_lf(self.formatting.doctype_lf))?;
                }
            }
            State::SelfClosing => {
                final_op_arm!(selfclosing self);
                self.apply_format_changes(self.formatting
                    .new_format(&TagPattern::self_closing(&self.log), indent))?;
            }
            State::Opening => {
                final_op_arm!(opening self);
                self.apply_format_changes(self.formatting
                    .new_format(&TagPattern::opening(&self.log), indent))?;
            }
            State::Closing => {
                final_op_arm!(closing self);
            }
            State::Text => {}
        }
        if let Some(next) = next {
            if matches!(next.typ, TagType::Closing) {
                let indent = self.indent_str.len();
                println!("apply changes because before closing: {:?}", indent);
                let changes = self.formatting
                    .new_format(&next, indent);
                println!("changes: {changes:?}");
                self.apply_format_changes(changes)?;
                // self.apply_format_changes(self.formatting
                //     .new_format(&TagPattern::closing(&self.log), indent))?;
                println!("applied changes because before closing: {:?}", self.indent_str.len());
            }
        }
        Ok(())
    }

    fn apply_format_changes(&mut self, changes: FormatChanges) -> Result<()> {
        if let Some(indent) = changes.new_indent {
            self.indent_str = " ".repeat(indent);
        }
        if changes.new_line {
            self.new_line_internal()?;
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! properties {
    ($($name:literal, $value:literal),*) => {
        vec![$(($name.to_string(), $value.to_string())),*]
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_unformatted_html() {
        let mut document = String::new();
        let mut markup = MarkupSth::new(&mut document, MarkupLanguage::Html).unwrap();
        markup.set_formatting(LanguageFormatting::NoFormatting);
        markup.open("html").unwrap();
        markup.text("Dies ist HTML").unwrap();
        markup.close().unwrap();
        markup.finalize().unwrap();
        assert_eq!(document, "<!DOCTYPE html><html>Dies ist HTML</html>");
    }

    #[test]
    fn unformatted_html_with_properties() {
        let mut document = String::new();
        let mut markup = MarkupSth::new(&mut document, MarkupLanguage::Html).unwrap();
        markup.set_formatting(LanguageFormatting::NoFormatting);
        markup.open("body").unwrap();
        markup.open("section").unwrap();
        markup.properties(&properties!["class", "class"]).unwrap();
        markup.open("div").unwrap();
        markup
            .properties(&properties!["keya", "value1", "keyb", "value2"])
            .unwrap();
        markup.text("Text").unwrap();
        markup.self_closing("img").unwrap();
        markup.properties(&properties!["src", "img.jpg"]).unwrap();
        markup.close_all().unwrap();
        markup.finalize().unwrap();
        assert_eq!(
            document,
            concat![r#"<!DOCTYPE html><body><section class="class">"#,
                r#"<div keya="value1" keyb="value2">"#,
                r#"Text<img src="img.jpg"></div></section></body>"#]
        );
    }

    #[test]
    fn default_html_formatting() {
        let mut document = String::new();
        let mut markup = MarkupSth::new(&mut document, MarkupLanguage::Html).unwrap();
        println!("Markup: {:?}", markup.formatting);
        markup.open("head").unwrap();
        markup.self_closing("meta").unwrap();
        markup.properties(&properties!["charset", "utf-8"]).unwrap();
        markup.close().unwrap();
        markup.open("body").unwrap();
        markup.open("section").unwrap();
        markup.open("div").unwrap();
        markup.open("p").unwrap();
        markup.text("Text").unwrap();
        markup.close().unwrap();
        markup.close().unwrap();
        markup.close().unwrap();
        markup.close().unwrap();
        markup.close_all().unwrap();
        assert_eq!(
            document,
            concat![
                r#"<!DOCTYPE html>"#, '\n',
                r#"<head>"#, '\n',
                r#"    <meta charset="utf-8">"#, '\n',
                r#"</head>"#, '\n',
                r#"<body>"#, '\n',
                r#"    <section>"#, '\n',
                r#"        <div><p>Text</p></div>"#, '\n',
                r#"    </section>"#, '\n',
                r#"</body>"#, '\n']
        );
    }
}
