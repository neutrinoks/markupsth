//! Contains implementations to model a tag-sequence, changes on format between two following
//! sequences, any kind of formatter, the state of an arbitrary formatter, or optional features of
//! those formatters to be used in MarkupSth.
//!
//! ### General Concept
//!
//! In this crate will be assumed, that any kind of formatter works on the fly, while editing, not
//! afterwards. Therefore an optional change of current formatting has to be applied between two
//! sequences (the past one and before the next one). Such optional changes of format can either be
//! a linefeed and/or a change of current indenting, but it cannot be change of indenting without a
//! linefeed.
//!
//! Any kind of formatter will therefor described by the trait `Formatter` in a very simple way.
//! Optional more complex formatters will describe their features by additional traits. Currently
//! there is only one additional feature trait `FixedRuleset`.
//!
//! ### Pre-defined Formatters
//!
//! You do not have to implement your own formatter anyway - there are three types of pre-defined
//! ones available in module `formatters`, have a look at them! By default `MarkupSth` is using the
//! pre-defined formatter `AutoIndent`.
//!
//! ### Auto-Indenting Rules
//!
//! At the moment, the pre-implemented formatter `AutoIndent` will be sufficient for the most cases.
//! The formatter `AutoIndent` can be configured by three simple rules:
//!
//! - **Indent-Always**: Can only be applied to tag pairs, not to self-closing tags. Tags assigned
//!   to this rule, will indent everything between them. Tags who are assigned to this rule, cannot
//!   be assigned to rule LF-Always too, but they can be assigned to rule LF-Closing.
//! - **LF-Always**: Can only be applied to tag pairs, not to self-closing tags. Tags assigned to
//!   this rule, will have a linefeed inserted after each tag, and eventually before also, opening
//!   and closing tag. Tags assigned to this rule, cannot be assigned to nor Indent-Always, nor
//!   LF-Closing. The philosophy here is to force such tags to have always a single line.
//! - **LF-Closing**: Can be applied to all kind of tags. For closing tags assigned to this rule, a
//!   linefeed will be inserted after it. Tags who are assigned to this rule, can also be assigne to
//!   rule Indent-Always, but not to LF-Always.
//!
//! So, tags can be assigned to rule **LF-Always** or to rule **Indent-Always**, but not both.
//! Optionally, **Indent-Always** can be combined with **LF-Closing**.
//!
//! Formatters who implement this ruleset will also implement the trait `FixedRuleset`. There is
//! one pre-defined formatter available in module `formatters`, named `AutoIndent`.

use crate::Result;

/// Crate default and initial indenting step size. Can be overwritten by trait methods.
pub const DEFAULT_INDENT: usize = 4;

/// Defines the type of a sequence (tags, text and linefeeds) from perspective of a formatter.
///
/// A Markup Language can have tag pair elements, self-closing elements, some initial header tag,
/// regular text content or manual linefeeds, which all of them can influence the behavior of a
/// formatter in MarkupSth.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Sequence {
    /// The document's headline, e.g. `<!DOCTYPE html>`.
    Initial,
    /// A self-clsoing tag, e.g. `<img href="image.jpg">`.
    SelfClosing,
    /// An opening tag in a tag pair, e.g. `<section>`.
    Opening,
    /// A closing tag in a tag pair, e.g. `</section>`.
    Closing,
    /// Regular text content, not related to text and usually no influence on formatting.
    Text,
    /// Linefeed inserted manually by a `MarkupSth`-user. Important to know for auto-indenting.
    LineFeed,
}

/// Pendant to the raw `Sequence`, but combined with a `String` to differ between various tags.
#[derive(Debug, Clone)]
pub struct TagSequence(pub Sequence, pub String);

impl<'s> TagSequence {
    /// Shortcut method for enum variant `SelfClosing`.
    pub fn self_closing(tag: &'s str) -> TagSequence {
        TagSequence(Sequence::SelfClosing, tag.to_string())
    }

    /// Shortcut method for enum variant `Opening`.
    pub fn opening(tag: &'s str) -> TagSequence {
        TagSequence(Sequence::Opening, tag.to_string())
    }

    /// Shortcut method for enum variant `Closing`.
    pub fn closing(tag: &'s str) -> TagSequence {
        TagSequence(Sequence::Closing, tag.to_string())
    }

    /// Shortcut method for enum variant `Text`.
    pub fn text() -> TagSequence {
        TagSequence(Sequence::Text, String::new())
    }

    /// Shortcut method for enum variant `LineFeed`.
    pub fn linefeed() -> TagSequence {
        TagSequence(Sequence::LineFeed, String::new())
    }

    /// Shortcut method for enum variant `Initial`.
    pub fn initial() -> TagSequence {
        TagSequence(Sequence::Initial, String::new())
    }

    /// Pendant method to trait `From` implementation.
    pub fn from(seq: &Sequence, tag: &'s str) -> TagSequence {
        match seq {
            Sequence::SelfClosing | Sequence::Opening | Sequence::Closing => {
                TagSequence(seq.clone(), tag.to_string())
            }
            _ => TagSequence(seq.clone(), String::new()),
        }
    }
}

