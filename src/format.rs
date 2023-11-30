//! This module implements optional tag pattern related auto-formattings, respectively
//! auto-line-feed and auto-indenting. Because Markup languages usually have tag pair elements,
//! e.g. in HTML `<head></head>`, we can specify if we want auto indenting between them. Also we
//! could define the rule to insert a line feed automatically after each `<img>` tag.
//!
//! This module contains all implementations and definitions to setup the formatting in
//! `MarkupSth`. Available options are currently indenting-step-size, auto-line-feed, and
//! auto-indenting.

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
}

impl Default for SequenceState {
    fn default() -> SequenceState {
        SequenceState::new()
    }
}

/// Possible changes regarding the current format between two following sequences, will be
/// described by this definition. This can be a linefeed to be inserted with or without changes in
/// current indenting. A change on current indenting will either be increased or decreased by the
/// indenting step size, which can be adjusted in any implementation of `Formatter`.
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
    /// New type pattern as default for constructing any kind of `Formatter`. The crate's default
    /// indenting step size `DEFAULT_INDENT` shall be set after calling this method.
    fn new() -> Self
    where
        Self: Sized;

    /// Modify and set the indenting step size. Default is `DEFAULT_INDENT`.
    fn set_indent_step_size(&mut self, step_size: usize);

    /// Returns the current indenting step size.
    fn get_indent_step_size(&self) -> usize;

    /// Checks for format changes between the last and the next `TagSequence`. Neccessary changes
    /// regarding indenting and/or line-feed will be returned as `FormatChanges`. See description
    /// for more informations.
    fn check(&mut self, state: &SequenceState) -> FormatChanges;
}

pub mod generic {
    use super::*;

    /// A pre-defined `Formatter`. TODO
    #[derive(Debug)]
    pub struct NoFormatting;

    impl Formatter for NoFormatting {
        fn new() -> NoFormatting {
            NoFormatting
        }

        fn set_indent_step_size(&mut self, _: usize) {}

        fn get_indent_step_size(&self) -> usize {
            DEFAULT_INDENT
        }

        fn check(&mut self, _: &SequenceState) -> FormatChanges {
            FormatChanges::nothing()
        }
    }

    /// A pre-defined `Formatter`. TODO
    #[derive(Debug)]
    pub struct AlwaysIndentAlwaysLf(usize);

    impl Formatter for AlwaysIndentAlwaysLf {
        fn new() -> AlwaysIndentAlwaysLf {
            AlwaysIndentAlwaysLf(DEFAULT_INDENT)
        }

        fn set_indent_step_size(&mut self, step_size: usize) {
            self.0 = step_size;
        }

        fn get_indent_step_size(&self) -> usize {
            self.0
        }

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
        /// List for tags, where content has always to be indented.
        pub fltr_indent_always: Vec<String>,
        /// List for tags, where a linefeed shall always be inserted.
        pub fltr_lf_always: Vec<String>,
        /// List for tags, where a linefeed shall inserted after closing tags.
        pub fltr_lf_closing: Vec<String>,
        /// Internal, operational, for tracking whether indented or not.
        indent_stack: Vec<bool>,
        /// The indenting step size.
        indent_step: usize,
    }

    // Styles of desire:
    // 1. Always linefeeds but no indenting, e.g. <html>
    // 2. Always content indented, e.g. <body>, <head>
    // 3. Linefeed after the closing tag, e.g. </div>
    enum AIFilter {
        IndentAlways,
        LfAlways,
        LfClosing,
    }

    impl AutoIndent {
        /// TODO
        pub fn set_filter_default_html(&mut self) {
            self.set_filter_indent_always(&["head", "body", "section", "header", "footer", "nav"]);
            self.set_filter_lf_always(&["html"]);
            self.set_filter_lf_closing(&["div"]);
        }

        /// Tag names after which shall always be indented more, can be setup in this filter. For
        /// example, if always shall be indented more after the following tag names: "head",
        /// "body", "section":
        /// ```ignore
        /// let mut formatter = AutoIndent::new();
        /// formatter.set_filter_indent_always(&["head", "body", "section"]);
        /// ```
        /// Default is empty.
        pub fn set_filter_indent_always(&mut self, filter: &[&str]) {
            self.fltr_indent_always = filter.iter().map(|s| s.to_string()).collect();
        }

