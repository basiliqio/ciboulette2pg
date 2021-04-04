use super::*;
use std::borrow::Borrow;
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Ciboulette2PostgresStr<'a> {
    Borrowed(&'a str),
    Arc(ArcStr),
}

impl<'a> Default for Ciboulette2PostgresStr<'a> {
    fn default() -> Self {
        Ciboulette2PostgresStr::Borrowed("")
    }
}

impl<'a> std::fmt::Display for Ciboulette2PostgresStr<'a> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match *self {
            Ciboulette2PostgresStr::Borrowed(x) => write!(f, "{}", x),
            Ciboulette2PostgresStr::Arc(ref x) => write!(f, "{}", x),
        }
    }
}

impl Deref for Ciboulette2PostgresStr<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match *self {
            Ciboulette2PostgresStr::Borrowed(c) => c,
            Ciboulette2PostgresStr::Arc(ref a) => a.borrow(),
        }
    }
}

impl<'a> From<&'a str> for Ciboulette2PostgresStr<'a> {
    fn from(other: &'a str) -> Self {
        Ciboulette2PostgresStr::Borrowed(other)
    }
}

impl<'a> From<String> for Ciboulette2PostgresStr<'a> {
    fn from(other: String) -> Self {
        Ciboulette2PostgresStr::Arc(ArcStr::from(other))
    }
}

impl<'a> From<Cow<'a, str>> for Ciboulette2PostgresStr<'a> {
    fn from(other: Cow<'a, str>) -> Self {
        match other {
            Cow::Borrowed(x) => Ciboulette2PostgresStr::Borrowed(x),
            Cow::Owned(x) => Ciboulette2PostgresStr::Arc(ArcStr::from(x)),
        }
    }
}

impl<'a> From<ArcStr> for Ciboulette2PostgresStr<'a> {
    fn from(other: ArcStr) -> Self {
        Ciboulette2PostgresStr::Arc(other)
    }
}
