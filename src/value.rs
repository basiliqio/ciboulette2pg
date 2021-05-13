use super::*;
use bigdecimal::FromPrimitive;
use serde_json::value::RawValue;
use sqlx::types::chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use sqlx::types::BigDecimal;
use sqlx::types::Uuid;
use sqlx::Arguments;
use sqlx::Type;
use std::convert::TryFrom;
use std::ops::Deref;

/// An [SQLx](sqlx) compatible value that'll contains the parameters of the queries
#[derive(Clone, Debug)]
pub enum Ciboulette2PgValue<'request> {
    Integer(Option<i64>),
    Float(Option<f32>),
    Double(Option<f64>),
    Text(Option<Cow<'request, str>>),
    Enum(Option<Cow<'request, str>>),
    Bytes(Option<Cow<'request, [u8]>>),
    Boolean(Option<bool>),
    Char(Option<char>),
    Array(Option<Vec<Ciboulette2PgValue<'request>>>),
    Numeric(Option<BigDecimal>),
    Json(Option<serde_json::Value>),
    Xml(Option<Cow<'request, str>>),
    Uuid(Option<Uuid>),
    DateTime(Option<DateTime<Utc>>),
    Date(Option<NaiveDate>),
    Time(Option<NaiveTime>),
    ArcStr(Option<ArcStr>),
}

impl<'request> Ciboulette2PgValue<'request> {
    pub fn from_id_selector(
        id_selector: &CibouletteIdSelector<'request>
    ) -> Vec<Ciboulette2PgValue<'request>> {
        match id_selector {
            CibouletteIdSelector::Single(x) => vec![Ciboulette2PgValue::from(x)],
            CibouletteIdSelector::Multi(x) => x.iter().map(Ciboulette2PgValue::from).collect(),
        }
    }

    pub fn from_id_type_selector(
        id_type_selector: &CibouletteIdTypeSelector
    ) -> Vec<Ciboulette2PgValue<'request>> {
        match id_type_selector {
            CibouletteIdTypeSelector::Single(x) => vec![Ciboulette2PgValue::from(x)],
            CibouletteIdTypeSelector::Multi(x) => x.iter().map(Ciboulette2PgValue::from).collect(),
        }
    }
}

impl<'store> TryFrom<&MessyJsonValue<'store>> for Ciboulette2PgValue<'store> {
    type Error = Ciboulette2PgError;

    fn try_from(
        val: &MessyJsonValue<'store>
    ) -> Result<Ciboulette2PgValue<'store>, Ciboulette2PgError> {
        Ok(match val {
            MessyJsonValue::Bool(val) => Ciboulette2PgValue::Boolean(Some(*val)),
            MessyJsonValue::Null(_, schema) => match schema {
                MessyJsonExpected::Root(root) => match root.deref() {
                    MessyJsonInner::Bool(_) => Ciboulette2PgValue::Boolean(None),
                    MessyJsonInner::Number(_) => Ciboulette2PgValue::Numeric(None),
                    MessyJsonInner::String(_) => Ciboulette2PgValue::Text(None),
                    MessyJsonInner::Array(_) => Ciboulette2PgValue::Array(None),
                    MessyJsonInner::Uuid(_) => Ciboulette2PgValue::Uuid(None),
                    MessyJsonInner::Obj(_) => unimplemented!(),
                },

                MessyJsonExpected::Obj(_) => unimplemented!(), // FIXME
            },
            MessyJsonValue::Number(val) => Ciboulette2PgValue::Numeric(Some(
                bigdecimal::FromPrimitive::from_u128(*val)
                    .ok_or_else(|| Ciboulette2PgError::BigDecimal(*val))?,
            )),
            MessyJsonValue::String(val) => Ciboulette2PgValue::Text(Some(val.clone())),
            MessyJsonValue::Uuid(val) => Ciboulette2PgValue::Uuid(Some(**val)),
            MessyJsonValue::Array(arr) => {
                let mut arr_res: Vec<Ciboulette2PgValue<'_>> = Vec::with_capacity(arr.len());
                for el in arr.iter() {
                    arr_res.push(Ciboulette2PgValue::try_from(el)?)
                }
                Ciboulette2PgValue::Array(Some(arr_res))
            }
            MessyJsonValue::Obj(_obj) => {
                unimplemented!() //TODO better
            }
        })
    }
}

