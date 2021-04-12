use super::*;

/// Store of the available tables
#[derive(Getters, Clone, Debug, Default)]
#[getset(get = "pub")]
pub struct Ciboulette2PostgresTableStore {
    map: BTreeMap<String, Arc<Ciboulette2PostgresTable>>,
}

impl Ciboulette2PostgresTableStore {
    /// Add a new table
    pub fn add_table(
        &mut self,
        key: String,
        val: Arc<Ciboulette2PostgresTable>,
    ) {
        self.map.insert(key, val);
    }

    /// Get a table
    pub fn get(
        &self,
        key: &str,
    ) -> Result<&Arc<Ciboulette2PostgresTable>, Ciboulette2SqlError> {
        self.map
            .get(key)
            .ok_or_else(|| Ciboulette2SqlError::UnknownTable(key.to_string()))
    }
}

impl std::iter::FromIterator<(String, Arc<Ciboulette2PostgresTable>)>
    for Ciboulette2PostgresTableStore
{
    fn from_iter<I: IntoIterator<Item = (String, Arc<Ciboulette2PostgresTable>)>>(
        iter: I
    ) -> Ciboulette2PostgresTableStore {
        Ciboulette2PostgresTableStore {
            map: iter.into_iter().collect(),
        }
    }
}

/// Type of table id
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Ciboulette2PostgresId {
    Number(Ciboulette2PostgresSafeIdent),
    Uuid(Ciboulette2PostgresSafeIdent),
    Text(Ciboulette2PostgresSafeIdent),
}

impl Ciboulette2PostgresId {
    /// Get the ident of an id
    pub fn get_ident(&self) -> &Ciboulette2PostgresSafeIdent {
        match self {
            Ciboulette2PostgresId::Number(x) => x,
            Ciboulette2PostgresId::Uuid(x) => x,
            Ciboulette2PostgresId::Text(x) => x,
        }
    }

    /// Get the type of a id
    pub fn get_type(&self) -> Ciboulette2PostgresSafeIdent {
        match self {
            Ciboulette2PostgresId::Number(_) => safe_ident::INTEGER_IDENT,
            Ciboulette2PostgresId::Uuid(_) => safe_ident::UUID_IDENT,
            Ciboulette2PostgresId::Text(_) => safe_ident::TEXT_IDENT,
        }
    }

    pub fn new_from_ciboulette_id_type(
        type_: CibouletteIdType,
        name: &ArcStr,
    ) -> Result<Self, Ciboulette2SqlError> {
        Ok(match type_ {
            CibouletteIdType::Number => {
                Ciboulette2PostgresId::Number(Ciboulette2PostgresSafeIdent::try_from(name.clone())?)
            }
            CibouletteIdType::Text => {
                Ciboulette2PostgresId::Text(Ciboulette2PostgresSafeIdent::try_from(name.clone())?)
            }
            CibouletteIdType::Uuid => {
                Ciboulette2PostgresId::Uuid(Ciboulette2PostgresSafeIdent::try_from(name.clone())?)
            }
        })
    }
}

/// A Postgres table
#[derive(Getters, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[getset(get = "pub")]
pub struct Ciboulette2PostgresTable {
    id: Ciboulette2PostgresId,
    schema: Option<Ciboulette2PostgresSafeIdent>,
    ciboulette_type: Arc<CibouletteResourceType>,
    name: Ciboulette2PostgresSafeIdent,
    is_cte: bool,
}

impl Ciboulette2PostgresTable {
    /// Create a new table
    pub fn new(
        id: Ciboulette2PostgresId,
        schema: Option<Ciboulette2PostgresSafeIdent>,
        name: Ciboulette2PostgresSafeIdent,
        ciboulette_type: Arc<CibouletteResourceType>,
    ) -> Self {
        Ciboulette2PostgresTable {
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
        prefix: Ciboulette2PostgresSafeIdent,
        suffix: Ciboulette2PostgresSafeIdent,
    ) -> Result<Self, Ciboulette2SqlError> {
        Ok(Ciboulette2PostgresTable {
            id: self.id.clone(),
            ciboulette_type: self.ciboulette_type.clone(),
            schema: None,
            name: self
                .name()
                .clone()
                .add_modifier(Ciboulette2PostgresSafeIdentModifier::Prefix(prefix))
                .add_modifier(Ciboulette2PostgresSafeIdentModifier::Suffix(suffix)),
            is_cte: true,
        })
    }

    /// Create a new CTE
    pub fn new_cte(
        id: Ciboulette2PostgresId,
        name: Ciboulette2PostgresSafeIdent,
        ciboulette_type: Arc<CibouletteResourceType>,
    ) -> Result<Self, Ciboulette2SqlError> {
        Ok(Ciboulette2PostgresTable {
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
    ) -> Result<(), Ciboulette2SqlError> {
        if self.is_cte {
            write!(writer, "cte_")?;
        }
        self.name().to_writer(&mut *writer)
    }
}
