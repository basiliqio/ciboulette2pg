use super::*;

pub const UUID_IDENT: Ciboulette2PostgresSafeIdent<'static> = {
    Ciboulette2PostgresSafeIdent {
        inner: Ciboulette2PostgresStr::Borrowed("UUID"),
    }
};
pub const TEXT_IDENT: Ciboulette2PostgresSafeIdent<'static> = {
    Ciboulette2PostgresSafeIdent {
        inner: Ciboulette2PostgresStr::Borrowed("TEXT"),
    }
};

pub const INTEGER_IDENT: Ciboulette2PostgresSafeIdent<'static> = {
    Ciboulette2PostgresSafeIdent {
        inner: Ciboulette2PostgresStr::Borrowed("INTERGER"),
    }
};

/// An identifier that is safe to be wrapped in quote
#[derive(Clone, Debug, PartialEq, Eq, Ord, Default, PartialOrd)]
pub struct Ciboulette2PostgresSafeIdent<'a> {
    pub inner: Ciboulette2PostgresStr<'a>,
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
    pub fn check(
        val: Ciboulette2PostgresStr<'a>
    ) -> Result<Ciboulette2PostgresStr<'a>, Ciboulette2SqlError> {
        if (*val).find('\0').is_some() {
            return Err(Ciboulette2SqlError::NullCharIdent(val.to_string()));
        }
        if !(*val).chars().all(|x| x.is_ascii()) {
            return Err(Ciboulette2SqlError::NonAsciiCharInIdent(val.to_string()));
        }
        if (*val).find('"').is_some() {
            return Ok(Ciboulette2PostgresStr::Arc(ArcStr::from(
                (*val).replace('"', "\"\""),
            )));
        }
        Ok(val)
    }
}

impl<'a> std::ops::Deref for Ciboulette2PostgresSafeIdent<'a> {
    type Target = Ciboulette2PostgresStr<'a>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> std::convert::TryFrom<&'a str> for Ciboulette2PostgresSafeIdent<'a> {
    type Error = Ciboulette2SqlError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(Ciboulette2PostgresSafeIdent {
            inner: Ciboulette2PostgresSafeIdent::check(Ciboulette2PostgresStr::from(value))?,
        })
    }
}

impl<'a> std::convert::TryFrom<String> for Ciboulette2PostgresSafeIdent<'a> {
    type Error = Ciboulette2SqlError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Ciboulette2PostgresSafeIdent {
            inner: Ciboulette2PostgresSafeIdent::check(Ciboulette2PostgresStr::Arc(ArcStr::from(
                value,
            )))?,
        })
    }
}

impl<'a> std::convert::TryFrom<ArcStr> for Ciboulette2PostgresSafeIdent<'a> {
    type Error = Ciboulette2SqlError;

    fn try_from(value: ArcStr) -> Result<Self, Self::Error> {
        Ok(Ciboulette2PostgresSafeIdent {
            inner: Ciboulette2PostgresSafeIdent::check(Ciboulette2PostgresStr::from(value))?,
        })
    }
}

impl<'a> std::convert::TryFrom<Cow<'a, str>> for Ciboulette2PostgresSafeIdent<'a> {
    type Error = Ciboulette2SqlError;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
        Ok(Ciboulette2PostgresSafeIdent {
            inner: Ciboulette2PostgresSafeIdent::check(Ciboulette2PostgresStr::from(value))?,
        })
    }
}

impl<'a> std::convert::TryFrom<Ciboulette2PostgresStr<'a>> for Ciboulette2PostgresSafeIdent<'a> {
    type Error = Ciboulette2SqlError;

    fn try_from(value: Ciboulette2PostgresStr<'a>) -> Result<Self, Self::Error> {
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
