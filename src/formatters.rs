//! Module contains the pre-implemented formatters of this crate. Suche as the `NoFormatting`, the
//! `AlwaysIndentAlwaysLf`, and the `AutoIndent`, which are ready to be used without any effort.
//!
//! All of the three pre-implemented formatters implement for sure the trait `Formatter`. Formatter
//! `AutoIndent` implements the additional feature trait `FixedRuleset` as well. Have a look at
//! their further documentation to get an overview.
//!
//! To overwrite the default formatter apply:
//! ```
//! use markupsth::{Formatter, Language, MarkupSth, NoFormatting};
//! // use markupsth::{AlwaysIndentAlwaysLf, Formatter, Language, MarkupSth};
//!
//! let mut doc = String::new();
//! let mut mus = MarkupSth::new(&mut doc, Language::Html).unwrap();
//!
//! mus.set_formatter(Box::new(NoFormatting::new()));
//! // mus.set_formatter(Box::new(AlwaysIndentAlwaysLf::new()));
//! ```
//!
//! To modify the default `AutoIndent` formatter use either the trait methods from trait
//! `Formatter` or `FixedRuleset` defined in module `format`.
//!
//! ### `NoFormatting`
//!
//! A pre-implemented formatter for havin a strict indenting and always linefeeds between tags.
//!
//! You want to have the clearest readable Markup file you can imagine, then this formatter is
//! yours. Output files may be suitable for debugging and error search, but maybe too pendantic.
//!
//! ### `AlwaysIndentAlwaysLf`
//!
//! A pre-implemented formatter for havin a strict indenting and always linefeeds between tags.
//!
//! You want to have the clearest readable Markup file you can imagine, then this formatter is
//! yours. Output files may be suitable for debugging and error search, but maybe too pendantic.
//!
//! ### `AutoIndent`
//!
//! A pre-implemented formatter which applies the fixed ruleset and auto-detects additional
//! indenting.
//!
//! This formatter applies the three fixed rules described in the Fixed Ruleset in module `format`,
//! but does also detect indenting-changes, when adding manual linefeeds. In general this formatter
//! tries to hit a very good and readable output, especially HTML and XML files.
//!
//! Inserting a linefeed by using `MarkupSth::new_line()` after an opening tag, will cause an
//! increase on indenting. It will automatically decreased when closing the pair by inserting the
//! closing tag. Tags who have been added to the ruleset will be treated by rule without any
//! further input.
//!
//! Depending on your personal taste, a good setup for HTML might be:
//! ```
//! # use markupsth::{FixedRule, FixedRuleset, Language, MarkupSth};
//! # let mut doc = String::new();
//! # let mut mus = MarkupSth::new(&mut doc, Language::Html).unwrap();
//! # let fmtr = mus.formatter.optional_fixed_ruleset().unwrap();
//! fmtr.add_tags_to_rule(
//!     &["head", "body", "header", "nav", "section", "footer"],
//!     FixedRule::IndentAlways
//!     ).unwrap();
//! fmtr.add_tags_to_rule(
//!     &["html"],
//!     FixedRule::LfAlways
//!     ).unwrap();
//! fmtr.add_tags_to_rule(
//!     &["p", "div", "link"],
//!     FixedRule::LfClosing
//!     ).unwrap();
//! ```

use crate::{format::*, Result};

/// A pre-implemented formatter for having no formatting at all. No linefeeds, no indenting at all.
///
/// You want no linefeeds, no indenting at all, this is your formatter! Suitable use cases may be
/// to generate a pure HTML file, which will only read by browsers for pure optimization.
#[derive(Debug)]
pub struct NoFormatting;

impl Formatter for NoFormatting {
    fn new() -> NoFormatting {
        NoFormatting
    }

    fn check(&mut self, _: &SequenceState) -> FormatChanges {
        FormatChanges::nothing()
    }
}

/// A pre-implemented formatter for havin a strict indenting and always linefeeds between tags.
///
/// You want to have the clearest readable Markup file you can imagine, then this formatter is
/// yours. Output files may be suitable for debugging and error search, but maybe too pendantic.
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

