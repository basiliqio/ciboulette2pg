use super::*;

macro_rules! safe_ident {
    ($name:ident, $text:literal) => {
        pub const $name: Ciboulette2PgSafeIdent = {
            Ciboulette2PgSafeIdent {
                inner: arcstr::literal!($text),
                prefix: arcstr::literal!(""),
                suffix: arcstr::literal!(""),
                index: None,
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
safe_ident!(CIBOULETTE_UPDATE_SUFFIX, "update");
safe_ident!(CIBOULETTE_DATA_SUFFIX, "data");
safe_ident!(CIBOULETTE_SORT_PREFIX, "sort");
safe_ident!(CIBOULETTE_CTE_FINAL_MAIN_DATA, "cte_final_main_data");
safe_ident!(CIBOULETTE_MAIN_IDENTIFIER, "main_id");
safe_ident!(CIBOULETTE_EMPTY_IDENT, "");
safe_ident!(CIBOULETTE_TYPE_IDENT, "type");
safe_ident!(CIBOULETTE_DATA_IDENT, "data");
safe_ident!(CIBOULETTE_RELATED_ID_IDENT, "related_id");
safe_ident!(CIBOULETTE_RELATED_TYPE_IDENT, "related_type");

/// An modifier for [Ciboulette2PgSafeIdent](Ciboulette2PgSafeIdent)
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub(crate) enum Ciboulette2PgSafeIdentModifier {
    /// A prefix identifier
    Prefix(Ciboulette2PgSafeIdent),
    /// A suffix identifier
    Suffix(Ciboulette2PgSafeIdent),
    /// An index to add to the ident
    Index(Option<usize>),
}

/// An selector for [Ciboulette2PgSafeIdent](Ciboulette2PgSafeIdent)
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Ciboulette2PgSafeIdentSelector {
    Single(Ciboulette2PgSafeIdent),
    Multi(Vec<Ciboulette2PgSafeIdent>),
}

/// An identifier that is safe to be wrapped in quote
#[derive(Clone, Debug, PartialEq, Eq, Ord, Default, PartialOrd)]
pub struct Ciboulette2PgSafeIdent {
    pub prefix: ArcStr,
    pub inner: ArcStr,
    pub suffix: ArcStr,
    pub index: Option<usize>,
}

impl std::fmt::Display for Ciboulette2PgSafeIdent {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match (self.prefix.is_empty(), self.suffix.is_empty(), self.index) {
            (false, false, Some(i)) => {
                write!(f, "{}_{}_{}_{}", self.prefix, self.inner, self.suffix, i)
            }
            (false, false, None) => write!(f, "{}_{}_{}", self.prefix, self.inner, self.suffix),
            (false, true, Some(i)) => write!(f, "{}_{}_{}", self.prefix, self.inner, i),
            (false, true, None) => write!(f, "{}_{}", self.prefix, self.inner),
            (true, false, Some(i)) => write!(f, "{}_{}_{}", self.inner, self.suffix, i),
            (true, false, None) => write!(f, "{}_{}", self.inner, self.suffix),
            (true, true, Some(i)) => write!(f, "{}_{}", self.inner, i),
            (true, true, None) => write!(f, "{}", self.inner),
        }
    }
}

impl Ciboulette2PgSafeIdent {
    /// Check that the identifier is safe
    fn check_routine(val: ArcStr) -> Result<ArcStr, Ciboulette2PgError> {
        if (*val).find('\0').is_some() {
            return Err(Ciboulette2PgError::NullCharIdent(val.to_string()));
        }
        if !(*val).chars().all(|x| x.is_ascii()) {
            return Err(Ciboulette2PgError::NonAsciiCharInIdent(val.to_string()));
        }
        if (*val).find('"').is_some() {
            return Ok(ArcStr::from((*val).replace('"', "\"\"")));
        }
        Ok(val)
    }

    fn take(self) -> ArcStr {
        self.inner
    }
    pub fn check(val: ArcStr) -> Result<Self, Ciboulette2PgError> {
        Ok(Ciboulette2PgSafeIdent {
            inner: Self::check_routine(val)?,
            ..Default::default()
        })
    }

    pub(crate) fn add_modifier(
        mut self,
        modifier: Ciboulette2PgSafeIdentModifier,
    ) -> Self {
        match modifier {
            Ciboulette2PgSafeIdentModifier::Prefix(prefix) => self.prefix = prefix.take(),
            Ciboulette2PgSafeIdentModifier::Suffix(suffix) => self.suffix = suffix.take(),
            Ciboulette2PgSafeIdentModifier::Index(index) => self.index = index,
        };
        self
    }

    pub(crate) fn to_writer(
        &self,
        writer: &mut dyn std::io::Write,
    ) -> Result<(), Ciboulette2PgError> {
        match (self.prefix.is_empty(), self.suffix.is_empty(), self.index) {
            (false, false, Some(i)) => write!(
                writer,
                "{}_{}_{}_{}",
                self.prefix, self.inner, self.suffix, i
            )?,
            (false, false, None) => {
                write!(writer, "{}_{}_{}", self.prefix, self.inner, self.suffix)?
            }
            (false, true, Some(i)) => write!(writer, "{}_{}_{}", self.prefix, self.inner, i)?,
            (false, true, None) => write!(writer, "{}_{}", self.prefix, self.inner)?,
            (true, false, Some(i)) => write!(writer, "{}_{}_{}", self.inner, self.suffix, i)?,
            (true, false, None) => write!(writer, "{}_{}", self.inner, self.suffix)?,
            (true, true, Some(i)) => write!(writer, "{}_{}", self.inner, i)?,
            (true, true, None) => write!(writer, "{}", self.inner)?,
        };
        Ok(())
    }
}

impl Ciboulette2PgSafeIdentSelector {
    pub(crate) fn add_modifier(
        mut self,
        modifier: Ciboulette2PgSafeIdentModifier,
    ) -> Self {
        match self {
            Ciboulette2PgSafeIdentSelector::Single(x) => {
                Ciboulette2PgSafeIdentSelector::Single(x.add_modifier(modifier))
            }
            Ciboulette2PgSafeIdentSelector::Multi(mut x) => {
                for ident in x.iter_mut() {
                    let ident_val = std::mem::take(ident);
                    *ident = ident_val.add_modifier(modifier.clone());
                }
                Ciboulette2PgSafeIdentSelector::Multi(x)
            }
        }
    }

    pub(crate) fn to_writer(
        &self,
        writer: &mut dyn std::io::Write,
    ) -> Result<(), Ciboulette2PgError> {
        match self {
            Ciboulette2PgSafeIdentSelector::Single(ident) => ident.to_writer(&mut *writer),
            Ciboulette2PgSafeIdentSelector::Multi(idents) => {
                let mut idents_iter = idents.iter().peekable();
                while let Some(ident) = idents_iter.next() {
                    ident.to_writer(&mut *writer)?;
                    if idents_iter.peek().is_some() {
                        writer.write_all(b", ")?;
                    }
                }
                Ok(())
            }
        }
    }
}

impl std::convert::TryFrom<ArcStr> for Ciboulette2PgSafeIdent {
    type Error = Ciboulette2PgError;

    fn try_from(value: ArcStr) -> Result<Self, Self::Error> {
        Ciboulette2PgSafeIdent::check(value)
    }
}

impl std::convert::TryFrom<CibouletteIdType> for Ciboulette2PgSafeIdent {
    type Error = Ciboulette2PgError;

    fn try_from(value: CibouletteIdType) -> Result<Self, Self::Error> {
        match value {
            CibouletteIdType::Number(x) => Ciboulette2PgSafeIdent::check(x),
            CibouletteIdType::Text(x) => Ciboulette2PgSafeIdent::check(x),
            CibouletteIdType::Uuid(x) => Ciboulette2PgSafeIdent::check(x),
        }
    }
}

impl std::convert::TryFrom<&CibouletteIdType> for Ciboulette2PgSafeIdent {
    type Error = Ciboulette2PgError;

    fn try_from(value: &CibouletteIdType) -> Result<Self, Self::Error> {
        match value.clone() {
            CibouletteIdType::Number(x) => Ciboulette2PgSafeIdent::check(x),
            CibouletteIdType::Text(x) => Ciboulette2PgSafeIdent::check(x),
            CibouletteIdType::Uuid(x) => Ciboulette2PgSafeIdent::check(x),
        }
    }
}

impl std::convert::TryFrom<&ArcStr> for Ciboulette2PgSafeIdent {
    type Error = Ciboulette2PgError;

    fn try_from(value: &ArcStr) -> Result<Self, Self::Error> {
        Ciboulette2PgSafeIdent::check(value.clone())
    }
}

impl std::convert::TryFrom<&'static str> for Ciboulette2PgSafeIdent {
    type Error = Ciboulette2PgError;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        Ciboulette2PgSafeIdent::check(ArcStr::from(value))
    }
}

impl std::convert::TryFrom<String> for Ciboulette2PgSafeIdent {
    type Error = Ciboulette2PgError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ciboulette2PgSafeIdent::check(ArcStr::from(value))
    }
}

impl From<&Ciboulette2PgSafeIdent> for Ciboulette2PgSafeIdent {
    fn from(value: &Ciboulette2PgSafeIdent) -> Self {
        Ciboulette2PgSafeIdent {
            inner: value.inner.clone(),
            ..Default::default()
        }
    }
}
