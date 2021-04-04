use super::*;

pub const UUID_IDENT: Ciboulette2PostgresSafeIdent<'static> = {
    Ciboulette2PostgresSafeIdent {
        inner: ArcCowStr::Cow(Cow::Borrowed("UUID")),
    }
};
pub const TEXT_IDENT: Ciboulette2PostgresSafeIdent<'static> = {
    Ciboulette2PostgresSafeIdent {
        inner: ArcCowStr::Cow(Cow::Borrowed("TEXT")),
    }
};

pub const INTEGER_IDENT: Ciboulette2PostgresSafeIdent<'static> = {
    Ciboulette2PostgresSafeIdent {
        inner: ArcCowStr::Cow(Cow::Borrowed("INTERGER")),
    }
};

/// An identifier that is safe to be wrapped in quote
#[derive(Clone, Debug, PartialEq, Eq, Ord, Default, PartialOrd)]
pub struct Ciboulette2PostgresSafeIdent<'a> {
    pub inner: ArcCowStr<'a>,
}

impl<'a> std::fmt::Display for Ciboulette2PostgresSafeIdent<'a> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<'a> Ciboulette2PostgresSafeIdent<'a> {
    /// Check that the identifier is safe
    pub fn check(val: ArcCowStr<'a>) -> Result<ArcCowStr<'a>, Ciboulette2SqlError> {
        if (*val).find('\0').is_some() {
            return Err(Ciboulette2SqlError::NullCharIdent(val.to_string()));
        }
        if !(*val).chars().all(|x| x.is_ascii()) {
            return Err(Ciboulette2SqlError::NonAsciiCharInIdent(val.to_string()));
        }
        if (*val).find('"').is_some() {
            return Ok(ArcCowStr::Cow(Cow::Owned((*val).replace('"', "\"\""))));
        }
        Ok(val)
    }
}

impl<'a> std::ops::Deref for Ciboulette2PostgresSafeIdent<'a> {
    type Target = ArcCowStr<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> std::convert::TryFrom<&'a str> for Ciboulette2PostgresSafeIdent<'a> {
    type Error = Ciboulette2SqlError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(Ciboulette2PostgresSafeIdent {
            inner: Ciboulette2PostgresSafeIdent::check(ArcCowStr::Cow(Cow::Borrowed(value)))?,
        })
    }
}

impl<'a> std::convert::TryFrom<String> for Ciboulette2PostgresSafeIdent<'a> {
    type Error = Ciboulette2SqlError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Ciboulette2PostgresSafeIdent {
            inner: Ciboulette2PostgresSafeIdent::check(ArcCowStr::Cow(Cow::Owned(value)))?,
        })
    }
}

impl<'a> std::convert::TryFrom<ArcStr> for Ciboulette2PostgresSafeIdent<'a> {
    type Error = Ciboulette2SqlError;

    fn try_from(value: ArcStr) -> Result<Self, Self::Error> {
        Ok(Ciboulette2PostgresSafeIdent {
            inner: Ciboulette2PostgresSafeIdent::check(ArcCowStr::Arc(value))?,
        })
    }
}

impl<'a> std::convert::TryFrom<Cow<'a, str>> for Ciboulette2PostgresSafeIdent<'a> {
    type Error = Ciboulette2SqlError;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Ok(Ciboulette2PostgresSafeIdent {
            inner: Ciboulette2PostgresSafeIdent::check(ArcCowStr::Cow(value))?,
        })
    }
}

impl<'a> std::convert::TryFrom<ArcCowStr<'a>> for Ciboulette2PostgresSafeIdent<'a> {
    type Error = Ciboulette2SqlError;

    fn try_from(value: ArcCowStr<'a>) -> Result<Self, Self::Error> {
        Ok(Ciboulette2PostgresSafeIdent {
            inner: Ciboulette2PostgresSafeIdent::check(value)?,
        })
    }
}

impl<'a> From<&Ciboulette2PostgresSafeIdent<'a>> for Ciboulette2PostgresSafeIdent<'a> {
    fn from(value: &Ciboulette2PostgresSafeIdent<'a>) -> Self {
        Ciboulette2PostgresSafeIdent {
            inner: value.inner.clone(),
        }
    }
}