/// A pre-implemented formatter which applies the fixed ruleset and auto-detects additional
/// indenting.
///
/// This formatter applies the three fixed rules described in the Fixed Ruleset in module `format`,
/// but does also detect indenting-changes, when adding manual linefeeds. In general this formatter
/// tries to hit a very good and readable output, especially HTML and XML files.
///
/// Inserting a linefeed by using `MarkupSth::new_line()` after an opening tag, will cause an
/// increase on indenting. It will automatically decreased when closing the pair by inserting the
/// closing tag. Tags who have been added to the ruleset will be treated by rule without any
/// further input.
///
/// Depending on your personal taste, a good setup for HTML might be:
/// ```
/// # use markupsth::{FixedRule, Language, MarkupSth};
/// # let mut doc = String::new();
/// # let mut mus = MarkupSth::new(&mut doc, Language::Html).unwrap();
/// # let fmtr = mus.formatter.optional_fixed_ruleset().unwrap();
/// fmtr.add_tags_to_rule(
///     &["head", "body", "header", "nav", "section", "footer"],
///     FixedRule::IndentAlways
///     ).unwrap();
/// fmtr.add_tags_to_rule(
///     &["html"],
///     FixedRule::LfAlways
///     ).unwrap();
/// fmtr.add_tags_to_rule(
///     &["p", "div", "link"],
///     FixedRule::LfClosing
///     ).unwrap();
/// ```
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

