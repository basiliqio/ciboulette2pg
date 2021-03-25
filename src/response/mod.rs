use super::*;
pub mod response_type;

pub fn gen_response<'a, T>(
    ciboulette_store: &'a CibouletteStore<'a>,
    ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
    request: &'a T,
) -> Result<CibouletteOutboundRequest<'a>, Ciboulette2SqlError>
where
    T: CibouletteInboundRequestCommons<'a>,
{
    todo!()
}