impl<'request> From<&CibouletteId<'request>> for Ciboulette2PgValue<'request> {
    fn from(val: &CibouletteId<'request>) -> Ciboulette2PgValue<'request> {
        match val {
            CibouletteId::Number(x) => Ciboulette2PgValue::Numeric(BigDecimal::from_u64(*x)),
            CibouletteId::Text(x) => Ciboulette2PgValue::Text(Some(x.clone())),
            CibouletteId::Uuid(x) => Ciboulette2PgValue::Uuid(Some(*x)),
        }
    }
}

impl<'request> From<CibouletteIdType> for Ciboulette2PgValue<'request> {
    fn from(val: CibouletteIdType) -> Ciboulette2PgValue<'request> {
        match val {
            CibouletteIdType::Number(_) => Ciboulette2PgValue::Numeric(None),
            CibouletteIdType::Text(_) => Ciboulette2PgValue::Text(None),
            CibouletteIdType::Uuid(_) => Ciboulette2PgValue::Uuid(None),
        }
    }
}

impl<'request> From<&CibouletteIdType> for Ciboulette2PgValue<'request> {
    fn from(val: &CibouletteIdType) -> Ciboulette2PgValue<'request> {
        match val {
            CibouletteIdType::Number(_) => Ciboulette2PgValue::Numeric(None),
            CibouletteIdType::Text(_) => Ciboulette2PgValue::Text(None),
            CibouletteIdType::Uuid(_) => Ciboulette2PgValue::Uuid(None),
        }
    }
}

impl<'request> From<&'request str> for Ciboulette2PgValue<'request> {
    fn from(val: &'request str) -> Ciboulette2PgValue<'request> {
        Ciboulette2PgValue::Text(Some(Cow::Borrowed(val)))
    }
}

impl<'request> From<Cow<'request, str>> for Ciboulette2PgValue<'request> {
    fn from(val: Cow<'request, str>) -> Ciboulette2PgValue<'request> {
        Ciboulette2PgValue::Text(Some(val))
    }
}

