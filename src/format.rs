//! This module implements optional tag pattern related auto-formattings, respectively
//! auto-line-feed and auto-indenting. Because Markup languages usually have tag pair elements,
//! e.g. in HTML `<head></head>`, we can specify if we want auto indenting between them. Also we
//! could define the rule to insert a line feed automatically after each `<img>` tag.
//!
//! This module contains all implementations and definitions to setup the formatting in
//! `MarkupSth`. Available options are currently indenting-step-size, auto-line-feed, and
//! auto-indenting.

// use crate::{syntax::MarkupLanguage, tag_rule_set};

pub const DEFAULT_INDENT: usize = 4;

/// Internal state of the MarkupSth for tracking operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Sequence {
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
    /// A new line affects auto-indenting, so we need to log it too.
    LineFeed,
}

/// A certain sequence in ML-generation, that means a 'Sequence' with a tag identifier.
#[derive(Debug, Clone)]
pub struct TagSequence<'s>(Sequence, &'s str);

impl<'s> TagSequence<'s> {
    pub fn self_closing(tag: &'s str) -> TagSequence<'s> {
        TagSequence(Sequence::SelfClosing, tag)
    }

    pub fn opening(tag: &'s str) -> TagSequence<'s> {
        TagSequence(Sequence::Opening, tag)
    }

    pub fn closing(tag: &'s str) -> TagSequence<'s> {
        TagSequence(Sequence::Closing, tag)
    }

    pub fn text() -> TagSequence<'s> {
        TagSequence(Sequence::Text, "")
    }

    pub fn linefeed() -> TagSequence<'s> {
        TagSequence(Sequence::LineFeed, "")
    }

    pub fn from(seq: &Sequence, tag: &'s str) -> TagSequence<'s> {
        match seq {
            Sequence::SelfClosing | Sequence::Opening | Sequence::Closing => {
                TagSequence(seq.clone(), tag)
            }
            _ => TagSequence(seq.clone(), ""),
        }
    }
}

/// Which formatting rules shall be applied, can be specified by this enum. Keep in mind, that on
/// self-closing tag elements only line-feed rules can be applied, because the indenting is related
/// to tag-pairs. Whether it shall always be indented or only after a line break does only take
/// affect between tag pairs.
///
/// Use cases:
/// - Self-closing tag: nothing / line break after
/// - Tag pairs: nothing / always with indenting and line feeds / indented when line feed inserted
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TagFormatting {
    /// Insert always a line-feed after tag. Can be applied on all tag variants.
    LineFeed,
    /// Always line feed and indent between a tag pair. Cannot be applied to self-closing tags.
    AlwaysIndent,
    /// Auto detect for optional indenting between tag pairs. Cannot be applied to self-closing.
    AutoIndent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TagRule {
    /// Identifier, e.g. `html` or `div`.
    pub ident: String,
    /// Rule to be applied with this tag.
    pub format: TagFormatting,
}

impl TagRule {
    /// New type pattern for creating a new `Tag` of type `Opening`.
    #[inline(always)]
    pub fn line_feed(ident: &str) -> TagRule {
        TagRule {
            ident: ident.to_string(),
            format: TagFormatting::LineFeed,
        }
    }

    /// New type pattern for creating a new `Tag` of type `Opening`.
    #[inline(always)]
    pub fn indented(ident: &str) -> TagRule {
        TagRule {
            ident: ident.to_string(),
            format: TagFormatting::AlwaysIndent,
        }
    }

    /// New type pattern for creating a new `Tag` of type `Opening`.
    #[inline(always)]
    pub fn auto_indent(ident: &str) -> TagRule {
        TagRule {
            ident: ident.to_string(),
            format: TagFormatting::AutoIndent,
        }
    }
}

#[macro_export]
macro_rules! tag_rule_set_internal {
    (linefeed $ident:literal) => {
        TagRule::line_feed($ident)
    };
    (indented $ident:literal) => {
        TagRule::indented($ident)
    };
    (auto $ident:literal) => {
        TagRule::auto_indent($ident)
    };
}

#[macro_export]
macro_rules! tag_rule_set {
    ($($rule:tt $ident:literal),*) => {
        vec![
            $($crate::tag_rule_set_internal!($rule $ident),)*
        ]
    }
}

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub enum LanguageFormatting {
//     /// Absolutely no formatting.
//     NoFormatting,
//     /// Generic block oriented indenting, blocks will always be indented.
//     GenericAlwaysIndent,
//     /// A pre-defined formatting for readable HTML.
//     CleanHtml,
//     /// Individual formatting configuration.
//     Other(Formatting),
// }

// impl From<MarkupLanguage> for LanguageFormatting {
//     fn from(ml: MarkupLanguage) -> LanguageFormatting {
//         match ml {
//             MarkupLanguage::Html => LanguageFormatting::CleanHtml,
//             _ => LanguageFormatting::NoFormatting,
//         }
//     }
// }

#[derive(Clone, Debug)]
pub struct FormatChanges {
    /// Shall a new line (line feed) be inserted after inserting certain tag elements.
    pub new_line: bool,
    /// Need indenting to be updated, and if yes, what is the new indenting and has it to be
    /// applied before inserting a tag element. For example in case of closing tag elements the
    /// indenting may be changed before.
    pub new_indent: Option<usize>,
}

impl FormatChanges {
    /// In case nothing shall be changed. No line indenting changes, and no line-feed will be
    /// inserted.
    pub fn nothing() -> FormatChanges {
        FormatChanges {
            new_line: false,
            new_indent: None,
        }
    }

    /// In case, a new line (line feed) may be inserted, this function may suit your needs.
    /// Indenting will not be touched, onle the `new_line` flag passed through.
    pub fn may_lf(new_line: bool) -> FormatChanges {
        FormatChanges {
            new_line,
            new_indent: None,
        }
    }

    /// An indented block is following. New line and increase indenting.
    pub fn indent_more(indent: usize, step: usize) -> FormatChanges {
        FormatChanges {
            new_line: true,
            new_indent: Some(indent + step),
        }
    }

    /// An indented block is ending. New line and decrease indenting.
    pub fn indent_less(indent: usize, step: usize) -> FormatChanges {
        let new_indent = if step > indent {
            Some(0)
        } else {
            Some(indent - step)
        };
        FormatChanges {
            new_line: true,
            new_indent,
        }
    }
}

/// TODO
pub trait Formatter: std::fmt::Debug {
    /// New type pattern as default.
    fn new() -> Self
    where
        Self: Sized;

    /// Sets the indenting step size, pre-defined default is 4.
    fn set_indent_size(&mut self, step_size: usize);

    /// Check for format changes between last und next `TagSequence`. Neccessary changes regarding
    /// indenting and/or line-feed will be returned by a `FormatChanges`.
    fn check(&self, last: &TagSequence, next: &TagSequence, indent: usize) -> FormatChanges;
}

pub mod generic {
    use super::*;

    /// A pre-defined `Formatter`. TODO
    #[derive(Debug)]
    pub struct NoFormatting;

    impl Formatter for NoFormatting {
        /// See trait description.
        fn new() -> NoFormatting {
            NoFormatting
        }

        /// See trait description.
        fn set_indent_size(&mut self, _: usize) {}

        /// See trait description.
        fn check(&self, _: &TagSequence, _: &TagSequence, _: usize) -> FormatChanges {
            FormatChanges::nothing()
        }
    }

    /// A pre-defined `Formatter`. TODO
    #[derive(Debug)]
    pub struct AlwaysIndentAlwaysLf(usize);

    impl Formatter for AlwaysIndentAlwaysLf {
        /// See trait description.
        fn new() -> AlwaysIndentAlwaysLf {
            AlwaysIndentAlwaysLf(DEFAULT_INDENT)
        }

        /// See trait description.
        fn set_indent_size(&mut self, step_size: usize) {
            self.0 = step_size;
        }

        /// See trait description.
        fn check(&self, last: &TagSequence, next: &TagSequence, indent: usize) -> FormatChanges {
            if matches!(next.0, Sequence::Closing) {
                match last.0 {
                    Sequence::Opening => FormatChanges::may_lf(true),
                    _ => FormatChanges::indent_less(indent, self.0),
                }
            } else {
                match last.0 {
                    Sequence::Initial => FormatChanges::may_lf(true),
                    Sequence::Opening => match next.0 {
                        Sequence::SelfClosing | Sequence::Text | Sequence::Opening => {
                            FormatChanges::indent_more(indent, self.0)
                        }
                        _ => FormatChanges::nothing(),
                    },
                    Sequence::Closing => FormatChanges::may_lf(true),
                    Sequence::SelfClosing => FormatChanges::may_lf(true),
                    _ => FormatChanges::nothing(),
                }
            }
        }
    }
}

// /// Main configuration struct for formatting in `MarkupSth`.
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct Formatting {
//     /// Step size of indenting.
//     pub indent_step: usize,
//     /// New line after doctype information.
//     pub doctype_lf: bool,
//     /// Pattern setup for auto-line-feed.
//     pub tag_rules: Vec<TagRule>,
// }

// impl Formatting {
//     pub fn new_format(&self, last: TagSequence, next: TagSequence, indent: usize) -> FormatChanges {
//         let mut new_line = false;
//         let mut new_indent = None;

//         for rule in self.tag_rules.iter() {
//             let last_matches = rule.ident == last.1;
//             let next_matches = rule.ident == next.1;
//             if last_matches {
//                 match rule.format {
//                     TagFormatting::LineFeed => {
//                     }
//                     TagFormatting::AlwaysIndent => {
//                     }
//                     TagFormatting::AutoIndent => {
//                     }
//                 }
//             }
//             if next_matches {
//             }
//         }

//         FormatChanges { new_line, new_indent }
//     }
// }

// impl From<MarkupLanguage> for Formatting {
//     fn from(ml: MarkupLanguage) -> Formatting {
//         Formatting::from(LanguageFormatting::from(ml))
//     }
// }

// impl From<LanguageFormatting> for Formatting {
//     fn from(formatting: LanguageFormatting) -> Formatting {
//         match formatting {
//             LanguageFormatting::NoFormatting => GenFmtrNoFormatting::new(4),
//             LanguageFormatting::GenericAlwaysIndent => GenFmtrAlwaysIndent::new(4),
//             // LanguageFormatting::CleanHtml => Formatting {
//             //     indent_step: 4,
//             //     doctype_lf: true,
//             //     tag_rules: tag_rule_set![
//             //         indented "html", indented "head", indented "head", indented "body",
//             //         indented "section", indented "header", indented "nav", indented "footer",
//             //         auto "div", indented "ul", linefeed "li"],
//             // },
//             LanguageFormatting::Other(f) => f,
//             _ => todo!(),
//         }
//     }
// }
