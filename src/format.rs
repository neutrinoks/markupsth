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
pub struct TagSequence(pub Sequence, pub String);

impl<'s> TagSequence {
    pub fn self_closing(tag: &'s str) -> TagSequence {
        TagSequence(Sequence::SelfClosing, tag.to_string())
    }

    pub fn opening(tag: &'s str) -> TagSequence {
        TagSequence(Sequence::Opening, tag.to_string())
    }

    pub fn closing(tag: &'s str) -> TagSequence {
        TagSequence(Sequence::Closing, tag.to_string())
    }

    pub fn text() -> TagSequence {
        TagSequence(Sequence::Text, String::new())
    }

    pub fn linefeed() -> TagSequence {
        TagSequence(Sequence::LineFeed, String::new())
    }

    pub fn initial() -> TagSequence {
        TagSequence(Sequence::Initial, String::new())
    }

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
    pub fn new() -> SequenceState {
        SequenceState {
            tag_stack: Vec::new(),
            last: TagSequence::initial(),
            next: TagSequence::text(),
            indent: 0,
        }
    }
}

impl Default for SequenceState {
    fn default() -> SequenceState {
        SequenceState::new()
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
    /// New type pattern as default.
    fn new() -> Self
    where
        Self: Sized;

    /// Sets the indenting step size, pre-defined default is 4.
    fn set_indent_step_size(&mut self, step_size: usize);

    /// Getter method for indenting step izse.
    fn get_indent_step_size(&self) -> usize;

    /// Check for format changes between last und next `TagSequence`. Neccessary changes regarding
    /// indenting and/or line-feed will be returned by a `FormatChanges`.
    fn check(&mut self, state: &SequenceState) -> FormatChanges;
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
        fn set_indent_step_size(&mut self, _: usize) {}

        /// See trait description.
        fn get_indent_step_size(&self) -> usize {
            0
        }

        /// See trait description.
        fn check(&mut self, _: &SequenceState) -> FormatChanges {
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
        fn set_indent_step_size(&mut self, step_size: usize) {
            self.0 = step_size;
        }

        /// See trait description.
        fn get_indent_step_size(&self) -> usize {
            self.0
        }

        /// See trait description.
        fn check(&mut self, state: &SequenceState) -> FormatChanges {
            if matches!(state.next.0, Sequence::Closing) {
                match state.last.0 {
                    Sequence::Opening => FormatChanges::may_lf(true),
                    _ => FormatChanges::indent_less(state.indent, self.0),
                }
            } else {
                match state.last.0 {
                    Sequence::Initial => FormatChanges::may_lf(true),
                    Sequence::Opening => match state.next.0 {
                        Sequence::SelfClosing | Sequence::Text | Sequence::Opening => {
                            FormatChanges::indent_more(state.indent, self.0)
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

    /// TODO: Decription
    #[derive(Debug)]
    pub struct AutoIndent {
        indent_filter: Vec<String>,
        indent_stack: Vec<bool>,
        indent_step: usize,
    }

    impl AutoIndent {
        /// Tag names after which shall always be indented more, can be setup in this filter. For
        /// example, if always shall be indented more after the following tag names: "head",
        /// "body", "section":
        /// ```ignore
        /// let mut formatter = AutoIndent::new();
        /// formatter.set_always_filter(&["head", "body", "section"]);
        /// ```
        /// Default is empty.
        pub fn set_always_filter(&mut self, filter: &[&str]) {
            self.indent_filter = filter.iter().map(|s| s.to_string()).collect();
        }

        // Internal check method if tag is contained in field `indent_filter`.
        fn is_indent_filter(&self, tag: &str) -> bool {
            for tf in self.indent_filter.iter() {
                if tf == tag {
                    return true;
                }
            }
            false
        }
    }

    impl Formatter for AutoIndent {
        /// See trait description.
        fn new() -> AutoIndent {
            AutoIndent {
                indent_filter: Vec::new(),
                indent_stack: Vec::new(),
                indent_step: DEFAULT_INDENT,
            }
        }

        /// See trait description.
        fn set_indent_step_size(&mut self, step_size: usize) {
            self.indent_step = step_size;
        }

        /// See trait description.
        fn get_indent_step_size(&self) -> usize {
            self.indent_step
        }

        /// See trait description.
        fn check(&mut self, state: &SequenceState) -> FormatChanges {
            // Rules:
            // 1. If opening and line-feed, do indent
            // 2. If opening and always-filter, do indent
            // 3. If closing and was indented, indent-back
            // 4. if closing and always-filter, do lf
            // 5. After initial, lf

            let mut changes = FormatChanges::nothing();
            // If last one was an opening-tag, it definitely has to added an entry on stack_indent.
            // Separate checking, because it is too complicated to depict it on each possible case.
            if matches!(state.last.0, Sequence::Opening) {
                let do_lf = matches!(state.next.0, Sequence::LineFeed)
                    || self.is_indent_filter(&state.last.1);
                self.indent_stack.push(do_lf);
                if do_lf {
                    changes = FormatChanges::indent_more(state.indent, self.indent_step);
                }
            } else if matches!(state.next.0, Sequence::Closing) {
                // Check next sequence and return FormatChanges
                if let Some(true) = self.indent_stack.pop() {
                    changes = FormatChanges::indent_less(state.indent, self.indent_step);
                }
            } else if matches!(state.last.0, Sequence::Closing) {
                if self.is_indent_filter(&state.last.1) {
                    changes = FormatChanges::lf();
                }
            } else if matches!(state.last.0, Sequence::Initial) {
                // If last tag was the initial document sequence, also line feed always!
                changes = FormatChanges::lf()
            }
            changes
        }
    }
}
