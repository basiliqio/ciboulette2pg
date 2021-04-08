use super::*;
use std::borrow::Borrow;
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Ciboulette2PostgresStr<'store> {
    Borrowed(&'store str),
    Arc(ArcStr),
}

impl<'store> Default for Ciboulette2PostgresStr<'store> {
    fn default() -> Self {
        Ciboulette2PostgresStr::Borrowed("")
    }
}

impl<'store> std::fmt::Display for Ciboulette2PostgresStr<'store> {
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

impl<'store> From<&'store str> for Ciboulette2PostgresStr<'store> {
    fn from(other: &'store str) -> Self {
        Ciboulette2PostgresStr::Borrowed(other)
    }
}

impl<'store> From<String> for Ciboulette2PostgresStr<'store> {
    fn from(other: String) -> Self {
        Ciboulette2PostgresStr::Arc(ArcStr::from(other))
    }
}

impl<'store> From<Cow<'store, str>> for Ciboulette2PostgresStr<'store> {
    fn from(other: Cow<'store, str>) -> Self {
        match other {
            Cow::Borrowed(x) => Ciboulette2PostgresStr::Borrowed(x),
            Cow::Owned(x) => Ciboulette2PostgresStr::Arc(ArcStr::from(x)),
        }
    }
}

impl<'store> From<ArcStr> for Ciboulette2PostgresStr<'store> {
    fn from(other: ArcStr) -> Self {
        Ciboulette2PostgresStr::Arc(other)
    }
}
