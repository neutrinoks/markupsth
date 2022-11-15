# notesth

A very simple Rust library for writing any kind of text files (note something). It is a basis
for various kinds of writer-types, written in Rust. The library assists in formatting, escpially 
indenting and adding line-feeds when writing. If you want to generate some kind of code, e.g. HTML,
Rust etc. this is very useful.

### Rust-boundaries

At the moment there is only a version available, that implements ```std::fmt::Write```. If requested
a version supporting ```std::io::Write``` could also be implemented. Just contact me.

### Examples

To generate the following HTML output:
```html
<div>
    <p>Some interesting written here</p>
</div>
```
an implementation would look like:
```rust
use notesth::NoteSth;
use std::fmt::{Error, Write};

fn main() -> Result<(), Error> {
    let mut text = String::new();
    let mut notes = NoteSth::new(&mut text);
    
    notes.open_element("<div>", "</div>")?;
    notes.line_feed_inc()?;
    notes.open_element("<p>", "</p>")?;
    notes.write_str("Some interesting written here")?;
    notes.close_element()?;
    notes.line_feed_dec()?;
    notes.close_element()?;
    println!("{}", text);
    
    Ok(())
}
```

Another formatted text example:
```rust
use notesth::NoteSth;
use std::fmt::{Error, Write};

fn main() -> Result<(), Error> {
    let mut text = String::new();
    let mut notes = NoteSth::new(&mut text);
    
    notes.open_element("Begin", "End")?;
    notes.line_feed_inc()?;
    notes.write_str("...of a very nice story. And, happy...")?;
    notes.line_feed_dec()?;
    notes.close_element()?;
    println!("{}", text);
    
    Ok(())
}
```