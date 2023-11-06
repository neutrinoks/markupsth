//!

use crate::{
    formatting::{Formatting, LanguageFormatting},
    syntax::{MarkupLanguage, SyntaxConfig},
};
use std::fmt::Write;

/// Result definition.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Internal state of the MarkupSth for tracking operations.
#[derive(Debug, PartialEq, Eq)]
enum State {
    /// Initial state, no operations so far.
    Initial,
    /// Doctype have been inserted last.
    DocType,
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
        let mut markup = MarkupSth {
            syntax: SyntaxConfig::from(cfg.clone()),
            formatting: Formatting::from(cfg),
            stack: Vec::new(),
            indent_str: String::new(),
            state: State::Initial,
            document,
        };
        markup.insert_doctype()?;
        Ok(markup)
    }

    pub fn set_formatting(&mut self, f: LanguageFormatting) {
        self.formatting = Formatting::from(f);
    }

    /// Inserts a single tag.
    pub fn self_closing(&mut self, tag: &str) -> Result<()> {
        self.finalize_last_op()?;
        if let Some(cfg) = &self.syntax.self_closing {
            self.document
                .write_fmt(format_args!("{}{}", cfg.before, tag))?;
            self.state = State::SelfClosing;
            Ok(())
        } else {
            Err("MarkupSth: in this syntaxuration are no self-closing tag elements allowed".into())
        }
    }

    pub fn open(&mut self, tag: &str) -> Result<()> {
        self.finalize_last_op()?;
        if let Some(cfg) = &self.syntax.tag_pairs {
            self.document
                .write_fmt(format_args!("{}{}", cfg.opener_before, tag))?;
            self.stack.push(tag.to_string());
            self.state = State::Opening;
            Ok(())
        } else {
            Err("MarkupSth: in this syntaxuration are no tag-pair element allowed".into())
        }
    }

    pub fn close(&mut self) -> Result<()> {
        self.finalize_last_op()?;
        if let Some(cfg) = &self.syntax.tag_pairs {
            if let Some(tag) = self.stack.pop() {
                self.document
                    .write_fmt(format_args!("{}{}", cfg.closer_before, tag))?;
                self.state = State::Closing;
                Ok(())
            } else {
                Err("MarkupSth: tag-pair stack error".into())
            }
        } else {
            Err("MarkupSth: in this syntaxuration are no tag-pair element allowed".into())
        }
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
        self.finalize_last_op()?;
        self.document.write_str(text)?;
        self.state = State::Text;
        Ok(())
    }

    pub fn new_line(&mut self) -> Result<()> {
        Ok(self
            .document
            .write_fmt(format_args!("\n{}", self.indent_str))?)
    }

    pub fn close_all(&mut self) -> Result<()> {
        println!("{:?}", self.stack);
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

    fn finalize_last_op(&mut self) -> Result<()> {
        match self.state {
            State::Initial => {
                panic!("MarkupSth: in Initial state there is nothing to be finalized")
            }
            State::SelfClosing => final_op_arm!(selfclosing self),
            State::Opening => final_op_arm!(opening self),
            State::Closing => final_op_arm!(closing self),
            State::DocType | State::Text => {}
        }
        Ok(())
    }

    fn insert_doctype(&mut self) -> Result<()> {
        assert!(self.state == State::Initial);
        if let Some(def) = &self.syntax.doctype {
            self.document.write_str(def)?;
        }
        self.state = State::DocType;
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
            r#"<!DOCTYPE html><body><section class="class"><div keya="value1" keyb="value2">Text<img src="img.jpg"></div></section></body>"#
        );
    }

    #[test]
    fn formatted_html() {}
}
