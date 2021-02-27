use super::*;
use getset::Getters;
use std::io::Write;

type Ciboulette2PostgresBuf = buf_redux::BufWriter<std::io::Cursor<Vec<u8>>>;

#[derive(Clone, Debug, Default, Getters)]
#[getset(get = "pub")]
pub struct Ciboulette2SqlArguments<'a> {
    inner: Vec<Ciboulette2SqlValue<'a>>,
}

impl<'a> Ciboulette2SqlArguments<'a> {
    pub fn take(self) -> Vec<Ciboulette2SqlValue<'a>> {
        self.inner
    }
}

impl<'a> std::ops::Deref for Ciboulette2SqlArguments<'a> {
    type Target = Vec<Ciboulette2SqlValue<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct Ciboulette2PostgresBuilder<'a> {
    buf: buf_redux::BufWriter<std::io::Cursor<Vec<u8>>>,
    params: Ciboulette2SqlArguments<'a>,
}

#[inline]
pub fn write_table_info(
    buf: &mut Ciboulette2PostgresBuf,
    table: &CibouletteTableSettings,
) -> Result<(), std::io::Error> {
    buf.write(POSTGRES_QUOTE)?;
    buf.write(table.schema.as_bytes())?;
    buf.write(b"\".\"")?;
    buf.write(table.name.as_bytes())?;
    buf.write(POSTGRES_QUOTE)?;
    Ok(())
}

pub fn gen_insert(
    buf: &mut Ciboulette2PostgresBuf,
    table: &CibouletteTableSettings,
    params: Vec<(&str, &str)>,
) -> Result<(), std::io::Error> {
    let mut param_value: Vec<&str> = Vec::with_capacity(params.len());
    let param_len = params.len();

    buf.write(b"INSERT INTO ")?;
    write_table_info(buf, table)?;
    buf.write(b" (")?;
    for (i, (n, v)) in params.into_iter().enumerate() {
        buf.write(POSTGRES_QUOTE)?;
        buf.write(n.as_bytes())?;
        buf.write(POSTGRES_QUOTE)?;
        if i < param_len - 1 {
            buf.write(b", ")?;
        }
        param_value.push(v);
    }
    buf.write(b") VALUES (")?;
    //params
    buf.write(b")");
    Ok(())
}
