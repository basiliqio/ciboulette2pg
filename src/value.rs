use super::*;
use serde_json::value::RawValue;
use sqlx::types::chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use sqlx::types::BigDecimal;
use sqlx::types::Uuid;
use sqlx::Arguments;
use sqlx::Type;
use std::convert::TryFrom;

#[derive(Clone, Debug)]
pub enum Ciboulette2SqlValue<'a> {
    Integer(Option<i64>),
    Float(Option<f32>),
    Double(Option<f64>),
    Text(Option<Cow<'a, str>>),
    Enum(Option<Cow<'a, str>>),
    Bytes(Option<Cow<'a, [u8]>>),
    Boolean(Option<bool>),
    Char(Option<char>),
    Array(Option<Vec<Ciboulette2SqlValue<'a>>>),
    Numeric(Option<BigDecimal>),
    Json(Option<serde_json::Value>),
    Xml(Option<Cow<'a, str>>),
    Uuid(Option<Uuid>),
    DateTime(Option<DateTime<Utc>>),
    Date(Option<NaiveDate>),
    Time(Option<NaiveTime>),
}

impl<'a> TryFrom<&MessyJsonValue<'a>> for Ciboulette2SqlValue<'a> {
    type Error = Ciboulette2SqlError;

    fn try_from(val: &MessyJsonValue<'a>) -> Result<Ciboulette2SqlValue<'a>, Ciboulette2SqlError> {
        Ok(match val {
            MessyJsonValue::Bool(val) => Ciboulette2SqlValue::Boolean(Some(*val)),
            MessyJsonValue::Null(schema) => match schema.as_ref() {
                MessyJson::Bool(_) => Ciboulette2SqlValue::Boolean(None),
                MessyJson::Number(_) => Ciboulette2SqlValue::Numeric(None),
                MessyJson::String(_) => Ciboulette2SqlValue::Text(None),
                MessyJson::Array(_) => Ciboulette2SqlValue::Array(None),
                MessyJson::Obj(_) => unimplemented!(),
            },
            MessyJsonValue::Number(val) => Ciboulette2SqlValue::Numeric(Some(
                bigdecimal::FromPrimitive::from_u128(*val)
                    .ok_or_else(|| Ciboulette2SqlError::BigDecimal(*val))?,
            )),
            MessyJsonValue::String(val) => Ciboulette2SqlValue::Text(Some(val.clone())),
            MessyJsonValue::Array(arr) => {
                let mut arr_res: Vec<Ciboulette2SqlValue<'_>> = Vec::with_capacity(arr.len());
                for el in arr.iter() {
                    arr_res.push(Ciboulette2SqlValue::try_from(el)?)
                }
                Ciboulette2SqlValue::Array(Some(arr_res))
            }
            MessyJsonValue::Obj(_obj) => {
                unimplemented!()
            }
        })
    }
}

impl<'a, 'q> sqlx::Encode<'q, sqlx::Postgres> for Ciboulette2SqlValue<'a> {
    #[inline]
    fn encode(
        self,
        buf: &mut <sqlx::Postgres as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        match self {
            Ciboulette2SqlValue::Integer(x) => x.encode(buf),
            Ciboulette2SqlValue::Float(x) => x.encode(buf),
            Ciboulette2SqlValue::Double(x) => x.encode(buf),
            Ciboulette2SqlValue::Boolean(x) => x.encode(buf),
            Ciboulette2SqlValue::Json(x) => x.encode(buf),
            Ciboulette2SqlValue::Text(x) => x.map(|x| x.to_string()).encode(buf),
            Ciboulette2SqlValue::Enum(x) => x.map(|x| x.to_string()).encode(buf),
            Ciboulette2SqlValue::Bytes(x) => x.map(|x| x.into_owned()).encode(buf),
            Ciboulette2SqlValue::Char(x) => x.map(|x| x.to_string()).encode(buf),
            Ciboulette2SqlValue::Array(x) => {
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
            Ciboulette2SqlValue::Numeric(x) => x.encode(buf),
            Ciboulette2SqlValue::Xml(x) => x.map(|x| x.to_string()).encode(buf),
            Ciboulette2SqlValue::Uuid(x) => x.encode(buf),
            Ciboulette2SqlValue::DateTime(x) => x.encode(buf),
            Ciboulette2SqlValue::Date(x) => x.encode(buf),
            Ciboulette2SqlValue::Time(x) => x.encode(buf),
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
            Ciboulette2SqlValue::Integer(x) => std::mem::size_of_val(&x),
            Ciboulette2SqlValue::Float(x) => std::mem::size_of_val(&x),
            Ciboulette2SqlValue::Double(x) => std::mem::size_of_val(&x),
            Ciboulette2SqlValue::Text(x) => std::mem::size_of_val(&x),
            Ciboulette2SqlValue::Enum(x) => std::mem::size_of_val(&x),
            Ciboulette2SqlValue::Bytes(x) => std::mem::size_of_val(&x),
            Ciboulette2SqlValue::Boolean(x) => std::mem::size_of_val(&x),
            Ciboulette2SqlValue::Char(x) => std::mem::size_of_val(&x),
            Ciboulette2SqlValue::Array(x) => std::mem::size_of_val(&x),
            Ciboulette2SqlValue::Numeric(x) => std::mem::size_of_val(&x),
            Ciboulette2SqlValue::Json(x) => std::mem::size_of_val(&x),
            Ciboulette2SqlValue::Xml(x) => std::mem::size_of_val(&x),
            Ciboulette2SqlValue::Uuid(x) => std::mem::size_of_val(&x),
            Ciboulette2SqlValue::DateTime(x) => std::mem::size_of_val(&x),
            Ciboulette2SqlValue::Date(x) => std::mem::size_of_val(&x),
            Ciboulette2SqlValue::Time(x) => std::mem::size_of_val(&x),
        }
    }

    #[inline]
    fn produces(&self) -> Option<sqlx::postgres::PgTypeInfo> {
        match self {
            Ciboulette2SqlValue::Integer(_) => Some(i64::type_info()),
            Ciboulette2SqlValue::Float(_) => Some(f32::type_info()),
            Ciboulette2SqlValue::Double(_) => Some(f64::type_info()),
            Ciboulette2SqlValue::Text(_) => Some(<&str>::type_info()),
            Ciboulette2SqlValue::Enum(_) => Some(<&str>::type_info()),
            Ciboulette2SqlValue::Bytes(_) => Some(<[u8]>::type_info()),
            Ciboulette2SqlValue::Boolean(_) => Some(bool::type_info()),
            Ciboulette2SqlValue::Char(_) => Some(<[u8]>::type_info()),
            Ciboulette2SqlValue::Array(_) => match self {
                Ciboulette2SqlValue::Integer(_) => Some(<[i64]>::type_info()),
                Ciboulette2SqlValue::Float(_) => Some(<[f32]>::type_info()),
                Ciboulette2SqlValue::Double(_) => Some(<[f64]>::type_info()),
                Ciboulette2SqlValue::Text(_) => Some(<[&str]>::type_info()),
                Ciboulette2SqlValue::Enum(_) => Some(<[&str]>::type_info()),
                Ciboulette2SqlValue::Bytes(_) => None,
                Ciboulette2SqlValue::Boolean(_) => Some(<[bool]>::type_info()),
                Ciboulette2SqlValue::Char(_) => Some(<[u8]>::type_info()),
                Ciboulette2SqlValue::Array(_) => None,
                Ciboulette2SqlValue::Numeric(_) => Some(<[BigDecimal]>::type_info()),
                Ciboulette2SqlValue::Json(_) => None,
                Ciboulette2SqlValue::Xml(_) => Some(<[&str]>::type_info()),
                Ciboulette2SqlValue::Uuid(_) => None,
                Ciboulette2SqlValue::DateTime(_) => Some(<[DateTime<Utc>]>::type_info()),
                Ciboulette2SqlValue::Date(_) => Some(<[NaiveDate]>::type_info()),
                Ciboulette2SqlValue::Time(_) => Some(<[NaiveTime]>::type_info()),
            },
            Ciboulette2SqlValue::Numeric(_) => Some(BigDecimal::type_info()),
            Ciboulette2SqlValue::Json(_) => Some(RawValue::type_info()),
            Ciboulette2SqlValue::Xml(_) => Some(<&str>::type_info()),
            Ciboulette2SqlValue::Uuid(_) => Some(Uuid::type_info()),
            Ciboulette2SqlValue::DateTime(_) => Some(DateTime::<Utc>::type_info()),
            Ciboulette2SqlValue::Date(_) => Some(NaiveDate::type_info()),
            Ciboulette2SqlValue::Time(_) => Some(NaiveTime::type_info()),
        }
    }
}

impl<'a> sqlx::Type<sqlx::Postgres> for Ciboulette2SqlValue<'a> {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        // Overloaded by `Encode::produce`
        <&str>::type_info()
    }
}

impl<'a, 'q> sqlx::IntoArguments<'q, sqlx::Postgres> for Ciboulette2SqlArguments<'a> {
    fn into_arguments(self) -> <sqlx::Postgres as sqlx::database::HasArguments<'q>>::Arguments {
        let mut res = sqlx::postgres::PgArguments::default();
        res.reserve(self.len(), std::mem::size_of::<Ciboulette2SqlValue>());

        for el in self.take().into_iter() {
            res.add(Ciboulette2SqlValue::from(el));
        }
        res
    }
}