impl<'store, 'q> sqlx::Encode<'q, sqlx::Postgres> for Ciboulette2PgValue<'store> {
    #[inline]
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        match self {
            Ciboulette2PgValue::Integer(x) => x.encode(buf),
            Ciboulette2PgValue::Float(x) => x.encode(buf),
            Ciboulette2PgValue::Double(x) => x.encode(buf),
            Ciboulette2PgValue::Boolean(x) => x.encode(buf),
            Ciboulette2PgValue::Json(x) => x.encode(buf),
            Ciboulette2PgValue::Text(x) => x.map(|x| x.to_string()).encode(buf),
            Ciboulette2PgValue::ArcStr(x) => x.map(|x| x.to_string()).encode(buf),
            Ciboulette2PgValue::Enum(x) => x.map(|x| x.to_string()).encode(buf),
            Ciboulette2PgValue::Bytes(x) => x.map(|x| x.into_owned()).encode(buf),
            Ciboulette2PgValue::Char(x) => x.map(|x| x.to_string()).encode(buf),
            Ciboulette2PgValue::Array(x) => {
                let mut res: sqlx::encode::IsNull = sqlx::encode::IsNull::Yes;
                if let Some(x) = x {
                    for i in x.into_iter() {
                        if matches!(i.encode(buf), sqlx::encode::IsNull::No) {
                            res = sqlx::encode::IsNull::No;
                        }
                    }
                }
                res
            }
            Ciboulette2PgValue::Numeric(x) => x.encode(buf),
            Ciboulette2PgValue::Xml(x) => x.map(|x| x.to_string()).encode(buf),
            Ciboulette2PgValue::Uuid(x) => x.encode(buf),
            Ciboulette2PgValue::DateTime(x) => x.encode(buf),
            Ciboulette2PgValue::Date(x) => x.encode(buf),
            Ciboulette2PgValue::Time(x) => x.encode(buf),
        }
    }

    #[inline]
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        self.encode(buf)
    }

    #[inline]
    fn size_hint(&self) -> usize {
        match self {
            Ciboulette2PgValue::Integer(x) => std::mem::size_of_val(&x),
            Ciboulette2PgValue::Float(x) => std::mem::size_of_val(&x),
            Ciboulette2PgValue::Double(x) => std::mem::size_of_val(&x),
            Ciboulette2PgValue::Text(x) => std::mem::size_of_val(&x),
            Ciboulette2PgValue::ArcStr(x) => std::mem::size_of_val(&x),
            Ciboulette2PgValue::Enum(x) => std::mem::size_of_val(&x),
            Ciboulette2PgValue::Bytes(x) => std::mem::size_of_val(&x),
            Ciboulette2PgValue::Boolean(x) => std::mem::size_of_val(&x),
            Ciboulette2PgValue::Char(x) => std::mem::size_of_val(&x),
            Ciboulette2PgValue::Array(x) => std::mem::size_of_val(&x),
            Ciboulette2PgValue::Numeric(x) => std::mem::size_of_val(&x),
            Ciboulette2PgValue::Json(x) => std::mem::size_of_val(&x),
            Ciboulette2PgValue::Xml(x) => std::mem::size_of_val(&x),
            Ciboulette2PgValue::Uuid(x) => std::mem::size_of_val(&x),
            Ciboulette2PgValue::DateTime(x) => std::mem::size_of_val(&x),
            Ciboulette2PgValue::Date(x) => std::mem::size_of_val(&x),
            Ciboulette2PgValue::Time(x) => std::mem::size_of_val(&x),
        }
    }

    #[inline]
    fn produces(&self) -> Option<sqlx::postgres::PgTypeInfo> {
        match self {
            Ciboulette2PgValue::Integer(_) => Some(i64::type_info()),
            Ciboulette2PgValue::Float(_) => Some(f32::type_info()),
            Ciboulette2PgValue::Double(_) => Some(f64::type_info()),
            Ciboulette2PgValue::ArcStr(_) => Some(<&str>::type_info()),
            Ciboulette2PgValue::Text(_) => Some(<&str>::type_info()),
            Ciboulette2PgValue::Enum(_) => Some(<&str>::type_info()),
            Ciboulette2PgValue::Bytes(_) => Some(<[u8]>::type_info()),
            Ciboulette2PgValue::Boolean(_) => Some(bool::type_info()),
            Ciboulette2PgValue::Char(_) => Some(<[u8]>::type_info()),
            Ciboulette2PgValue::Array(_) => match self {
                Ciboulette2PgValue::Integer(_) => Some(<[i64]>::type_info()),
                Ciboulette2PgValue::Float(_) => Some(<[f32]>::type_info()),
                Ciboulette2PgValue::Double(_) => Some(<[f64]>::type_info()),
                Ciboulette2PgValue::ArcStr(_) => Some(<[&str]>::type_info()),
                Ciboulette2PgValue::Text(_) => Some(<[&str]>::type_info()),
                Ciboulette2PgValue::Enum(_) => Some(<[&str]>::type_info()),
                Ciboulette2PgValue::Bytes(_) => None,
                Ciboulette2PgValue::Boolean(_) => Some(<[bool]>::type_info()),
                Ciboulette2PgValue::Char(_) => Some(<[u8]>::type_info()),
                Ciboulette2PgValue::Array(_) => None,
                Ciboulette2PgValue::Numeric(_) => Some(<[BigDecimal]>::type_info()),
                Ciboulette2PgValue::Json(_) => None,
                Ciboulette2PgValue::Xml(_) => Some(<[&str]>::type_info()),
                Ciboulette2PgValue::Uuid(_) => None,
                Ciboulette2PgValue::DateTime(_) => Some(<[DateTime<Utc>]>::type_info()),
                Ciboulette2PgValue::Date(_) => Some(<[NaiveDate]>::type_info()),
                Ciboulette2PgValue::Time(_) => Some(<[NaiveTime]>::type_info()),
            },
            Ciboulette2PgValue::Numeric(_) => Some(BigDecimal::type_info()),
            Ciboulette2PgValue::Json(_) => Some(RawValue::type_info()),
            Ciboulette2PgValue::Xml(_) => Some(<&str>::type_info()),
            Ciboulette2PgValue::Uuid(_) => Some(Uuid::type_info()),
            Ciboulette2PgValue::DateTime(_) => Some(DateTime::<Utc>::type_info()),
            Ciboulette2PgValue::Date(_) => Some(NaiveDate::type_info()),
            Ciboulette2PgValue::Time(_) => Some(NaiveTime::type_info()),
        }
    }
}

impl<'store> sqlx::Type<sqlx::Postgres> for Ciboulette2PgValue<'store> {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        // Overloaded by `Encode::produce`
        <&str>::type_info()
    }
}

impl<'store, 'q> sqlx::IntoArguments<'q, sqlx::Postgres> for Ciboulette2PgArguments<'store> {
    fn into_arguments(self) -> <sqlx::Postgres as sqlx::database::HasArguments<'q>>::Arguments {
        let mut res = sqlx::postgres::PgArguments::default();
        res.reserve(self.len(), std::mem::size_of::<Ciboulette2PgValue>());

        for el in self.take().into_iter() {
            res.add(el);
        }
        res
    }
}
