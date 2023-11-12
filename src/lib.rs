//! # notesth
//!
//! A very simple Rust library for writing any kind of text files (note something). It is a basis
//! for various kinds of writer-types, written in Rust. The library assists in formatting, escpially
//! indenting and adding line-feeds when writing. If you want to generate some kind of code, e.g. HTML,
//! Rust etc. this is very useful.
//!
//! ### Rust-boundaries
//!
//! At the moment there is only a version available, that implements ```std::fmt::Write```. If requested
//! a version supporting ```std::io::Write``` could also be implemented. Just contact me.
//!
//! ### Examples
//!
//! To generate the following HTML output:
//! ```html
//! <div>
//!     <p>Some interesting written here</p>
//! </div>
//! ```
//! an implementation would look like:
//!
//! Another formatted text example:

pub mod format;
pub mod markupsth;
pub mod syntax;

pub use crate::{markupsth::MarkupSth, syntax::MarkupLanguage};

// #[cfg(test)]
// mod tests {
// use super::*;

// #[test]
// fn html() {
//     todo!();
// }

// #[test]
// fn xml() {
//     todo!();
// }
// }
