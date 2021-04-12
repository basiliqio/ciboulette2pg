use super::*;

macro_rules! safe_ident {
    ($name:ident, $text:literal) => {
        pub const $name: Ciboulette2PostgresSafeIdent = {
            Ciboulette2PostgresSafeIdent {
                inner: arcstr::literal!($text),
                prefix: arcstr::literal!(""),
                suffix: arcstr::literal!(""),
            }
        };
    };
}

safe_ident!(UUID_IDENT, "UUID");
safe_ident!(TEXT_IDENT, "TEXT");
safe_ident!(INTEGER_IDENT, "INTEGER");
safe_ident!(CIBOULETTE_ID_IDENT, "id");
safe_ident!(CIBOULETTE_REL_PREFIX, "rel");
safe_ident!(CIBOULETTE_INSERT_SUFFIX, "insert");
safe_ident!(CIBOULETTE_REL_DATA_SUFFIX, "rel_data");
safe_ident!(CIBOULETTE_UPDATE_SUFFIX, "update");
safe_ident!(CIBOULETTE_DATA_SUFFIX, "data");
safe_ident!(CIBOULETTE_ID_SUFFIX, "id");
safe_ident!(CIBOULETTE_SORT_PREFIX, "sort");
safe_ident!(CIBOULETTE_MAIN_IDENTIFIER_PREFIX, "main");
safe_ident!(CIBOULETTE_EMPTY_IDENT, "");
safe_ident!(CIBOULETTE_TYPE_IDENT, "type");
safe_ident!(CIBOULETTE_DATA_IDENT, "data");
safe_ident!(CIBOULETTE_RELATED_ID_IDENT, "related_id");
safe_ident!(CIBOULETTE_RELATED_TYPE_IDENT, "related_type");

/// An modifier for [Ciboulette2PostgresSafeIdent](Ciboulette2PostgresSafeIdent)
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) enum Ciboulette2PostgresSafeIdentModifier {
    Prefix(Ciboulette2PostgresSafeIdent),
    Suffix(Ciboulette2PostgresSafeIdent),
}

/// An identifier that is safe to be wrapped in quote
#[derive(Clone, Debug, PartialEq, Eq, Ord, Default, PartialOrd)]
pub struct Ciboulette2PostgresSafeIdent {
    pub prefix: ArcStr,
    pub inner: ArcStr,
    pub suffix: ArcStr,
}

impl std::fmt::Display for Ciboulette2PostgresSafeIdent {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match (self.prefix.is_empty(), self.suffix.is_empty()) {
            (false, false) => write!(f, "{}_{}_{}", self.prefix, self.inner, self.suffix),
            (false, true) => write!(f, "{}_{}", self.prefix, self.inner),
            (true, false) => write!(f, "{}_{}", self.inner, self.suffix),
            (true, true) => write!(f, "{}", self.inner),
        }
    }
}

impl Ciboulette2PostgresSafeIdent {
    /// Check that the identifier is safe
    fn check_routine(val: ArcStr) -> Result<ArcStr, Ciboulette2SqlError> {
        if (*val).find('\0').is_some() {
            return Err(Ciboulette2SqlError::NullCharIdent(val.to_string()));
        }
        if !(*val).chars().all(|x| x.is_ascii()) {
            return Err(Ciboulette2SqlError::NonAsciiCharInIdent(val.to_string()));
        }
        if (*val).find('"').is_some() {
            return Ok(ArcStr::from((*val).replace('"', "\"\"")));
        }
        Ok(val)
    }

    fn take(self) -> ArcStr {
        self.inner
    }
    pub fn check(val: ArcStr) -> Result<Self, Ciboulette2SqlError> {
        Ok(Ciboulette2PostgresSafeIdent {
            inner: Self::check_routine(val)?,
            ..Default::default()
        })
    }

    pub(crate) fn add_modifier(
        mut self,
        modifier: Ciboulette2PostgresSafeIdentModifier,
    ) -> Self {
        match modifier {
            Ciboulette2PostgresSafeIdentModifier::Prefix(prefix) => self.prefix = prefix.take(),
            Ciboulette2PostgresSafeIdentModifier::Suffix(suffix) => self.suffix = suffix.take(),
        };
        self
    }

    pub(crate) fn to_writer(
        &self,
        writer: &mut dyn std::io::Write,
    ) -> Result<(), Ciboulette2SqlError> {
        match (self.prefix.is_empty(), self.suffix.is_empty()) {
            (false, false) => write!(writer, "{}_{}_{}", self.prefix, self.inner, self.suffix)?,
            (false, true) => write!(writer, "{}_{}", self.prefix, self.inner)?,
            (true, false) => write!(writer, "{}_{}", self.inner, self.suffix)?,
            (true, true) => write!(writer, "{}", self.inner)?,
        };
        Ok(())
    }
}

impl std::convert::TryFrom<ArcStr> for Ciboulette2PostgresSafeIdent {
    type Error = Ciboulette2SqlError;

    fn try_from(value: ArcStr) -> Result<Self, Self::Error> {
        Ciboulette2PostgresSafeIdent::check(value)
    }
}

impl std::convert::TryFrom<&ArcStr> for Ciboulette2PostgresSafeIdent {
    type Error = Ciboulette2SqlError;

    fn try_from(value: &ArcStr) -> Result<Self, Self::Error> {
        Ciboulette2PostgresSafeIdent::check(value.clone())
    }
}

impl std::convert::TryFrom<&'static str> for Ciboulette2PostgresSafeIdent {
    type Error = Ciboulette2SqlError;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        Ciboulette2PostgresSafeIdent::check(ArcStr::from(value))
    }
}

impl From<&Ciboulette2PostgresSafeIdent> for Ciboulette2PostgresSafeIdent {
    fn from(value: &Ciboulette2PostgresSafeIdent) -> Self {
        Ciboulette2PostgresSafeIdent {
            inner: value.inner.clone(),
            ..Default::default()
        }
    }
}
