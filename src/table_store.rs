use super::*;

/// Store of the available tables
#[derive(Getters, Clone, Debug, Default)]
#[getset(get = "pub")]
pub struct Ciboulette2PgTableStore {
    map: BTreeMap<ArcStr, Arc<Ciboulette2PgTable>>,
}

impl Ciboulette2PgTableStore {
    /// Add a new table
    pub fn add_table(
        &mut self,
        key: ArcStr,
        val: Arc<Ciboulette2PgTable>,
    ) {
        self.map.insert(key, val);
    }

    /// Get a table
    pub fn get(
        &self,
        key: &str,
    ) -> Result<&Arc<Ciboulette2PgTable>, Ciboulette2PgError> {
        self.map
            .get(key)
            .ok_or_else(|| Ciboulette2PgError::UnknownTable(key.to_string()))
    }
}

impl std::iter::FromIterator<(ArcStr, Arc<Ciboulette2PgTable>)> for Ciboulette2PgTableStore {
    fn from_iter<I: IntoIterator<Item = (ArcStr, Arc<Ciboulette2PgTable>)>>(
        iter: I
    ) -> Ciboulette2PgTableStore {
        Ciboulette2PgTableStore {
            map: iter.into_iter().collect(),
        }
    }
}

/// Type of table id
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Ciboulette2PgId {
    Number(Ciboulette2PgSafeIdent),
    Uuid(Ciboulette2PgSafeIdent),
    Text(Ciboulette2PgSafeIdent),
}

impl Ciboulette2PgId {
    /// Get the ident of an id
    pub fn get_ident(&self) -> &Ciboulette2PgSafeIdent {
        match self {
            Ciboulette2PgId::Number(x) => x,
            Ciboulette2PgId::Uuid(x) => x,
            Ciboulette2PgId::Text(x) => x,
        }
    }

    /// Get the type of a id
    pub fn get_type(&self) -> Ciboulette2PgSafeIdent {
        match self {
            Ciboulette2PgId::Number(_) => safe_ident::INTEGER_IDENT,
            Ciboulette2PgId::Uuid(_) => safe_ident::UUID_IDENT,
            Ciboulette2PgId::Text(_) => safe_ident::TEXT_IDENT,
        }
    }
}

impl std::convert::TryFrom<CibouletteIdType> for Ciboulette2PgId {
    type Error = Ciboulette2PgError;

    fn try_from(value: CibouletteIdType) -> Result<Self, Self::Error> {
        Ok(match value {
            CibouletteIdType::Number(name) => {
                Ciboulette2PgId::Number(Ciboulette2PgSafeIdent::try_from(name)?)
            }
            CibouletteIdType::Text(name) => {
                Ciboulette2PgId::Text(Ciboulette2PgSafeIdent::try_from(name)?)
            }
            CibouletteIdType::Uuid(name) => {
                Ciboulette2PgId::Uuid(Ciboulette2PgSafeIdent::try_from(name)?)
            }
        })
    }
}

impl std::convert::TryFrom<&CibouletteIdType> for Ciboulette2PgId {
    type Error = Ciboulette2PgError;

    fn try_from(value: &CibouletteIdType) -> Result<Self, Self::Error> {
        Ok(match value {
            CibouletteIdType::Number(name) => {
                Ciboulette2PgId::Number(Ciboulette2PgSafeIdent::try_from(name.clone())?)
            }
            CibouletteIdType::Text(name) => {
                Ciboulette2PgId::Text(Ciboulette2PgSafeIdent::try_from(name.clone())?)
            }
            CibouletteIdType::Uuid(name) => {
                Ciboulette2PgId::Uuid(Ciboulette2PgSafeIdent::try_from(name.clone())?)
            }
        })
    }
}

/// A Postgres table
#[derive(Getters, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[getset(get = "pub")]
pub struct Ciboulette2PgTable {
    id: Vec<Ciboulette2PgId>,
    schema: Option<Ciboulette2PgSafeIdent>,
    ciboulette_type: Arc<CibouletteResourceType>,
    name: Ciboulette2PgSafeIdent,
    is_cte: bool,
}

impl Ciboulette2PgTable {
    /// Create a new table
    pub fn new(
        id: Vec<Ciboulette2PgId>,
        schema: Option<Ciboulette2PgSafeIdent>,
        name: Ciboulette2PgSafeIdent,
        ciboulette_type: Arc<CibouletteResourceType>,
    ) -> Self {
        Ciboulette2PgTable {
            id,
            schema,
            name,
            ciboulette_type,
            is_cte: false,
        }
    }

    /// Create a new CTE from the current table
    pub fn to_cte(
        &self,
        builder: &mut Ciboulette2PgBuilder,
        prefix: Ciboulette2PgSafeIdent,
        suffix: Ciboulette2PgSafeIdent,
    ) -> Result<Self, Ciboulette2PgError> {
        Ok(Ciboulette2PgTable {
            id: self.id.clone(),
            ciboulette_type: self.ciboulette_type.clone(),
            schema: None,
            name: self
                .name()
                .clone()
                .add_modifier(Ciboulette2PgSafeIdentModifier::Prefix(prefix))
                .add_modifier(Ciboulette2PgSafeIdentModifier::Suffix(suffix))
                .add_modifier(Ciboulette2PgSafeIdentModifier::Index(Some(
                    builder.get_new_cte_index(),
                ))),
            is_cte: true,
        })
    }

    /// Create a new CTE
    pub fn new_cte(
        id: Vec<Ciboulette2PgId>,
        name: Ciboulette2PgSafeIdent,
        ciboulette_type: Arc<CibouletteResourceType>,
    ) -> Result<Self, Ciboulette2PgError> {
        Ok(Ciboulette2PgTable {
            id,
            schema: None,
            ciboulette_type,
            name,
            is_cte: true,
        })
    }

    pub fn to_writer(
        &self,
        writer: &mut dyn std::io::Write,
    ) -> Result<(), Ciboulette2PgError> {
        if self.is_cte {
            write!(writer, "cte_")?;
        }
        self.name().to_writer(&mut *writer)
    }
}
