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
pub struct Ciboulette2PostgresSafeIdent<'store> {
    pub inner: Ciboulette2PostgresStr<'store>,
}

impl<'store> std::fmt::Display for Ciboulette2PostgresSafeIdent<'store> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl<'store> Ciboulette2PostgresSafeIdent<'store> {
    /// Check that the identifier is safe
    pub fn check(
        val: Ciboulette2PostgresStr<'store>
    ) -> Result<Ciboulette2PostgresStr<'store>, Ciboulette2SqlError> {
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

impl<'store> std::ops::Deref for Ciboulette2PostgresSafeIdent<'store> {
    type Target = Ciboulette2PostgresStr<'store>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'store> std::convert::TryFrom<&'store str> for Ciboulette2PostgresSafeIdent<'store> {
    type Error = Ciboulette2SqlError;

    fn try_from(value: &'store str) -> Result<Self, Self::Error> {
        Ok(Ciboulette2PostgresSafeIdent {
            inner: Ciboulette2PostgresSafeIdent::check(Ciboulette2PostgresStr::from(value))?,
        })
    }
}

impl<'store> std::convert::TryFrom<String> for Ciboulette2PostgresSafeIdent<'store> {
    type Error = Ciboulette2SqlError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Ciboulette2PostgresSafeIdent {
            inner: Ciboulette2PostgresSafeIdent::check(Ciboulette2PostgresStr::Arc(ArcStr::from(
                value,
            )))?,
        })
    }
}

impl<'store> std::convert::TryFrom<ArcStr> for Ciboulette2PostgresSafeIdent<'store> {
    type Error = Ciboulette2SqlError;

    fn try_from(value: ArcStr) -> Result<Self, Self::Error> {
        Ok(Ciboulette2PostgresSafeIdent {
            inner: Ciboulette2PostgresSafeIdent::check(Ciboulette2PostgresStr::from(value))?,
        })
    }
}

impl<'store> std::convert::TryFrom<Cow<'store, str>> for Ciboulette2PostgresSafeIdent<'store> {
    type Error = Ciboulette2SqlError;

    fn try_from(value: Cow<'store, str>) -> Result<Self, Self::Error> {
        Ok(Ciboulette2PostgresSafeIdent {
            inner: Ciboulette2PostgresSafeIdent::check(Ciboulette2PostgresStr::from(value))?,
        })
    }
}

impl<'store> std::convert::TryFrom<Ciboulette2PostgresStr<'store>>
    for Ciboulette2PostgresSafeIdent<'store>
{
    type Error = Ciboulette2SqlError;

    fn try_from(value: Ciboulette2PostgresStr<'store>) -> Result<Self, Self::Error> {
        Ok(Ciboulette2PostgresSafeIdent {
            inner: Ciboulette2PostgresSafeIdent::check(value)?,
        })
    }
}

impl<'store> From<&Ciboulette2PostgresSafeIdent<'store>> for Ciboulette2PostgresSafeIdent<'store> {
    fn from(value: &Ciboulette2PostgresSafeIdent<'store>) -> Self {
        Ciboulette2PostgresSafeIdent {
            inner: value.inner.clone(),
        }
    }
}