impl AutoIndent {
    // Internal method to check if tags are in another filter too.
    fn check_other_filter(&self, tags: &[&str], fltr: FixedRule, other: FixedRule) -> Result<()> {
        let errtags: Vec<String> = tags
            .iter()
            .filter(|t| self.is_ts_in_filter(&TagSequence::opening(t), other))
            .map(|t| t.to_string())
            .collect();
        if errtags.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "{}({:?}), {} {:?} too: {:?}",
                "AutoIndent::add_tags_to_rule",
                fltr,
                "trying to add tags which have already been added to filter",
                other,
                errtags
            )
            .into())
        }
    }

    /// Internal check method, if tag is contained in filter `fltr`.
    fn is_ts_in_filter(&self, tagseq: &TagSequence, fltr: FixedRule) -> bool {
        let fltr: &Vec<String> = match fltr {
            FixedRule::IndentAlways => &self.fltr_indent_always,
            FixedRule::LfAlways => &self.fltr_lf_always,
            FixedRule::LfClosing => &self.fltr_lf_closing,
        };
        for tf in fltr.iter() {
            if tf == &tagseq.1 {
                return true;
            }
        }
        false
    }

    /// Internal check method, if tag is contained in filter `fltr` and of type `seq`.
    fn is_ts_in_fltr_aot(&self, tagseq: &TagSequence, fltr: FixedRule, seq: Sequence) -> bool {
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

    fn optional_fixed_ruleset(&mut self) -> Option<&mut dyn FixedRuleset> {
        Some(self)
    }

    fn check(&mut self, state: &SequenceState) -> FormatChanges {
        let mut changes = FormatChanges::nothing();

        if matches!(state.next.0, Sequence::Closing) {
            // In case of a following closing tag, everything behaves a little different,
            // because of optional less-indenting.
            if matches!(state.last.0, Sequence::Opening)
                && self.is_ts_in_filter(&state.last, FixedRule::LfAlways)
            {
                // Detect if lf or not...
                changes = FormatChanges::lf();
            } else if let Some(true) = self.indent_stack.pop() {
                // otherwise we can check for a less-indenting on indent_stack!
                changes = FormatChanges::indent_less(state.indent, self.indent_step);
            } else if self.is_ts_in_fltr_aot(
                &state.last,
                FixedRule::IndentAlways,
                Sequence::Closing,
            ) || self.is_ts_in_filter(&state.last, FixedRule::LfAlways)
                || self.is_ts_in_fltr_aot(&state.last, FixedRule::LfClosing, Sequence::Closing)
                || self.is_ts_in_fltr_aot(&state.last, FixedRule::LfClosing, Sequence::SelfClosing)
            {
                changes = FormatChanges::lf();
            }
        } else {
            match state.last.0 {
                Sequence::Opening => {
                    // After an opening-tag LINEFEED and optional indenting can be desired
                    // Anyway, for each opening tag we add a flag for indenting on the internal stack.
                    let do_indent = (matches!(state.next.0, Sequence::LineFeed)
                        && !self.is_ts_in_filter(&state.last, FixedRule::LfAlways))
                        || self.is_ts_in_filter(&state.last, FixedRule::IndentAlways);
                    self.indent_stack.push(do_indent);
                    if do_indent {
                        changes = FormatChanges::indent_more(state.indent, self.indent_step);
                    } else if self.is_ts_in_filter(&state.last, FixedRule::LfAlways) {
                        changes = FormatChanges::lf();
                    }
                }
                Sequence::Closing => {
                    // After a closing-tag a LINEFEED can be desired
                    if self.is_ts_in_filter(&state.last, FixedRule::IndentAlways)
                        || self.is_ts_in_filter(&state.last, FixedRule::LfAlways)
                        || self.is_ts_in_filter(&state.last, FixedRule::LfClosing)
                    {
                        changes = FormatChanges::lf();
                    }
                }
                Sequence::SelfClosing => {
                    if self.is_ts_in_fltr_aot(
                        &state.last,
                        FixedRule::LfClosing,
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

impl FixedRuleset for AutoIndent {
    fn add_tags_to_rule(&mut self, tags: &[&str], rule: FixedRule) -> Result<()> {
        match rule {
            FixedRule::IndentAlways => {
                self.check_other_filter(tags, FixedRule::IndentAlways, FixedRule::LfAlways)?;
                self.fltr_indent_always = tags.iter().map(|s| s.to_string()).collect();
            }
            FixedRule::LfAlways => {
                self.check_other_filter(tags, FixedRule::LfAlways, FixedRule::IndentAlways)?;
                self.check_other_filter(tags, FixedRule::LfAlways, FixedRule::LfClosing)?;
                self.fltr_lf_always = tags.iter().map(|s| s.to_string()).collect();
            }
            FixedRule::LfClosing => {
                self.check_other_filter(tags, FixedRule::LfClosing, FixedRule::LfAlways)?;
                self.fltr_lf_closing = tags.iter().map(|s| s.to_string()).collect();
            }
        }
        Ok(())
    }

    fn reset_ruleset(&mut self) -> Result<()> {
        self.fltr_indent_always.clear();
        self.fltr_lf_always.clear();
        self.fltr_lf_closing.clear();
        Ok(())
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
    fn auto_indenting_after_initial() {
        let mut fmtr = Box::new(AutoIndent::new());
        assert_eq!(fmtr.check(&SequenceState::initial_open("html")), LINEFEED);
    }

    #[test]
    fn auto_indenting_rule_always_indent() {
        let mut fmtr = Box::new(AutoIndent::new());
        fmtr.add_tags_to_rule(&["html"], FixedRule::IndentAlways)
            .unwrap();

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
        fmtr.add_tags_to_rule(&["html"], FixedRule::LfAlways)
            .unwrap();

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
        fmtr.add_tags_to_rule(&["html", "img"], FixedRule::LfClosing)
            .unwrap();

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
        fmtr.add_tags_to_rule(&["html", "body"], FixedRule::IndentAlways)
            .unwrap();
        assert_err!(fmtr.add_tags_to_rule(&["html", "head"], FixedRule::LfAlways));
        fmtr.add_tags_to_rule(&["html", "body"], FixedRule::LfClosing)
            .unwrap();
        fmtr.reset_to_defaults();

        fmtr.add_tags_to_rule(&["html", "body"], FixedRule::LfAlways)
            .unwrap();
        assert_err!(fmtr.add_tags_to_rule(&["html", "head"], FixedRule::IndentAlways));
        assert_err!(fmtr.add_tags_to_rule(&["body", "header"], FixedRule::LfClosing));
        fmtr.reset_to_defaults();

        // Now, test setup for the operational test cases.
        fmtr.add_tags_to_rule(&["head", "body"], FixedRule::IndentAlways)
            .unwrap();
        fmtr.add_tags_to_rule(&["html"], FixedRule::LfAlways)
            .unwrap();
        fmtr.add_tags_to_rule(&["body", "div"], FixedRule::LfClosing)
            .unwrap();

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
