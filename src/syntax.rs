//! This module implements the syntax configuration of any kind of Markup Language within markupsth.
//! There are currently pre-defined configurations for HTML and XML available, but also the
//! possibility to individually define your own configuration for another ML.
//!
//! ### Examples for available configurations
//!
//! To use the pre-defined configuration for HTML, pass `Language::Html` when creating the
//! `MarkupSth` struct:
//!    ```
//!    use markupsth::{MarkupSth, Language};
//!
//!    let mut document = String::new();
//!    let mut markupsth = MarkupSth::new(&mut document, Language::Html).unwrap();
//!    ```
//!
//! ### Example for defining your own configuration
//!
//! To use an individual configuration for another ML, pass the fully defined `Config` struct via
//! `Language::Other(cfg)`:
//!    ```
//!    use markupsth::{MarkupSth, Language};
//!    use markupsth::syntax::{SyntaxConfig, Insertion::*, TagPairConfig, SelfClosingTagConfig};
//!
//!    let cfg = SyntaxConfig {
//!        doctype: None,
//!        self_closing: Some(SelfClosingTagConfig {
//!            before: Single('|'),
//!            after: Single('|'),
//!        }),
//!        tag_pairs: Some(TagPairConfig {
//!            opener_before: Single('|'),
//!            opener_after: Single('|'),
//!            closer_before: Double('|', '!'),
//!            closer_after: Single('|'),
//!        }),
//!        properties: None,
//!    };
//!
//!    let mut document = String::new();
//!    let mut markupsth = MarkupSth::new(&mut document, Language::Other(cfg)).unwrap();
//!    ```

use std::fmt;
use Insertion::*;

/// Possible type auf automatic insertions. There can be nothing to be inserted, a single or
/// multiple characters. For example in HTML and XML, every tag will be openend by a single
/// character `<` and closed by either a single character `>` or maybe by two `/>`. This different
/// setups can be defined this enumeration type.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Insertion {
    /// No character.
    Nothing,
    /// A single character.
    Single(char),
    /// Two characters.
    Double(char, char),
    /// Three characters.
    Triple(char, char, char),
}

impl fmt::Display for Insertion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        match self {
            Nothing => Ok(()),
            Single(c) => write!(f, "{}", c),
            Double(c1, c2) => write!(f, "{}{}", c1, c2),
            Triple(c1, c2, c3) => write!(f, "{}{}{}", c1, c2, c3),
        }
    }
}

/// Configuration of a self-closing tag element, e.g. in HTML `<img>`.
#[derive(Clone, Debug)]
pub struct SelfClosingTagConfig {
    /// Optional character to be set before a single tag name (opening character).
    pub before: Insertion,
    /// Optional character to be set after a single tag (closing character).
    pub after: Insertion,
}

/// Configuration of a tag-pair element, e.g. HTML `<p></p>`.
#[derive(Clone, Debug)]
pub struct TagPairConfig {
    /// Insertion before the opening tag element identifier.
    pub opener_before: Insertion,
    /// Inseration after the opening tag element.
    pub opener_after: Insertion,
    /// Inseration before the closing tag element identifier.
    pub closer_before: Insertion,
    /// Insertion after the closing tag element.
    pub closer_after: Insertion,
}

/// Configuration for additional properties of tag elements.
#[derive(Clone, Debug)]
pub struct PropertyConfig {
    /// Initiator, character to be inserted between a tag identifier and the first property.
    pub initiator: Insertion,
    /// Insertions before a property name identifier.
    pub name_before: Insertion,
    /// Insertions after a property name identifier.
    pub name_after: Insertion,
    /// Insertions before a property value.
    pub value_before: Insertion,
    /// Insertions after a property value.
    pub value_after: Insertion,
    /// Seperator between property name and property value.
    pub name_separator: Insertion,
    /// Separator between multiple properties.
    pub value_separator: Insertion,
}

/// Main configuration struct for a full syntax configuration of `MarkupSth`.
#[derive(Clone, Debug)]
pub struct SyntaxConfig {
    /// Some optional pre-definitions, e.g. "<?xml version="1.0" encoding="UTF-8"?>.
    pub doctype: Option<String>,
    /// Configuration for self-closing tag elements. When set to `None`, it means there are no tag
    /// pairs available in the Markup language.
    pub self_closing: Option<SelfClosingTagConfig>,
    /// Configuration for tag pair elements. When set to `None`, it means there are no tag pairs
    /// available in the Markup language.
    pub tag_pairs: Option<TagPairConfig>,
    /// Configuration of properties of tag elements. When set to `None`, it means there are no tag
    /// properties available in the Markup language.
    pub properties: Option<PropertyConfig>,
}

/// Selector for available pre-defined syntax configurations and wrapper for passing your own.
#[derive(Clone, Debug)]
pub enum Language {
    Html,
    Xml,
    Other(SyntaxConfig),
}

impl From<Language> for SyntaxConfig {
    fn from(cfg_sel: Language) -> SyntaxConfig {
        match cfg_sel {
            Language::Html => SyntaxConfig {
                doctype: Some(r#"<!DOCTYPE html>"#.to_string()),
                self_closing: Some(SelfClosingTagConfig {
                    before: Single('<'),
                    after: Single('>'),
                }),
                tag_pairs: Some(TagPairConfig {
                    opener_before: Single('<'),
                    opener_after: Single('>'),
                    closer_before: Double('<', '/'),
                    closer_after: Single('>'),
                }),
                properties: Some(PropertyConfig {
                    initiator: Single(' '),
                    name_before: Nothing,
                    name_after: Nothing,
                    value_before: Single('\"'),
                    value_after: Single('\"'),
                    name_separator: Single('='),
                    value_separator: Single(' '),
                }),
            },
            Language::Xml => SyntaxConfig {
                doctype: Some(r#"<?xml version="1.0" encoding="UTF-8"?>"#.to_string()),
                self_closing: Some(SelfClosingTagConfig {
                    before: Single('<'),
                    after: Triple(' ', '/', '>'),
                }),
                tag_pairs: Some(TagPairConfig {
                    opener_before: Single('<'),
                    opener_after: Single('>'),
                    closer_before: Double('<', '/'),
                    closer_after: Single('>'),
                }),
                properties: Some(PropertyConfig {
                    initiator: Single(' '),
                    name_before: Nothing,
                    name_after: Nothing,
                    value_before: Single('\"'),
                    value_after: Single('\"'),
                    name_separator: Single('='),
                    value_separator: Single(' '),
                }),
            },
            Language::Other(cfg) => cfg,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_selector_smoke_test() {
        let _ = SyntaxConfig::from(Language::Html);
        let cfg = SyntaxConfig::from(Language::Xml);
        let _ = SyntaxConfig::from(Language::Other(cfg));
    }

    #[test]
    fn insertion_to_string() {
        assert_eq!(Nothing.to_string(), "".to_string());
        assert_eq!(Single('<').to_string(), "<".to_string());
        assert_eq!(Double('/', '>').to_string(), "/>".to_string());
        assert_eq!(Triple(' ', '/', '>').to_string(), " />".to_string());
    }
}
