//! This module implements possible configuration functionality of markupsth.

#[derive(Clone, Debug, PartialEq)]
pub enum Insertion {
    /// No character.
    None,
    /// A single character.
    Single(char),
    /// Two characters.
    Double(char, char),
}

/// Configuration of a single tag element, e.g. in HTML <img>.
#[derive(Clone, Debug)]
pub struct SingleTagConfig {
    /// Optional character to be set before a single tag name (opening character).
    before: Insertion,
    /// Optional character to be set after a single tag (closing character).
    after: Insertion,
    // TODO: Optional operations to be done after closing a single tag ???
}

/// Configuration of a tag-pair element, e.g. HTML <p></p>.
#[derive(Clone, Debug)]
pub struct TagPairConfig {
    /// Insertion before the opening tag element identifier.
    opener_before: Insertion,
    /// Inseration after the opening tag element.
    opener_after: Insertion,
    /// Inseration before the closing tag element identifier.
    closer_before: Insertion,
    /// Insertion after the closing tag element.
    closer_after: Insertion,
}

/// Configuration for properties of tag elements.
#[derive(Clone, Debug)]
pub struct PropertyConfig {
    name_before: Insertion,
    name_after: Insertion,
    value_before: Insertion,
    value_after: Insertion,
    separator: Insertion,
    space: bool
}

/// Main configuration struct for a full setup of markupsth.
#[derive(Clone, Debug)]
pub struct Config {
    /// Configuration for single tag elements.
    single_tags: SingleTagConfig,
    /// Configuration for tag pair elements.
    tag_pairs: TagPairConfig,
    /// Configuration of properties of tag elements.
    properties: PropertyConfig, 
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn something() {
        todo!();
    }
}
