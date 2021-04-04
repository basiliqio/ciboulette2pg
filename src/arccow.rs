use super::*;
use std::borrow::Borrow;
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum ArcCowStr<'a> {
    Cow(Cow<'a, str>),
    Arc(ArcStr),
}

impl<'a> Default for ArcCowStr<'a> {
    fn default() -> Self {
        ArcCowStr::Cow(Cow::Borrowed(""))
    }
}

impl<'a> std::fmt::Display for ArcCowStr<'a> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            ArcCowStr::Cow(x) => write!(f, "{}", x.as_ref()),
            ArcCowStr::Arc(x) => write!(f, "{}", x),
        }
    }
}

impl Deref for ArcCowStr<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match *self {
            ArcCowStr::Cow(ref c) => c.borrow(),
            ArcCowStr::Arc(ref a) => a.borrow(),
        }
    }
}

impl<'a> From<&'a str> for ArcCowStr<'a> {
    fn from(other: &'a str) -> Self {
        ArcCowStr::Cow(Cow::Borrowed(other))
    }
}

impl<'a> From<String> for ArcCowStr<'a> {
    fn from(other: String) -> Self {
        ArcCowStr::Cow(Cow::Owned(other))
    }
}

impl<'a> From<Cow<'a, str>> for ArcCowStr<'a> {
    fn from(other: Cow<'a, str>) -> Self {
        ArcCowStr::Cow(other)
    }
}

impl<'a> From<ArcStr> for ArcCowStr<'a> {
    fn from(other: ArcStr) -> Self {
        ArcCowStr::Arc(other)
    }
}
