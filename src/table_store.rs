use super::*;

/// Store of the available tables
#[derive(Getters, Clone, Debug, Default)]
#[getset(get = "pub")]
pub struct Ciboulette2PostgresTableStore<'store> {
    map: BTreeMap<String, Arc<Ciboulette2PostgresTable<'store>>>,
}

impl<'store> Ciboulette2PostgresTableStore<'store> {
    /// Add a new table
    pub fn add_table(
        &mut self,
        key: String,
        val: Arc<Ciboulette2PostgresTable<'store>>,
    ) {
        self.map.insert(key, val);
    }

    /// Get a table
    pub fn get(
        &self,
        key: &str,
    ) -> Result<&Ciboulette2PostgresTable<'store>, Ciboulette2SqlError> {
        self.map
            .get(key)
            .map(Arc::as_ref)
            .ok_or_else(|| Ciboulette2SqlError::UnknownTable(key.to_string()))
    }
}

impl<'store> std::iter::FromIterator<(String, Arc<Ciboulette2PostgresTable<'store>>)>
    for Ciboulette2PostgresTableStore<'store>
{
    fn from_iter<I: IntoIterator<Item = (String, Arc<Ciboulette2PostgresTable<'store>>)>>(
        iter: I
    ) -> Ciboulette2PostgresTableStore<'store> {
        Ciboulette2PostgresTableStore {
            map: iter.into_iter().collect(),
        }
    }
}

/// Type of table id
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Ciboulette2PostgresId<'store> {
    Number(Ciboulette2PostgresSafeIdent<'store>),
    Uuid(Ciboulette2PostgresSafeIdent<'store>),
    Text(Ciboulette2PostgresSafeIdent<'store>),
}

impl<'store> Ciboulette2PostgresId<'store> {
    /// Get the ident of an id
    pub fn get_ident(&self) -> &Ciboulette2PostgresSafeIdent<'store> {
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

    pub fn new_from_ciboulette_id_type(
        type_: CibouletteIdType,
        name: &str,
    ) -> Result<Self, Ciboulette2SqlError> {
        Ok(match type_ {
            CibouletteIdType::Number => Ciboulette2PostgresId::Number(
                Ciboulette2PostgresSafeIdent::try_from(name.to_string())?,
            ),
            CibouletteIdType::Text => Ciboulette2PostgresId::Text(
                Ciboulette2PostgresSafeIdent::try_from(name.to_string())?,
            ),
            CibouletteIdType::Uuid => Ciboulette2PostgresId::Uuid(
                Ciboulette2PostgresSafeIdent::try_from(name.to_string())?,
            ),
        })
    }
}

/// A Postgres table
#[derive(Getters, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[getset(get = "pub")]
pub struct Ciboulette2PostgresTable<'store> {
    id: Ciboulette2PostgresId<'store>,
    schema: Option<Ciboulette2PostgresSafeIdent<'store>>,
    ciboulette_type: Arc<CibouletteResourceType<'store>>,
    name: Ciboulette2PostgresSafeIdent<'store>,
}

impl<'store> Ciboulette2PostgresTable<'store> {
    /// Create a new table
    pub fn new(
        id: Ciboulette2PostgresId<'store>,
        schema: Option<Ciboulette2PostgresSafeIdent<'store>>,
        name: Ciboulette2PostgresSafeIdent<'store>,
        ciboulette_type: Arc<CibouletteResourceType<'store>>,
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
        &'store self,
        name: Cow<'store, str>,
    ) -> Result<Self, Ciboulette2SqlError> {
        Ok(Ciboulette2PostgresTable {
            id: self.id.clone(),
            ciboulette_type: self.ciboulette_type.clone(),
            schema: None,
            name: Ciboulette2PostgresSafeIdent::try_from(name)?,
        })
    }

    /// Create a new CTE
    pub fn new_cte(
        id: Ciboulette2PostgresId<'store>,
        name: Cow<'store, str>,
        ciboulette_type: Arc<CibouletteResourceType<'store>>,
    ) -> Result<Self, Ciboulette2SqlError> {
        Ok(Ciboulette2PostgresTable {
            id: id.clone(),
            schema: None,
            ciboulette_type,
            name: Ciboulette2PostgresSafeIdent::try_from(name)?,
        })
    }
}
