use super::*;

impl<'request> Ciboulette2PostgresBuilder<'request> {
    /// Gen an inner join between two tables
    pub(crate) fn gen_inner_join<'store>(
        buf: &mut Ciboulette2PostgresBuf,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        left_table: &Ciboulette2PostgresTable,
        rel_details: &CibouletteResourceRelationshipDetails,
        right_table_override: Option<&Ciboulette2PostgresTable>,
    ) -> Result<(), Ciboulette2SqlError> {
        let left_type = left_table.ciboulette_type();
        let right_table = match right_table_override {
            Some(x) => x,
            None => state.table_store().get(rel_details.related_type().name())?,
        };
        match rel_details.relation_option() {
            CibouletteRelationshipOption::ManyToMany(opt) => {
                Self::gen_inner_join_many_to_many_rel(
                    &mut *buf,
                    state,
                    opt,
                    rel_details.related_type().clone(),
                    right_table,
                    left_table,
                    left_type.clone(),
                )?;
            }
            CibouletteRelationshipOption::OneToMany(opt) => {
                Self::gen_inner_join_one_to_many_rel_table(&mut *buf, &state, left_table, opt)?;
            }
            CibouletteRelationshipOption::ManyToOne(opt) => {
                Self::gen_inner_join_many_to_one_rel_table(
                    &mut *buf,
                    left_table,
                    right_table,
                    opt,
                )?;
            }
        }
        Ok(())
    }

    /// Gen an inner join between two tables, in case of a many-to-many relationships
    fn gen_inner_join_many_to_many_rel<'store>(
        buf: &mut Ciboulette2PostgresBuf,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        opt: &CibouletteRelationshipManyToManyOption,
        right_type: Arc<CibouletteResourceType>,
        right_table: &Ciboulette2PostgresTable,
        left_table: &Ciboulette2PostgresTable,
        left_type: Arc<CibouletteResourceType>,
    ) -> Result<(), Ciboulette2SqlError> {
        let bucket_table = state
            .table_store()
            .get(opt.bucket_resource().name().as_str())?;
        Self::gen_inner_join_multi_rel_rel_table(
            &mut *buf,
            bucket_table,
            opt,
            right_type,
            right_table,
        )?;
        Self::gen_inner_join_many_to_many_rel_table(
            &mut *buf,
            left_table,
            opt,
            left_type,
            bucket_table,
        )?;
        Ok(())
    }

    /// Gen an inner join between the left table and the bucket, in case of a one-to-many relationship
    fn gen_inner_join_multi_rel_rel_table(
        buf: &mut Ciboulette2PostgresBuf,
        bucket_table: &Ciboulette2PostgresTable,
        opt: &CibouletteRelationshipManyToManyOption,
        right_type: Arc<CibouletteResourceType>,
        right_table: &Ciboulette2PostgresTable,
    ) -> Result<(), Ciboulette2SqlError> {
        buf.write_all(b" INNER JOIN ")?;
        Self::write_table_info_inner(&mut *buf, bucket_table)?;
        buf.write_all(b" ON ")?;
        Self::insert_ident_inner(
            &mut *buf,
            &Ciboulette2PostgresTableField::new(
                Ciboulette2PostgresSafeIdent::try_from(opt.keys_for_type(&right_type)?)?,
                None,
                None,
            ),
            bucket_table,
            None,
        )?;
        buf.write_all(b" = ")?;
        Self::insert_ident_inner(
            &mut *buf,
            &match right_table.is_cte() {
                true => Ciboulette2PostgresTableField::new(CIBOULETTE_MAIN_IDENTIFIER, None, None),
                false => Ciboulette2PostgresTableField::new(
                    right_table.id().get_ident().clone(),
                    None,
                    None,
                ),
            },
            right_table,
            None,
        )?;
        Ok(())
    }

    /// Gen an inner join between the right table and the bucket, in case of a many-to-many relationship
    fn gen_inner_join_many_to_many_rel_table(
        buf: &mut Ciboulette2PostgresBuf,
        left_table: &Ciboulette2PostgresTable,
        opt: &CibouletteRelationshipManyToManyOption,
        left_type: Arc<CibouletteResourceType>,
        bucket_table: &Ciboulette2PostgresTable,
    ) -> Result<(), Ciboulette2SqlError> {
        buf.write_all(b" INNER JOIN ")?;
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
                Ciboulette2PostgresSafeIdent::try_from(opt.keys_for_type(&left_type)?)?,
                None,
                None,
            ),
            bucket_table,
            None,
        )?;
        Ok(())
    }

    /// Gen an inner join between the right table and the bucket, in case of a one-to-many relationship
    fn gen_inner_join_one_to_many_rel_table<'store>(
        buf: &mut Ciboulette2PostgresBuf,
        state: &Ciboulette2PostgresBuilderState<'store, 'request>,
        left_table: &Ciboulette2PostgresTable,
        opt: &CibouletteRelationshipOneToManyOption,
    ) -> Result<(), Ciboulette2SqlError> {
        let many_table = state.table_store().get(opt.many_table().name().as_str())?;
        buf.write_all(b" INNER JOIN ")?;
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
                Ciboulette2PostgresSafeIdent::try_from(opt.many_table_key())?,
                None,
                None,
            ),
            many_table,
            None,
        )?;
        Ok(())
    }

    /// Gen an inner join between the right table and the bucket, in case of a many-to-one relationship
    fn gen_inner_join_many_to_one_rel_table(
        buf: &mut Ciboulette2PostgresBuf,
        left_table: &Ciboulette2PostgresTable,
        right_table: &Ciboulette2PostgresTable,
        opt: &CibouletteRelationshipOneToManyOption,
    ) -> Result<(), Ciboulette2SqlError> {
        buf.write_all(b" INNER JOIN ")?;
        Self::write_table_info_inner(buf, left_table)?;
        buf.write_all(b" ON ")?;
        Self::insert_ident_inner(
            buf,
            &Ciboulette2PostgresTableField::new(
                match left_table.is_cte() {
                    true => Ciboulette2PostgresSafeIdent::try_from(opt.many_table_key())?
                        .add_modifier(Ciboulette2PostgresSafeIdentModifier::Prefix(
                            CIBOULETTE_REL_PREFIX,
                        )),
                    false => Ciboulette2PostgresSafeIdent::try_from(opt.many_table_key())?,
                },
                None,
                None,
            ),
            left_table,
            None,
        )?;
        buf.write_all(b" = ")?;
        Self::insert_ident_inner(
            buf,
            &match right_table.is_cte() {
                true => Ciboulette2PostgresTableField::new(CIBOULETTE_MAIN_IDENTIFIER, None, None),
                false => Ciboulette2PostgresTableField::new(
                    right_table.id().get_ident().clone(),
                    None,
                    None,
                ),
            },
            right_table,
            None,
        )?;
        Ok(())
    }
}
