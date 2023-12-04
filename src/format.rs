//! Contains implementations to model a tag-sequence, changes on format between two following
//! sequences, any kind of formatter, the state of an arbitrary formatter, or optional features of
//! those formatters to be used in MarkupSth.
//!

use crate::Result;

/// Crate default and initial indenting step size. Can be overwritten by trait methods.
pub const DEFAULT_INDENT: usize = 4;

/// Internal state of the MarkupSth for tracking operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Sequence {
    /// The document's headline, e.g. `<!DOCTYPE html>`.
    Initial,
    /// A self-clsoing tag, e.g. `<img href="image.jpg">`.
    SelfClosing,
    /// An opening tag, e.g. `<section>`.
    Opening,
    /// A closing tag, e.g. `</section>`.
    Closing,
    /// Regular text content, not related to text and usually no influence on formatting.
    Text,
    /// Linefeed inserted manually by a `MarkupSth`-user. Important to know for auto-indenting.
    LineFeed,
}

/// A certain sequence in ML-generation, that means a 'Sequence' with a tag identifier.
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
    /// New type pattern for a default `SequenceState`.
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

/// Possible changes regarding the current format between two following sequences, will be
/// described by this definition. This can be a LINEFEED to be inserted with or without changes in
/// current indenting. A change on current indenting will either be increased or decreased by the
/// indenting step size, which can be adjusted in any implementation of `Formatter`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormatChanges {
    /// Shall a new line (line feed) be inserted after inserting certain tag elements.
    pub new_line: bool,
    /// Need indenting to be updated, and if yes, what is the new indenting and has it to be
    /// applied before inserting a tag element. For example in case of closing tag elements the
    /// indenting may be changed before.
    pub new_indent: Option<usize>,
}

impl FormatChanges {
    /// In case NOTHING shall be changed. No line indenting changes, and no line-feed will be
    /// inserted.
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

/// The `Formatter` trait is markupsth's default interface for formatter-implementations. A
/// `Formatter` will check live while generating Markup content, whether the current input shall be
/// indented or a line feed shall be inserted. There will be a couple of pre-defined
/// implementations, e.g. `AutoIndent` or `AlwaysIndentAlwaysLf` or `NoFormatting`.
pub trait Formatter: std::fmt::Debug {
    /// New type pattern as default for constructing any kind of `Formatter`. The crate's default
    /// indenting step size `DEFAULT_INDENT` shall be set after calling this method.
    fn new() -> Self
    where
        Self: Sized;

    /// Modify and set the indenting step size. Default is `DEFAULT_INDENT`.
    fn set_indent_step_size(&mut self, _step_size: usize) {}

    /// Returns the current indenting step size.
    fn get_indent_step_size(&self) -> usize {
        DEFAULT_INDENT
    }

    /// TODO
    fn optional_fixed_ruleset(&mut self) -> Option<&mut dyn FixedRuleset> {
        None
    }

    /// Whatever may configurable, this function shall rest to its defaults.
    fn reset_to_defaults(&mut self) {}

    /// Checks for format changes between the last and the next `TagSequence`. Neccessary changes
    /// regarding indenting and/or line-feed will be returned as `FormatChanges`. See description
    /// for more informations.
    fn check(&mut self, state: &SequenceState) -> FormatChanges;
}

/// TODO
pub trait FixedRuleset: Formatter {
    /// TODO
    /// Tag names after which shall always be indented more, can be setup in this filter. For
    /// example, if always shall be indented more after the following tag names: "head",
    /// "body", "section":
    /// ```ignore
    /// let mut formatter = AutoIndent::new();
    /// formatter.set_filter_indent_always(&["head", "body", "section"]);
    /// ```
    /// Default is empty.
    fn set_filter_indent_always(&mut self, _fltr: &[&str]) -> Result<()>;

    /// TODO
    fn set_filter_lf_always(&mut self, _fltr: &[&str]) -> Result<()>;

    /// TODO
    fn set_filter_lf_closing(&mut self, _fltr: &[&str]) -> Result<()>;
}
