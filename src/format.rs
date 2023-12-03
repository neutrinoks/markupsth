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

    #[cfg(test)]
    pub(crate) fn teststate(last: TagSequence, next: TagSequence) -> SequenceState {
        SequenceState {
            tag_stack: Vec::new(),
            last,
            next,
            indent: DEFAULT_INDENT,
        }
    }

    // /// Only for testing purposes used internally.
    // #[cfg(test)]
    // pub(crate) fn initial_open(next: &str) -> SequenceState {
    //     Self::teststate(TagSequence::initial(), TagSequence::opening(next))
    // }

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

    // /// Only for testing purposes used internally.
    // #[cfg(test)]
    // pub(crate) fn close_open(last: &str, next: &str) -> SequenceState {
    //     Self::teststate(TagSequence::closing(last), TagSequence::opening(next))
    // }

    // /// Only for testing purposes used internally.
    // #[cfg(test)]
    // pub(crate) fn close_self_closing(last: &str, next: &str) -> SequenceState {
    //     Self::teststate(TagSequence::closing(last), TagSequence::self_closing(next))
    // }

    /// Only for testing purposes used internally.
    #[cfg(test)]
    pub(crate) fn close_close(last: &str, next: &str) -> SequenceState {
        Self::teststate(TagSequence::closing(last), TagSequence::closing(next))
    }

    // /// Only for testing purposes used internally.
    // #[cfg(test)]
    // pub(crate) fn close_lf(last: &str) -> SequenceState {
    //     Self::teststate(TagSequence::closing(last), TagSequence::LINEFEED())
    // }

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

    // /// Only for testing purposes used internally.
    // #[cfg(test)]
    // pub(crate) fn self_closing_open(last: &str, next: &str) -> SequenceState {
    //     Self::teststate(TagSequence::self_closing(last), TagSequence::opening(next))
    // }

    // /// Only for testing purposes used internally.
    // #[cfg(test)]
    // pub(crate) fn self_closing_lf(last: &str) -> SequenceState {
    //     Self::teststate(TagSequence::self_closing(last), TagSequence::LINEFEED())
    // }

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
    fn set_indent_step_size(&mut self, step_size: usize);

    /// Returns the current indenting step size.
    fn get_indent_step_size(&self) -> usize;

    /// Whatever may configurable, this function shall rest to its defaults.
    fn reset_to_defaults(&mut self);

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

        fn reset_to_defaults(&mut self) {}

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

        fn reset_to_defaults(&mut self) {
            self.0 = DEFAULT_INDENT;
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
        /// List for tags, where a LINEFEED shall always be inserted.
        pub fltr_lf_always: Vec<String>,
        /// List for tags, where a LINEFEED shall inserted after closing tags.
        pub fltr_lf_closing: Vec<String>,
        /// Internal, operational, for tracking whether indented or not.
        indent_stack: Vec<bool>,
        /// The indenting step size.
        indent_step: usize,
    }

    // Styles of desire:
    // 1. Always LINEFEEDs but no indenting, e.g. <html>
    // 2. Always content indented, e.g. <body>, <head>
    // 3. Linefeed after the closing tag, e.g. </div>
    #[derive(Copy, Clone, Debug)]
    enum AIFilter {
        IndentAlways,
        LfAlways,
        LfClosing,
    }

    impl AutoIndent {
        /// A default filter setup for HTML auto indenting will be applied.
        pub fn set_filter_default_html(&mut self) {
            self.set_filter_indent_always(&["head", "body", "section", "header", "footer", "nav"])
                .unwrap();
            self.set_filter_lf_always(&["html"]).unwrap();
            self.set_filter_lf_closing(&["title", "link", "div"]).unwrap();
        }

        /// Resets all internal filters for tag to be formatted. See documentation on
        /// `fltr_indent_always`, `fltr_lf_always`, and `fltr_lf_closing`.
        pub fn reset_filters(&mut self) {
            self.fltr_indent_always.clear();
            self.fltr_lf_always.clear();
            self.fltr_lf_closing.clear();
        }

        /// Tag names after which shall always be indented more, can be setup in this filter. For
        /// example, if always shall be indented more after the following tag names: "head",
        /// "body", "section":
        /// ```ignore
        /// let mut formatter = AutoIndent::new();
        /// formatter.set_filter_indent_always(&["head", "body", "section"]);
        /// ```
        /// Default is empty.
        pub fn set_filter_indent_always(&mut self, filter: &[&str]) -> Result<(), String> {
            self.check_other_filter(filter, AIFilter::IndentAlways, AIFilter::LfAlways)?;
            self.fltr_indent_always = filter.iter().map(|s| s.to_string()).collect();
            Ok(())
        }

        /// TODO
        pub fn set_filter_lf_always(&mut self, filter: &[&str]) -> Result<(), String> {
            self.check_other_filter(filter, AIFilter::LfAlways, AIFilter::IndentAlways)?;
            self.check_other_filter(filter, AIFilter::LfAlways, AIFilter::LfClosing)?;
            self.fltr_lf_always = filter.iter().map(|s| s.to_string()).collect();
            Ok(())
        }

        /// TODO
        pub fn set_filter_lf_closing(&mut self, filter: &[&str]) -> Result<(), String> {
            self.check_other_filter(filter, AIFilter::LfClosing, AIFilter::LfAlways)?;
            self.fltr_lf_closing = filter.iter().map(|s| s.to_string()).collect();
            Ok(())
        }

        // Internal method to check if tags are in another filter too.
        fn check_other_filter(
            &self,
            tags: &[&str],
            fltr: AIFilter,
            other: AIFilter,
        ) -> Result<(), String> {
            let errtags: Vec<String> = tags
                .iter()
                .filter(|t| self.is_ts_in_filter(&TagSequence::opening(t), other))
                .map(|t| t.to_string())
                .collect();
            if errtags.is_empty() {
                Ok(())
            } else {
                Err(format!(
                    "{}_({:?}), {} {:?} too: {:?}",
                    "AutoIndent::set_filter",
                    fltr,
                    "trying to add tags which have already been added to filter",
                    other,
                    errtags
                ))
            }
        }

        // Internal check method if tag is contained in field `fltr_indent_always`.
        fn is_ts_in_filter(&self, tagseq: &TagSequence, fltr: AIFilter) -> bool {
            let fltr: &Vec<String> = match fltr {
                AIFilter::IndentAlways => &self.fltr_indent_always,
                AIFilter::LfAlways => &self.fltr_lf_always,
                AIFilter::LfClosing => &self.fltr_lf_closing,
            };
            for tf in fltr.iter() {
                if tf == &tagseq.1 {
                    return true;
                }
            }
            false
        }

        fn is_ts_in_fltr_aot(&self, tagseq: &TagSequence, fltr: AIFilter, seq: Sequence) -> bool {
            if tagseq.0 != seq {
                return false;
            }
            self.is_ts_in_filter(tagseq, fltr)
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

        fn reset_to_defaults(&mut self) {
            self.fltr_indent_always.clear();
            self.fltr_lf_always.clear();
            self.fltr_lf_closing.clear();
            self.indent_step = DEFAULT_INDENT;
        }

        fn check(&mut self, state: &SequenceState) -> FormatChanges {
            let mut changes = FormatChanges::nothing();

            if matches!(state.next.0, Sequence::Closing) {
                // In case of a following closing tag, everything behaves a little different,
                // because of optional less-indenting.
                if matches!(state.last.0, Sequence::Opening)
                    && self.is_ts_in_filter(&state.last, AIFilter::LfAlways)
                {
                    // Detect if lf or not...
                    changes = FormatChanges::lf();
                } else if let Some(true) = self.indent_stack.pop() {
                    // otherwise we can check for a less-indenting on indent_stack!
                    changes = FormatChanges::indent_less(state.indent, self.indent_step);
                } else if self.is_ts_in_fltr_aot(&state.last, AIFilter::IndentAlways, Sequence::Closing)
                    || self.is_ts_in_filter(&state.last, AIFilter::LfAlways)
                    || self.is_ts_in_fltr_aot(&state.last, AIFilter::LfClosing, Sequence::Closing)
                    || self.is_ts_in_fltr_aot(
                        &state.last,
                        AIFilter::LfClosing,
                        Sequence::SelfClosing,
                    )
                {
                    changes = FormatChanges::lf();
                }
            } else {
                match state.last.0 {
                    Sequence::Opening => {
                        // After an opening-tag LINEFEED and optional indenting can be desired
                        // Anyway, for each opening tag we add a flag for indenting on the internal stack.
                        let do_indent = (matches!(state.next.0, Sequence::LineFeed)
                            && !self.is_ts_in_filter(&state.last, AIFilter::LfAlways))
                            || self.is_ts_in_filter(&state.last, AIFilter::IndentAlways);
                        self.indent_stack.push(do_indent);
                        if do_indent {
                            changes = FormatChanges::indent_more(state.indent, self.indent_step);
                        } else if self.is_ts_in_filter(&state.last, AIFilter::LfAlways) {
                            changes = FormatChanges::lf();
                        }
                    }
                    Sequence::Closing => {
                        // After a closing-tag a LINEFEED can be desired
                        if self.is_ts_in_filter(&state.last, AIFilter::IndentAlways)
                            || self.is_ts_in_filter(&state.last, AIFilter::LfAlways)
                            || self.is_ts_in_filter(&state.last, AIFilter::LfClosing)
                        {
                            changes = FormatChanges::lf();
                        }
                    }
                    Sequence::SelfClosing => {
                        if self.is_ts_in_fltr_aot(
                            &state.last,
                            AIFilter::LfClosing,
                            Sequence::SelfClosing,
                        ) {
                            changes = FormatChanges::lf();
                        }
                    }
                    Sequence::Initial => {
                        // If last tag was the initial document sequence, also line feed always!
                        changes = FormatChanges::lf()
                    }
                    _ => {}
                }
            }
            changes
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use totems::assert_err;

        const NOTHING: FormatChanges = FormatChanges {
            new_line: false,
            new_indent: None,
        };
        const LINEFEED: FormatChanges = FormatChanges {
            new_line: true,
            new_indent: None,
        };
        const INDENT_LESS: FormatChanges = FormatChanges {
            new_line: true,
            new_indent: Some(0),
        };
        const INDENT_MORE: FormatChanges = FormatChanges {
            new_line: true,
            new_indent: Some(8),
        };

        fn get_formatters_list() -> Vec<Box<dyn Formatter>> {
            vec![
                Box::new(NoFormatting::new()),
                Box::new(AlwaysIndentAlwaysLf::new()),
                Box::new(AutoIndent::new()),
            ]
        }

        #[test]
        fn all_initially_default() {
            for fmt in get_formatters_list().iter_mut() {
                assert_eq!(fmt.get_indent_step_size(), DEFAULT_INDENT);
            }
        }

        #[test]
        fn after_reset_default_again() {
            for fmt in get_formatters_list().iter_mut() {
                fmt.set_indent_step_size(DEFAULT_INDENT + 1);
                fmt.reset_to_defaults();
                assert_eq!(fmt.get_indent_step_size(), DEFAULT_INDENT);
            }
        }

        // Because opening tags are influencing the AutoIndent's state, consider open tags!!!
        // Meaningful to test in rows of three, e.g. <div><img></div>, <p>text</p>, except for
        // special cases like <div></div>.

        #[test]
        fn auto_indenting_rule_always_indent() {
            let mut fmtr = Box::new(AutoIndent::new());
            fmtr.reset_to_defaults();
            fmtr.set_filter_indent_always(&["html"]).unwrap();

            // Test: Auto-indenting works on non-registered tags with manual LF.
            // <body>\n<img></body>
            assert_eq!(fmtr.check(&SequenceState::open_lf("body")), INDENT_MORE);
            assert_eq!(fmtr.check(&SequenceState::lf_self_closing("img")), NOTHING);
            assert_eq!(
                fmtr.check(&SequenceState::self_closing_close("img", "body")),
                INDENT_LESS
            );

            // Test: Auto-indenting works on registered tags with manual LF.
            // <html>\n<img></html>
            assert_eq!(fmtr.check(&SequenceState::open_lf("html")), INDENT_MORE);
            assert_eq!(fmtr.check(&SequenceState::lf_self_closing("img")), NOTHING);
            assert_eq!(
                fmtr.check(&SequenceState::self_closing_close("img", "html")),
                INDENT_LESS
            );

            // Test: Auto-indenting works on registered tags without input.
            // <html><img></html>
            assert_eq!(
                fmtr.check(&SequenceState::open_self_closing("html", "img")),
                INDENT_MORE
            );
            assert_eq!(
                fmtr.check(&SequenceState::self_closing_close("img", "html")),
                INDENT_LESS
            );

            // Test: Multiple stages with manual and auto-indenting mixed tags.
            assert_eq!(
                fmtr.check(&SequenceState::open_open("html", "body")),
                INDENT_MORE
            );
            assert_eq!(fmtr.check(&SequenceState::open_lf("body")), INDENT_MORE);
            assert_eq!(fmtr.check(&SequenceState::lf_self_closing("img")), NOTHING);
            assert_eq!(
                fmtr.check(&SequenceState::self_closing_close("img", "body")),
                INDENT_LESS
            );
            assert_eq!(
                fmtr.check(&SequenceState::close_close("body", "html")),
                INDENT_LESS
            );

            // ? Error on self-closing tags if added to always-indent ?
        }

        #[test]
        fn auto_indenting_rule_lf_always() {
            let mut fmtr = Box::new(AutoIndent::new());
            fmtr.reset_to_defaults();
            fmtr.set_filter_lf_always(&["html"]).unwrap();

            // Test: Auto-LF after opening-tag and after closing-tag of registered tags.
            // <html><img></html>
            assert_eq!(
                fmtr.check(&SequenceState::open_self_closing("html", "img")),
                LINEFEED
            );
            assert_eq!(
                fmtr.check(&SequenceState::self_closing_close("img", "html")),
                NOTHING
            );
            assert_eq!(fmtr.check(&SequenceState::close_text("html")), LINEFEED);

            // Test: No auto-LF after opening- and closing-tags of non-registered tags.
            // <body><img></body>
            assert_eq!(
                fmtr.check(&SequenceState::open_self_closing("body", "img")),
                NOTHING
            );
            assert_eq!(
                fmtr.check(&SequenceState::self_closing_close("img", "body")),
                NOTHING
            );
            assert_eq!(fmtr.check(&SequenceState::close_text("body")), NOTHING);

            // Test: Auto-indenting after registered tags and opening-tag folowed by LF.
            // <html>\n<img></html>
            assert_eq!(fmtr.check(&SequenceState::open_lf("html")), LINEFEED);
            assert_eq!(fmtr.check(&SequenceState::lf_self_closing("img")), NOTHING);
            assert_eq!(
                fmtr.check(&SequenceState::self_closing_close("img", "html")),
                NOTHING
            );
            assert_eq!(fmtr.check(&SequenceState::close_text("html")), LINEFEED);

            // Test: Auto-indenting after non-registered tags and following LF.
            // <body>\n<img></body>
            assert_eq!(fmtr.check(&SequenceState::open_lf("body")), INDENT_MORE);
            assert_eq!(fmtr.check(&SequenceState::lf_self_closing("img")), NOTHING);
            assert_eq!(
                fmtr.check(&SequenceState::self_closing_close("img", "body")),
                INDENT_LESS
            );
            assert_eq!(fmtr.check(&SequenceState::close_text("body")), NOTHING);

            // ? Error on self-closing tags if added to lf-always ?
        }

        #[test]
        fn auto_indenting_rule_lf_closing() {
            let mut fmtr = Box::new(AutoIndent::new());
            fmtr.reset_to_defaults();
            fmtr.set_filter_lf_closing(&["html", "img"]).unwrap();

            // Test: open-close works without problems, no formatting at all in between.
            // <html></html>
            assert_eq!(
                fmtr.check(&SequenceState::open_close("html", "html")),
                NOTHING
            );

            // Test: Auto-LF after closing block will be inserted.
            // <html>Text</html>
            assert_eq!(fmtr.check(&SequenceState::open_text("html")), NOTHING);
            assert_eq!(fmtr.check(&SequenceState::text_close("html")), NOTHING);
            assert_eq!(fmtr.check(&SequenceState::close_text("html")), LINEFEED);

            // Test: No auto-LF after non-listed tags.
            // <body>Text</body>
            assert_eq!(fmtr.check(&SequenceState::open_text("body")), NOTHING);
            assert_eq!(fmtr.check(&SequenceState::text_close("body")), NOTHING);
            assert_eq!(fmtr.check(&SequenceState::close_text("body")), NOTHING);

            // Test: Auto-indenting on open-lf for non-listed tags.
            // <body>\n<link></body>
            assert_eq!(fmtr.check(&SequenceState::open_lf("body")), INDENT_MORE);
            assert_eq!(fmtr.check(&SequenceState::lf_self_closing("link")), NOTHING);
            assert_eq!(
                fmtr.check(&SequenceState::self_closing_close("link", "body")),
                INDENT_LESS
            );
            assert_eq!(fmtr.check(&SequenceState::close_text("body")), NOTHING);

            // Test: Auto-indenting on open-lf for listed tags.
            // <html>\n<link></html>
            assert_eq!(fmtr.check(&SequenceState::open_lf("html")), INDENT_MORE);
            assert_eq!(fmtr.check(&SequenceState::lf_self_closing("link")), NOTHING);
            assert_eq!(
                fmtr.check(&SequenceState::self_closing_close("link", "html")),
                INDENT_LESS
            );
            assert_eq!(fmtr.check(&SequenceState::close_text("html")), LINEFEED);

            // Test: Auto-LF after listed self-closing tags.
            // <div><img></div>
            assert_eq!(
                fmtr.check(&SequenceState::open_self_closing("div", "img")),
                NOTHING
            );
            assert_eq!(
                fmtr.check(&SequenceState::self_closing_close("img", "div")),
                LINEFEED
            );
            assert_eq!(fmtr.check(&SequenceState::close_text("div")), NOTHING);

            // Test: No auto-LF after non-listed self-clsoing tags.
            // already tested that before two times.
        }

        #[test]
        fn auto_indenting_mixed_rules() {
            let mut fmtr = Box::new(AutoIndent::new());

            // Test some error messages
            fmtr.reset_to_defaults();
            fmtr.set_filter_indent_always(&["html", "body"]).unwrap();
            assert_err!(fmtr.set_filter_lf_always(&["html", "head"]));
            fmtr.set_filter_lf_closing(&["html", "body"]).unwrap();

            fmtr.reset_to_defaults();
            fmtr.set_filter_lf_always(&["html", "body"]).unwrap();
            assert_err!(fmtr.set_filter_indent_always(&["html", "head"]));
            assert_err!(fmtr.set_filter_lf_closing(&["body", "header"]));

            // Now, test setup for the operational test cases.
            fmtr.reset_to_defaults();
            fmtr.set_filter_indent_always(&["head", "body"]).unwrap();
            fmtr.set_filter_lf_always(&["html"]).unwrap();
            fmtr.set_filter_lf_closing(&["body", "div"]).unwrap();

            // Test: Auto-indenting combined with lf-always.
            // <html><head><link></head></html>
            assert_eq!(
                fmtr.check(&SequenceState::open_open("html", "head")),
                LINEFEED
            );
            assert_eq!(
                fmtr.check(&SequenceState::open_self_closing("head", "link")),
                INDENT_MORE
            );
            assert_eq!(
                fmtr.check(&SequenceState::self_closing_close("link", "head")),
                INDENT_LESS
            );
            assert_eq!(
                fmtr.check(&SequenceState::close_close("head", "html")),
                LINEFEED
            );
            assert_eq!(fmtr.check(&SequenceState::close_text("html")), LINEFEED);

            // Test: Auto-indenting works also if lf-closing activated too.
            // <body><div>Text</div></body>
            assert_eq!(
                fmtr.check(&SequenceState::open_open("body", "div")),
                INDENT_MORE
            );
            assert_eq!(fmtr.check(&SequenceState::open_text("div")), NOTHING);
            assert_eq!(fmtr.check(&SequenceState::text_close("div")), NOTHING);
            assert_eq!(
                fmtr.check(&SequenceState::close_close("div", "body")),
                INDENT_LESS
            );
            assert_eq!(fmtr.check(&SequenceState::close_text("body")), LINEFEED);

            // Test: Auto-indenting works also if lf-closing on self-closing tags activated.
            // <body><img></body>
            assert_eq!(
                fmtr.check(&SequenceState::open_self_closing("body", "img")),
                INDENT_MORE
            );
            assert_eq!(
                fmtr.check(&SequenceState::self_closing_close("img", "body")),
                INDENT_LESS
            );
            assert_eq!(fmtr.check(&SequenceState::close_text("body")), LINEFEED);
        }
    }
}
