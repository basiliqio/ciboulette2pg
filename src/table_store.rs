use super::*;

#[derive(Getters, Clone, Debug, Default)]
#[getset(get = "pub")]
pub struct Ciboulette2PostgresTableStore<'a> {
    map: BTreeMap<String, Ciboulette2PostgresTableSettings<'a>>,
}

impl<'a> Ciboulette2PostgresTableStore<'a> {
    pub fn add_table(&mut self, key: String, val: Ciboulette2PostgresTableSettings<'a>) {
        self.map.insert(key, val);
    }

    pub fn get(
        &self,
        key: &str,
    ) -> Result<&Ciboulette2PostgresTableSettings<'a>, Ciboulette2SqlError> {
        self.map
            .get(key)
            .ok_or_else(|| Ciboulette2SqlError::UnknownTable(key.to_string()))
    }
}

impl<'a> std::iter::FromIterator<(String, Ciboulette2PostgresTableSettings<'a>)>
    for Ciboulette2PostgresTableStore<'a>
{
    fn from_iter<I: IntoIterator<Item = (String, Ciboulette2PostgresTableSettings<'a>)>>(
        iter: I,
    ) -> Ciboulette2PostgresTableStore<'a> {
        Ciboulette2PostgresTableStore {
            map: iter.into_iter().collect(),
        }
    }
}

#[derive(Getters, Clone, Debug, Default)]
#[getset(get = "pub")]
pub struct Ciboulette2PostgresTableSettings<'a> {
    id_name: Cow<'a, str>,
    id_type: Cow<'a, str>,
    schema: Option<Cow<'a, str>>,
    name: Cow<'a, str>,
}

impl<'a> Ciboulette2PostgresTableSettings<'a> {
    pub fn new(
        id_name: Cow<'a, str>,
        id_type: Cow<'a, str>,
        schema: Option<Cow<'a, str>>,
        name: Cow<'a, str>,
    ) -> Self {
        Ciboulette2PostgresTableSettings {
            id_name,
            id_type,
            schema,
            name,
        }
    }

    pub fn new_cte(id_name: Cow<'a, str>, id_type: Cow<'a, str>, name: Cow<'a, str>) -> Self {
        Ciboulette2PostgresTableSettings {
            id_name,
            id_type,
            schema: None,
            name,
        }
    }
}
