//! TODO

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TagType {
    SelfClosing,
    Opening,
    Closing,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TagPattern {
    /// Identifier, e.g. `html` or `div`.
    pub ident: String,
    /// Type of tag, see `TagType`.
    pub typ: TagType,
}

impl TagPattern {
    /// New type pattern for creating a new `Tag` of type `Opening`.
    #[inline(always)]
    pub fn opening(ident: &str) -> TagPattern {
        TagPattern {
            ident: ident.to_string(),
            typ: TagType::Opening,
        }
    }

    /// New type pattern for creating a new `Tag` of type `Opening`.
    #[inline(always)]
    pub fn closing(ident: &str) -> TagPattern {
        TagPattern {
            ident: ident.to_string(),
            typ: TagType::Closing,
        }
    }

    /// New type pattern for creating a new `Tag` of type `Opening`.
    #[inline(always)]
    pub fn self_closing(ident: &str) -> TagPattern {
        TagPattern {
            ident: ident.to_string(),
            typ: TagType::SelfClosing,
        }
    }
}

#[macro_export]
macro_rules! tag_rule_set_internal {
    (on_open $ident:literal $vec:expr) => {
        $vec.push(TagPattern::opening($ident));
    };
    (on_close $ident:literal $vec:expr) => {
        $vec.push(TagPattern::closing($ident));
    };
    (on_self $ident:literal $vec:expr) => {
        $vec.push(TagPattern::self_closing($ident));
    };
    (on_both $ident:literal $vec:expr) => {
        $vec.push(TagPattern::opening($ident));
        $vec.push(TagPattern::closing($ident));
    };
}

#[macro_export]
macro_rules! tag_rule_set {
    ($($rule:tt $ident:literal),*) => {
        {
            let mut v: Vec<crate::tagpattern::TagPattern> = Vec::new();
            $($crate::tag_rule_set_internal!($rule $ident v);)*
            v
        }
    }
}
