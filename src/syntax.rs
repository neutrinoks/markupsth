//! This module implements the syntax configuration of any kind of Markup Language within this
//! crate. There are currently pre-defined configurations for HTML and XML available, but also the
//! possibility to individually define your own configuration for another ML.
//!
//! ### Concept
//!
//! It is assumed, that any kind of Markup Language has tag pairs and optional self-closing tag
//! elements. For example in HTML, we have pair elements such as `<html></html>` and self-closing
//! ones such as `<img>` (usually `<img/>`, but the whole web is still writing the old style...).
//! MarkupSth assumes these three type of tags as basic syntax to be configured.
//!
//! It is assumed, that any Markup Language can be defined by a couple of single character
//! insertions before or after a tag identifier/name. For example, in HTML `<html>` we have only
//! one insertion `<` and one insertion `>` after the identifier `html`. Those simple modifications
//! on a single tag can be described by definition `Insertion`.
//!
//! Optional is also a header line, to specifiy maybe the version of the used language, e.g. in
//! HTML `<!DOCTYPE html>`. The whole configuration will be encapsuled into definition
//! `SyntaxConfig`.
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
//!            opening_before: Single('|'),
//!            opening_after: Single('|'),
//!            closing_before: Double('|', '!'),
//!            closing_after: Single('|'),
//!        }),
//!        properties: None,
//!    };
//!
//!    let mut document = String::new();
//!    let mut markupsth = MarkupSth::new(&mut document, Language::Other(cfg)).unwrap();
//!    ```

use std::fmt;
use Insertion::*;

/// Defines an auto-insertion of MarkupSth before/after a tag element in form of (a) character(s).
///
/// Possible types auf automatic insertions, respectively no insertion or 1-3 characters. For
/// example in HTML and XML, every tag will be openend by a single character `<` and closed by
/// either a single character `>` or maybe by two `/>`. This different setups can be defined this
/// enumeration type. Note: this is the definition of one insertion either before or after a tag
/// identifier.
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

/// Defines the configuration of a self-closing tag element, e.g. in HTML `<img>`.
///
/// This struct stores the insertion before and the insertion after a tag identifier. Have a look at
/// the documentation of `Insertion` too.
#[derive(Clone, Debug)]
pub struct SelfClosingTagConfig {
    /// Optional character to be set before a single tag name (opening character).
    pub before: Insertion,
    /// Optional character to be set after a single tag (closing character).
    pub after: Insertion,
}

/// Defines the configuration of a tag pair, e.g. HTML `<p></p>`, opening tag and closing tag.
///
/// Which insertion shall be made before and after for each opening and closing tag element.
#[derive(Clone, Debug)]
pub struct TagPairConfig {
    /// Insertion before the opening tag element identifier.
    pub opening_before: Insertion,
    /// Inseration after the opening tag element.
    pub opening_after: Insertion,
    /// Inseration before the closing tag element identifier.
    pub closing_before: Insertion,
    /// Insertion after the closing tag element.
    pub closing_after: Insertion,
}

/// Defines the configuration of all optional properties, the tag can have additionally.
///
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

/// Defines a full configuration of a complete syntax in this crate, such as HTML or XML.
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

/// Selector for available pre-defined syntax configurations and wrapper to pass your own.
#[derive(Clone, Debug)]
pub enum Language {
    /// Selects the pre-defined HTML syntax.
    Html,
    /// Selects the pre-defined XML syntax.
    Xml,
    /// Wrapper selector to pass your own configuration.
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
                    opening_before: Single('<'),
                    opening_after: Single('>'),
                    closing_before: Double('<', '/'),
                    closing_after: Single('>'),
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
                doctype: Some(
                    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#.to_string(),
                ),
                self_closing: Some(SelfClosingTagConfig {
                    before: Single('<'),
                    after: Triple(' ', '/', '>'),
                }),
                tag_pairs: Some(TagPairConfig {
                    opening_before: Single('<'),
                    opening_after: Single('>'),
                    closing_before: Double('<', '/'),
                    closing_after: Single('>'),
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
