//! This module contains all implementations and definitions to setup the formatting in
//! `MarkupSth`. Available options are currently indenting-step-size, auto-line-feed, and
//! auto-indenting.

use crate::{
    syntax::MarkupLanguage,
    tag_rule_set,
    tagpattern::{TagPattern, TagType},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LanguageFormatting {
    /// Absolutely no formatting.
    NoFormatting,
    /// A pre-defined formatting for readable HTML.
    CleanHtml,
    /// Individual formatting configuration.
    Other(Formatting),
}

impl From<MarkupLanguage> for LanguageFormatting {
    fn from(ml: MarkupLanguage) -> LanguageFormatting {
        match ml {
            MarkupLanguage::Html => LanguageFormatting::CleanHtml,
            _ => LanguageFormatting::NoFormatting,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FormatChanges {
    /// Is line-feed required?
    pub new_line: bool,
    /// Need indenting to be updated?
    pub new_indent: Option<usize>,
}

impl FormatChanges {
    /// A simple situation, where nothing shall be changed, but also not a default situation.
    pub fn nothing() -> FormatChanges {
        FormatChanges{ new_line: false, new_indent: None }
    }

    pub fn may_lf(new_line: bool) -> FormatChanges {
        FormatChanges{ new_line, new_indent: None }
    }
}

/// Main configuration struct for formatting in `MarkupSth`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Formatting {
    /// Step size of indenting.
    pub indent_step: usize,
    /// New line after doctype information.
    pub doctype_lf: bool,
    /// Flag for automatic line feed.
    pub auto_lf: bool,
    /// Pattern setup for auto-line-feed.
    pub auto_lf_pat: Vec<TagPattern>,
    /// Flag for automatic indenting.
    pub auto_indenting: bool,
    /// Pattern setup for auto-indenting.
    pub auto_indent_pat: Vec<TagPattern>,
}

impl Formatting {
    pub fn new_format(&self, tag: &TagPattern, indent: usize) -> FormatChanges {
        FormatChanges {
            new_line: self.new_line(tag),
            new_indent: self.new_indent(tag, indent),
        }
    }

    fn new_line(&self, tag_p: &TagPattern) -> bool {
        if self.auto_lf {
            for pat in self.auto_lf_pat.iter() {
                if pat == tag_p {
                    return true;
                }
            }
        }
        false
    }

    fn new_indent(&self, tag: &TagPattern, indent: usize) -> Option<usize> {
        if self.auto_indenting {
            for pat in self.auto_indent_pat.iter() {
                if pat == tag {
                    match tag.typ {
                        TagType::Closing => {
                            if self.indent_step > indent {
                                return Some(0)
                            } else {
                                return Some(indent - self.indent_step)
                            }
                        }
                        TagType::Opening => {
                            return Some(indent + self.indent_step)
                        }
                        _ => {}
                    }
                }
            }
        }
        None
    }
}

impl From<MarkupLanguage> for Formatting {
    fn from(ml: MarkupLanguage) -> Formatting {
        Formatting::from(LanguageFormatting::from(ml))
    }
}

impl From<LanguageFormatting> for Formatting {
    fn from(formatting: LanguageFormatting) -> Formatting {
        match formatting {
            LanguageFormatting::NoFormatting => Formatting {
                indent_step: 4,
                doctype_lf: false,
                auto_lf: false,
                auto_lf_pat: Vec::new(),
                auto_indenting: false,
                auto_indent_pat: Vec::new(),
            },
            LanguageFormatting::CleanHtml => Formatting {
                indent_step: 4,
                doctype_lf: true,
                auto_lf: true,
                auto_lf_pat: tag_rule_set![
                    on_open "html", on_both "head", on_both "head", on_both "body",
                    on_both "section", on_both "header", on_both "nav", on_both "footer",
                    on_close "div", on_close "ul", on_close "li"],
                auto_indenting: true,
                auto_indent_pat: tag_rule_set![
                    on_both "head", on_both "body", on_both "section", on_both "header",
                    on_both "nav", on_both "footer", on_both "ul"],
            },
            LanguageFormatting::Other(f) => f,
        }
    }
}
