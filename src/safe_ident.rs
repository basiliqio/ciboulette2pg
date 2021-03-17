use super::*;

pub const UUID_IDENT: Ciboulette2PostgresSafeIdent<'static> = {
    Ciboulette2PostgresSafeIdent {
        inner: Cow::Borrowed("UUID"),
    }
};
pub const TEXT_IDENT: Ciboulette2PostgresSafeIdent<'static> = {
    Ciboulette2PostgresSafeIdent {
        inner: Cow::Borrowed("TEXT"),
    }
};

pub const INTEGER_IDENT: Ciboulette2PostgresSafeIdent<'static> = {
    Ciboulette2PostgresSafeIdent {
        inner: Cow::Borrowed("INTEGER"),
    }
};

#[derive(Clone, Debug, PartialEq, Eq, Ord, Default, PartialOrd)]
pub struct Ciboulette2PostgresSafeIdent<'a> {
    pub inner: Cow<'a, str>,
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
    pub fn check(val: Cow<'a, str>) -> Result<Cow<'a, str>, Ciboulette2SqlError> {
        if val.as_ref().find('\0').is_some() {
            return Err(Ciboulette2SqlError::NullCharIdent(val.to_string()));
        }
        if !val.as_ref().chars().all(|x| x.is_ascii()) {
            return Err(Ciboulette2SqlError::NonAsciiCharInIdent(val.to_string()));
        }
        if val.as_ref().find('"').is_some() {
            return Ok(Cow::Owned(val.replace('"', "\"\"")));
        }
        Ok(val)
    }
}

impl<'a> std::ops::Deref for Ciboulette2PostgresSafeIdent<'a> {
    type Target = Cow<'a, str>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> std::convert::TryFrom<&'a str> for Ciboulette2PostgresSafeIdent<'a> {
    type Error = Ciboulette2SqlError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(Ciboulette2PostgresSafeIdent {
            inner: Ciboulette2PostgresSafeIdent::check(Cow::Borrowed(value))?,
        })
    }
}

impl<'a> std::convert::TryFrom<String> for Ciboulette2PostgresSafeIdent<'a> {
    type Error = Ciboulette2SqlError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Ciboulette2PostgresSafeIdent {
            inner: Ciboulette2PostgresSafeIdent::check(Cow::Owned(value))?,
        })
    }
}

impl<'a> std::convert::TryFrom<Cow<'a, str>> for Ciboulette2PostgresSafeIdent<'a> {
    type Error = Ciboulette2SqlError;

    fn try_from(value: Cow<'a, str>) -> Result<Self, Self::Error> {
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
