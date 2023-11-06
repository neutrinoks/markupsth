//!

use crate::config::{Config, MarkupConfig};
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
    /// Configuration of MarkupSth.
    pub config: Config,
    /// Stack of open tags.
    stack: Vec<String>,
    /// Internal state of the printer.
    state: State,
    /// Reference to a Document.
    document: &'r mut String,
}

impl<'d> MarkupSth<'d> {
    /// New type pattern for creating a new MarkupSth.
    pub fn new(document: &'d mut String, cfg: MarkupConfig) -> Result<MarkupSth<'d>> {
        let mut printer = MarkupSth {
            config: Config::from(cfg),
            stack: Vec::new(),
            state: State::Initial,
            document,
        };
        printer.insert_doctype()?;
        Ok(printer)
    }

    /// Inserts a single tag.
    pub fn self_closing_tag(&mut self, tag: &str) -> Result<()> {
        self.finalize_last_op()?;
        if let Some(cfg) = &self.config.self_closing {
            self.document
                .write_fmt(format_args!("{}{}", cfg.before, tag))?;
            self.state = State::SelfClosing;
            Ok(())
        } else {
            Err("MarkupSth: in this configuration are no self-closing tag elements allowed".into())
        }
    }

    pub fn open_tag(&mut self, tag: &str) -> Result<()> {
        self.finalize_last_op()?;
        if let Some(cfg) = &self.config.tag_pairs {
            self.document
                .write_fmt(format_args!("{}{}", cfg.opener_before, tag))?;
            self.stack.push(tag.to_string());
            self.state = State::Opening;
            Ok(())
        } else {
            Err("MarkupSth: in this configuration are no tag-pair element allowed".into())
        }
    }

    pub fn close_tag(&mut self) -> Result<()> {
        self.finalize_last_op()?;
        if let Some(cfg) = &self.config.tag_pairs {
            if let Some(tag) = self.stack.pop() {
                self.document
                    .write_fmt(format_args!("{}{}", cfg.closer_before, tag))?;
                self.state = State::Closing;
                Ok(())
            } else {
                Err("MarkupSth: tag-pair stack error".into())
            }
        } else {
            Err("MarkupSth: in this configuration are no tag-pair element allowed".into())
        }
    }

    /// Inserts a single tag with properties.
    pub fn add_properties(&mut self, properties: &[(String, String)]) -> Result<()> {
        if !matches!(self.state, State::SelfClosing | State::Opening) {
            return Err(
                "MarkupSth: properties can only be added to self-closing or opening tags".into(),
            );
        }

        if let Some(cfg) = &self.config.properties {
            self.document.write_fmt(format_args!("{}", cfg.initiator))?;
            let len = properties.len();
            for property in properties[..len - 1].iter() {
                self.document.write_fmt(format_args!(
                    "{}{}{}{}",
                    property.0, cfg.name_separator, property.1, cfg.value_separator
                ))?;
            }
            self.document.write_fmt(format_args!(
                "{}{}{}",
                properties[len].0, cfg.name_separator, properties[len].1
            ))?;
            Ok(())
        } else {
            Err("MarkupSth: in this configuration are no properties in tag elements allowed".into())
        }
    }

    pub fn text(&mut self, text: &str) -> Result<()> {
        self.finalize_last_op()?;
        self.document.write_str(text)?;
        self.state = State::Text;
        Ok(())
    }

    pub fn finalize(self) -> Result<()> {
        match self.state {
            State::SelfClosing => {
                self.document.write_fmt(format_args!(
                    "{}",
                    self.config.self_closing.as_ref().unwrap().after
                ))?;
            }
            State::Opening => {
                self.document.write_fmt(format_args!(
                    "{}",
                    self.config.tag_pairs.as_ref().unwrap().opener_after
                ))?;
            }
            State::Closing => {
                self.document.write_fmt(format_args!(
                    "{}",
                    self.config.tag_pairs.as_ref().unwrap().closer_after
                ))?;
            }
            _ => {}
        }
        Ok(())
    }

    fn finalize_last_op(&mut self) -> Result<()> {
        match self.state {
            State::Initial => {
                panic!("MarkupSth: in Initial state there is nothing to be finalized")
            }
            State::SelfClosing => {
                self.document.write_fmt(format_args!(
                    "{}",
                    self.config.self_closing.as_ref().unwrap().after
                ))?;
            }
            State::Opening => {
                self.document.write_fmt(format_args!(
                    "{}",
                    self.config.tag_pairs.as_ref().unwrap().opener_after
                ))?;
            }
            State::Closing => {
                self.document.write_fmt(format_args!(
                    "{}",
                    self.config.tag_pairs.as_ref().unwrap().closer_after
                ))?;
            }
            State::DocType | State::Text => {}
        }
        Ok(())
    }

    fn insert_doctype(&mut self) -> Result<()> {
        assert!(self.state == State::Initial);
        if let Some(def) = &self.config.doctype {
            self.document.write_str(def)?;
        }
        self.state = State::DocType;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_html() {
        let mut document = String::new();
        let mut printer = MarkupSth::new(&mut document, MarkupConfig::Html)
            .expect("MarkupSth::new() fail");
        printer.open_tag("html").expect(r#"opening("html") fail"#);
        printer.text("Dies ist HTML").expect(r#"content fail"#);
        printer.close_tag();
        printer.finalize();
        assert_eq!(document, "<!DOCTYPE html><html>Dies ist HTML</html>");
    }
}