/// Defines the state of any arbitrary formatter in MarkupSth. Probably only used in `MarkupSth`.
///
/// The `SequenceState` encapsules everything one need to know, to create change orders related to
/// formatting (order for changes, e.g. indent more, less or insert line feed etc.). This changes
/// are described by the `FormatChanges` definition.
#[derive(Debug)]
pub struct SequenceState {
    /// Stack of open tags.
    pub tag_stack: Vec<String>,
    /// Internal log of the last tag sequence.
    pub last: TagSequence,
    /// Next tag to be printed (just commanded).
    pub next: TagSequence,
    /// Current steps of indenting in total.
    pub indent: usize,
}

impl SequenceState {
    /// New type pattern for a default `SequenceState`, which starts with an `Initial` sequence.
    pub fn new() -> SequenceState {
        SequenceState {
            tag_stack: Vec::new(),
            last: TagSequence::initial(),
            next: TagSequence::text(),
            indent: 0,
        }
    }

    #[cfg(test)]
    pub(crate) fn teststate(last: TagSequence, next: TagSequence) -> SequenceState {
        SequenceState {
            tag_stack: Vec::new(),
            last,
            next,
            indent: DEFAULT_INDENT,
        }
    }

    /// Only for testing purposes used internally.
    #[cfg(test)]
    pub(crate) fn initial_open(next: &str) -> SequenceState {
        Self::teststate(TagSequence::initial(), TagSequence::opening(next))
    }

    /// Only for testing purposes used internally.
    #[cfg(test)]
    pub(crate) fn open_open(last: &str, next: &str) -> SequenceState {
        Self::teststate(TagSequence::opening(last), TagSequence::opening(next))
    }

    /// Only for testing purposes used internally.
    #[cfg(test)]
    pub(crate) fn open_close(last: &str, next: &str) -> SequenceState {
        Self::teststate(TagSequence::opening(last), TagSequence::closing(next))
    }

    /// Only for testing purposes used internally.
    #[cfg(test)]
    pub(crate) fn open_text(last: &str) -> SequenceState {
        Self::teststate(TagSequence::opening(last), TagSequence::text())
    }

    /// Only for testing purposes used internally.
    #[cfg(test)]
    pub(crate) fn open_lf(last: &str) -> SequenceState {
        Self::teststate(TagSequence::opening(last), TagSequence::linefeed())
    }

    /// Only for testing purposes used internally.
    #[cfg(test)]
    pub(crate) fn open_self_closing(last: &str, next: &str) -> SequenceState {
        Self::teststate(TagSequence::opening(last), TagSequence::self_closing(next))
    }

    /// Only for testing purposes used internally.
    #[cfg(test)]
    pub(crate) fn close_close(last: &str, next: &str) -> SequenceState {
        Self::teststate(TagSequence::closing(last), TagSequence::closing(next))
    }

    /// Only for testing purposes used internally.
    #[cfg(test)]
    pub(crate) fn close_text(last: &str) -> SequenceState {
        Self::teststate(TagSequence::closing(last), TagSequence::text())
    }

    /// Only for testing purposes used internally.
    #[cfg(test)]
    pub(crate) fn lf_self_closing(next: &str) -> SequenceState {
        Self::teststate(TagSequence::linefeed(), TagSequence::self_closing(next))
    }

    /// Only for testing purposes used internally.
    #[cfg(test)]
    pub(crate) fn self_closing_close(last: &str, next: &str) -> SequenceState {
        Self::teststate(TagSequence::self_closing(last), TagSequence::closing(next))
    }

    /// Only for testing purposes used internally.
    #[cfg(test)]
    pub(crate) fn text_close(last: &str) -> SequenceState {
        Self::teststate(TagSequence::text(), TagSequence::closing(last))
    }
}

impl Default for SequenceState {
    fn default() -> SequenceState {
        SequenceState::new()
    }
}

