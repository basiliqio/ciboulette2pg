use super::*;

/// Store of the available tables
#[derive(Getters, Clone, Debug, Default)]
#[getset(get = "pub")]
pub struct Ciboulette2PostgresTableStore<'a> {
    map: BTreeMap<String, Ciboulette2PostgresTable<'a>>,
}

impl<'a> Ciboulette2PostgresTableStore<'a> {
    /// Add a new table
    pub fn add_table(
        &mut self,
        key: String,
        val: Ciboulette2PostgresTable<'a>,
    ) {
        self.map.insert(key, val);
    }

    /// Get a table
    pub fn get(
        &self,
        key: &str,
    ) -> Result<&Ciboulette2PostgresTable<'a>, Ciboulette2SqlError> {
        self.map
            .get(key)
            .ok_or_else(|| Ciboulette2SqlError::UnknownTable(key.to_string()))
    }
}

impl<'a> std::iter::FromIterator<(String, Ciboulette2PostgresTable<'a>)>
    for Ciboulette2PostgresTableStore<'a>
{
    fn from_iter<I: IntoIterator<Item = (String, Ciboulette2PostgresTable<'a>)>>(
        iter: I
    ) -> Ciboulette2PostgresTableStore<'a> {
        Ciboulette2PostgresTableStore {
            map: iter.into_iter().collect(),
        }
    }
}

/// Type of table id
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Ciboulette2PostgresId<'a> {
    Number(Ciboulette2PostgresSafeIdent<'a>),
    Uuid(Ciboulette2PostgresSafeIdent<'a>),
    Text(Ciboulette2PostgresSafeIdent<'a>),
}

impl<'a> Ciboulette2PostgresId<'a> {
    /// Get the ident of an id
    pub fn get_ident(&self) -> &Ciboulette2PostgresSafeIdent<'a> {
        match self {
            Ciboulette2PostgresId::Number(x) => x,
            Ciboulette2PostgresId::Uuid(x) => x,
            Ciboulette2PostgresId::Text(x) => x,
        }
    }

    /// Get the type of a id
    pub fn get_type(&self) -> &Ciboulette2PostgresSafeIdent<'static> {
        match self {
            Ciboulette2PostgresId::Number(_) => &safe_ident::INTEGER_IDENT,
            Ciboulette2PostgresId::Uuid(_) => &safe_ident::UUID_IDENT,
            Ciboulette2PostgresId::Text(_) => &safe_ident::TEXT_IDENT,
        }
    }
}

/// A Postgres table
#[derive(Getters, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[getset(get = "pub")]
pub struct Ciboulette2PostgresTable<'a> {
    id: Ciboulette2PostgresId<'a>,
    schema: Option<Ciboulette2PostgresSafeIdent<'a>>,
    ciboulette_type: &'a CibouletteResourceType<'a>,
    name: Ciboulette2PostgresSafeIdent<'a>,
}

impl<'a> Ciboulette2PostgresTable<'a> {
    /// Create a new table
    pub fn new(
        id: Ciboulette2PostgresId<'a>,
        schema: Option<Ciboulette2PostgresSafeIdent<'a>>,
        name: Ciboulette2PostgresSafeIdent<'a>,
        ciboulette_type: &'a CibouletteResourceType<'a>,
    ) -> Self {
        Ciboulette2PostgresTable {
            id,
            schema,
            name,
            ciboulette_type,
        }
    }

    /// Create a new CTE from the current table
    pub fn to_cte(
        &'a self,
        name: Cow<'a, str>,
    ) -> Result<Self, Ciboulette2SqlError> {
        Ok(Ciboulette2PostgresTable {
            id: self.id.clone(),
            ciboulette_type: self.ciboulette_type,
            schema: None,
            name: Ciboulette2PostgresSafeIdent::try_from(name)?,
        })
    }

    /// Create a new CTE
    pub fn new_cte(
        id: Ciboulette2PostgresId<'a>,
        name: Cow<'a, str>,
        ciboulette_type: &'a CibouletteResourceType<'a>,
    ) -> Result<Self, Ciboulette2SqlError> {
        Ok(Ciboulette2PostgresTable {
            id: id.clone(),
            schema: None,
            ciboulette_type,
            name: Ciboulette2PostgresSafeIdent::try_from(name)?,
        })
    }
}
