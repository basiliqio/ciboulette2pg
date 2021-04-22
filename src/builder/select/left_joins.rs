use super::*;

impl<'request> Ciboulette2PostgresBuilder<'request> {
    /// Gen an LEFT JOIN between two tables
    pub(crate) fn gen_left_join(
        buf: &mut Ciboulette2PostgresBuf,
        left_table: &Ciboulette2PostgresTable,
        rel_details: &CibouletteResourceRelationshipDetails,
        right_table: &Ciboulette2PostgresTable,
    ) -> Result<(), Ciboulette2SqlError> {
        match rel_details.relation_option() {
            CibouletteRelationshipOption::ManyToOne(opt) => {
                Self::gen_left_join_many_to_one_rel_table(&mut *buf, left_table, right_table, opt)?;
            }
            _ => {
                return Err(Ciboulette2SqlError::SortingByMultiRel(
                    left_table.ciboulette_type().name().to_string(),
                    rel_details.relation_alias().to_string(),
                ))
            }
        }
        Ok(())
    }

    /// Gen an LEFT JOIN between the right table and the bucket, in case of a many-to-one relationship
    fn gen_left_join_many_to_one_rel_table(
        buf: &mut Ciboulette2PostgresBuf,
        left_table: &Ciboulette2PostgresTable,
        right_table: &Ciboulette2PostgresTable,
        opt: &CibouletteRelationshipOneToManyOption,
    ) -> Result<(), Ciboulette2SqlError> {
        buf.write_all(b" LEFT JOIN ")?;
        Self::write_table_info_inner(buf, left_table)?;
        buf.write_all(b" ON ")?;
        Self::insert_ident_inner(
            buf,
            &match left_table.is_cte() {
                true => Ciboulette2PostgresTableField::new(CIBOULETTE_MAIN_IDENTIFIER, None, None),
                false => Ciboulette2PostgresTableField::new(
                    left_table.id().get_ident().clone(),
                    None,
                    None,
                ),
            },
            left_table,
            None,
        )?;
        buf.write_all(b" = ")?;
        Self::insert_ident_inner(
            buf,
            &Ciboulette2PostgresTableField::new(
                match right_table.is_cte() {
                    true => Ciboulette2PostgresSafeIdent::try_from(opt.many_table_key())?
                        .add_modifier(Ciboulette2PostgresSafeIdentModifier::Prefix(
                            CIBOULETTE_REL_PREFIX,
                        )),
                    false => Ciboulette2PostgresSafeIdent::try_from(opt.many_table_key())?,
                },
                None,
                None,
            ),
            right_table,
            None,
        )?;
        Ok(())
    }
}