/// Optional changes of current format between two sequences will be described by this definition.
///
/// Possible changes can either be a linefeed and changes in current indenting. Usually there
/// cannot be a change on indenting while there is no linefeed, this is against the general concept
/// of formatting in this crate.
///
/// Those included changes will always be applied instaniously, which means after the last inserted
/// tag, and before the next one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormatChanges {
    /// Flag for a linefeed to be inserted.
    pub new_line: bool,
    /// Optional: New indenting size in case of a linefeed.
    pub new_indent: Option<usize>,
}

impl FormatChanges {
    /// In case, nothing shall be changed. No linefeed will be inserted, no change on indenting.
    pub fn nothing() -> FormatChanges {
        FormatChanges {
            new_line: false,
            new_indent: None,
        }
    }

    /// Insert a line feed without changing the current indenting.
    pub fn lf() -> FormatChanges {
        FormatChanges {
            new_line: true,
            new_indent: None,
        }
    }

    /// In case, a new line (line feed) may be inserted, this function may suit your needs.
    pub fn may_lf(new_line: bool) -> FormatChanges {
        FormatChanges {
            new_line,
            new_indent: None,
        }
    }

    /// Indenting gets increased without additional new line (line feed).
    pub fn indent_more(indent: usize, step: usize) -> FormatChanges {
        FormatChanges {
            new_line: false,
            new_indent: Some(indent + step),
        }
    }

    /// An indented block is following. New line and increase current indenting.
    pub fn lf_indent_more(indent: usize, step: usize) -> FormatChanges {
        let mut fc = FormatChanges::indent_more(indent, step);
        fc.new_line = true;
        fc
    }

    /// An indented block is ending. New line and decrease current indenting.
    pub fn indent_less(indent: usize, step: usize) -> FormatChanges {
        let new_indent = if step > indent {
            Some(0)
        } else {
            Some(indent - step)
        };
        FormatChanges {
            new_line: false,
            new_indent,
        }
    }

    /// An indented block is following. New line and increase current indenting.
    pub fn lf_indent_less(indent: usize, step: usize) -> FormatChanges {
        let mut fc = FormatChanges::indent_less(indent, step);
        fc.new_line = true;
        fc
    }
}

/// Defines the basic bahavior of any formatter in this crate. Extensions are defined by other
/// traits.
///
/// Regarding the general concept, described in module `format`, this traits maps the functionality
/// of setting/getting the indenting-step-size and the core function `check`, which checks for
/// possible format changes between two sequences. A default constructor and a reset method is also
/// expected. Addition features have to be described by additional crates and can be requested via
/// this one (see `optional_fixed_ruleset`).
pub trait Formatter: std::fmt::Debug {
    /// New type pattern as default for constructing any kind of `Formatter`. The crate's default
    /// indenting step size `DEFAULT_INDENT` shall be set after calling this method.
    fn new() -> Self
    where
        Self: Sized;

    /// Modify and set the indenting-step-size. Default is `DEFAULT_INDENT`.
    fn set_indent_step_size(&mut self, _step_size: usize) {}

    /// Returns the current indenting-step-size.
    fn get_indent_step_size(&self) -> usize {
        DEFAULT_INDENT
    }

    /// Whatever may configurable and may have been re-configured, this function shall reset all
    /// configurable properties back to their defaults.
    fn reset_to_defaults(&mut self) {}

    /// The core function of this crate's general concept. It shall check for optional format
    /// changes between the last inserted tag and the next one, before it will get inserted into
    /// the document under edit.
    fn check(&mut self, state: &SequenceState) -> FormatChanges;

    /// Returns this special kind of Formatter.
    fn get_ext_auto_indenting(&mut self) -> Option<&mut dyn ExtAutoIndenting> {
        None
    }
}

/// Selector for available auto-formatting rules for the `AutoFormatter`.
///
/// The `AutoFormatter` is one of the default formatter implementations, which is a pre-defined
/// extension of the basic `Formatter` trait.
#[derive(Copy, Clone, Debug)]
pub enum AutoFmtRule {
    /// Selector for rule Indent-Always.
    IndentAlways,
    /// Selector for rule LF-Always.
    LfAlways,
    /// Selector for rule LF-Closing.
    LfClosing,
}

/// An extension trait for the `AutoFormatting` formatter implementation. This formatter
/// accepts different rules for configuring individual needs on auto-formatting. The available
/// rules are described be the `AutoFmtRule` definition.
pub trait ExtAutoIndenting: Formatter {
    /// Adds all given tags to a register for rule selected by a `FixedRule`.
    fn add_tags_to_rule(&mut self, tags: &[&str], rule: AutoFmtRule) -> Result<()>;

    /// Shall reset and empty all registers for fixed rules.
    fn reset_ruleset(&mut self) -> Result<()>;
}