        /// TODO
        pub fn set_filter_lf_always(&mut self, filter: &[&str]) {
            self.fltr_lf_always = filter.iter().map(|s| s.to_string()).collect();
        }

        /// TODO
        pub fn set_filter_lf_closing(&mut self, filter: &[&str]) {
            self.fltr_lf_closing = filter.iter().map(|s| s.to_string()).collect();
        }

        // Internal check method if tag is contained in field `fltr_indent_always`.
        fn is_tag_in_fltr(&self, tag: &str, fltr: AIFilter) -> bool {
            let fltr: &Vec<String> = match fltr {
                AIFilter::IndentAlways => &self.fltr_indent_always,
                AIFilter::LfAlways => &self.fltr_lf_always,
                AIFilter::LfClosing => &self.fltr_lf_closing,
            };
            for tf in fltr.iter() {
                if tf == tag {
                    return true;
                }
            }
            false
        }
    }

    impl Formatter for AutoIndent {
        fn new() -> AutoIndent {
            AutoIndent {
                fltr_indent_always: Vec::new(),
                fltr_lf_always: Vec::new(),
                fltr_lf_closing: Vec::new(),
                indent_stack: Vec::new(),
                indent_step: DEFAULT_INDENT,
            }
        }

        fn set_indent_step_size(&mut self, step_size: usize) {
            self.indent_step = step_size;
        }

        fn get_indent_step_size(&self) -> usize {
            self.indent_step
        }

        fn check(&mut self, state: &SequenceState) -> FormatChanges {
            let mut changes = FormatChanges::nothing();

            if matches!(state.next.0, Sequence::Closing) {
                // Before a closing-tag we have to check for less-indenting and linefeed.
                if let Some(true) = self.indent_stack.pop() {
                    changes = FormatChanges::indent_less(state.indent, self.indent_step);
                }
            } else if matches!(state.last.0, Sequence::Opening) {
                // After an opening-tag linefeed and optional indenting can be desired
                // Anyway, for each opening tag we add a flag for indenting on the internal stack.
                let do_indent = (matches!(state.next.0, Sequence::LineFeed)
                    && !self.is_tag_in_fltr(&state.last.1, AIFilter::LfAlways))
                    || self.is_tag_in_fltr(&state.last.1, AIFilter::IndentAlways);
                self.indent_stack.push(do_indent);
                if do_indent {
                    changes = FormatChanges::indent_more(state.indent, self.indent_step);
                } else if self.is_tag_in_fltr(&state.last.1, AIFilter::LfAlways) {
                    changes = FormatChanges::lf();
                }
            } else if matches!(state.last.0, Sequence::Closing) {
                // After a closing-tag a linefeed can be desired
                if self.is_tag_in_fltr(&state.last.1, AIFilter::LfAlways)
                    || self.is_tag_in_fltr(&state.last.1, AIFilter::LfClosing)
                {
                    changes = FormatChanges::lf();
                }
            } else if matches!(state.last.0, Sequence::Initial) {
                // If last tag was the initial document sequence, also line feed always!
                changes = FormatChanges::lf()
            }
            changes
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::{Formatter, Language, MarkupSth};

        #[test]
        fn all_initially_default() {
            let fmtrs: Vec<Box<dyn Formatter>> = vec![
                Box::new(NoFormatting::new()),
                Box::new(AlwaysIndentAlwaysLf::new()),
                Box::new(AutoIndent::new()),
            ];
            for fmt in fmtrs.iter() {
                assert_eq!(fmt.get_indent_step_size(), DEFAULT_INDENT);
            }
        }

        #[test]
        fn auto_indenting_rule_always_indent() {
            let mut doc = String::new();
            let mut mls = MarkupSth::new(&mut doc, Language::Html);
            // Define test cases first
        }

        #[test]
        fn auto_indenting_rule_lf_always() {
            todo!();
        }

        #[test]
        fn auto_indenting_rule_lf_closing() {
            todo!();
        }
    }
}
