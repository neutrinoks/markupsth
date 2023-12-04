# markupsth

A very simple Rust library for writing almost all kinds of formatted text files, escpially
Markup files, such as HTML and XML. Its original purpose was to develop a Text-to-HTML
converter, but got slowly extended step-by-step. HTML and XML support is default and
implemented, but you can also define your own Markup Language by configuring your individual
syntax. There are also some pre-defined formatters to either no formatting at all the Markup
output or have some out-of-the-box formatting styles for a beautiful and readable HTML code.

For an individual syntax style (own Markup Language), have a look at the `syntax` module. For
an own formatting style (implement your own `Formatter` to be used with this crate), have a
look at the `format` module.

### Request for changes

In case of interest, I am also willing to extend this crate any time by new Markup Languages,
formatting styles, or any other kind of meaningful modifications. Feel free to contact me via
Email provided in the cargo manifest file.

## Examples

By using an implemented Markup Language such as HTML or XML, and a pre-defined `Formatter`, you
can quickly write some converter or HTML-generator. Such a quick-start guide can be seen in the
following example.

### Readable HTML

To generate the following HTML code:
```html
<!DOCTYPE html>
<html>
<head>
    <title>New Website</title>
    <link href="css/style.css" rel="stylesheet">
</head>
<body>
    <section>
        <div>
            <div><img src="image.jpg"></div>
            <p>This is HTML</p>
        </div>
    </section>
</body>
</html>
```
you need to implement:
```rust
use markupsth::{
    AutoIndent, FixedRule, Language, MarkupSth, properties,
};

// Setup a document (String), MarkupSth and a default formatter.
let mut document = String::new();
let mut mus = MarkupSth::new(&mut document, Language::Html).unwrap();

// Default Formatter is an AutoIndent, so get it, configure it!
let fmtr = mus.formatter.optional_fixed_ruleset().unwrap();
fmtr.add_tags_to_rule(&["head", "body", "section"], FixedRule::IndentAlways)
    .unwrap();
fmtr.add_tags_to_rule(&["html"], FixedRule::LfAlways).unwrap();
fmtr.add_tags_to_rule(&["title", "link", "div", "p"], FixedRule::LfClosing)
    .unwrap();

// Generate the content of example shown above.
mus.open("html").unwrap();
mus.open("head").unwrap();
mus.open_close_w("title", "New Website").unwrap();
mus.self_closing("link").unwrap();
properties!(mus, "href", "css/style.css", "rel", "stylesheet").unwrap();
mus.close().unwrap();
mus.open("body").unwrap();
mus.open("section").unwrap();
mus.open("div").unwrap();
mus.new_line().unwrap();
mus.open("div").unwrap();
mus.self_closing("img").unwrap();
properties!(mus, "src", "image.jpg").unwrap();
mus.close().unwrap();
mus.open_close_w("p", "This is HTML").unwrap();
mus.close_all().unwrap();
mus.finalize().unwrap();
# assert_eq!(document, markupsth::testfile("formatted_html_auto_indent.html"));
```

### Readable XML

To generate the following output:
```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<directory>
    <title>Wikipedia List of Cities</title>
    <entry>
        <keyword>Hamburg</keyword>
        <entrystext>Hamburg is the residence of ...</entrystext>
    </entry>
    <entry>
        <keyword>Munich</keyword>
        <entrystext>Munich is the residence of ...</entrystext>
    </entry>
</directory>
```
you have to implement:
```rust
use markupsth::{AutoIndent, FixedRule, Language, MarkupSth, properties};

let do_entry = |mus: &mut MarkupSth, name: &str| {
    mus.open("entry").unwrap();
    mus.open("keyword").unwrap();
    mus.text(name).unwrap();
    mus.close().unwrap();
    mus.open("entrystext").unwrap();
    mus.text(&format!("{} is the residence of ...", name))
        .unwrap();
    mus.close().unwrap();
    mus.close().unwrap();
};

// Setup a document (String), MarkupSth and a default formatter.
let mut document = String::new();
let mut mus = MarkupSth::new(&mut document, Language::Html).unwrap();

// Default Formatter is an AutoIndent, so get it, configure it!
let fmtr = mus.formatter.optional_fixed_ruleset().unwrap();
fmtr.add_tags_to_rule(&["directory", "entry"], FixedRule::IndentAlways).unwrap();
fmtr.add_tags_to_rule(&["title", "keyword", "entrystext"], FixedRule::LfClosing).unwrap();

// Generate the content of example shown above.
mus.open("directory").unwrap();
mus.open("title").unwrap();
mus.text("Wikipedia List of Cities").unwrap();
mus.close().unwrap();
do_entry(&mut mus, "Hamburg");
do_entry(&mut mus, "Munich");
mus.close_all().unwrap();
mus.finalize().unwrap();
```

## More Informations

Have a look into the crate's code documentation by using cargo.
