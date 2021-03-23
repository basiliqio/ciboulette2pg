use super::*;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    /// Gen an inner join between two tables
    pub(crate) fn gen_inner_join(
        buf: &mut Ciboulette2PostgresBuf,
        state: &Ciboulette2PostgresBuilderState<'a>,
        left_table: &Ciboulette2PostgresTable<'a>,
        right_table: &Ciboulette2PostgresTable<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        let left_type = left_table.ciboulette_type();
        let right_type = right_table.ciboulette_type();
        let right_type_alias = left_type.get_alias(right_type.name().as_str())?;
        let (_, opt) = state
            .store()
            .get_rel(left_type.name().as_str(), right_type_alias)?;
        match opt {
            CibouletteRelationshipOption::OneToOne(opt) => {
                Self::gen_inner_join_single_rel(&mut *buf, left_table, opt, right_table)?;
            }
            CibouletteRelationshipOption::ManyToMany(opt) => {
                Self::gen_inner_join_many_to_many_rel(
                    &mut *buf,
                    state,
                    opt,
                    right_type,
                    right_table,
                    left_table,
                    left_type,
                )?;
            }
            CibouletteRelationshipOption::OneToMany(opt) => {
                Self::gen_inner_join_one_to_many_rel_table(&mut *buf, &state, left_table, opt)?;
            }
            CibouletteRelationshipOption::ManyToOne(opt) => {
                Self::gen_inner_join_many_to_one_rel_table(&mut *buf, &state, right_table, opt)?;
            }
        }
        Ok(())
    }

    /// Gen an inner join between two tables, in case of a one-to-one relationships
    fn gen_inner_join_single_rel(
        buf: &mut Ciboulette2PostgresBuf,
        left_table: &Ciboulette2PostgresTable,
        opt: &CibouletteRelationshipOneToOneOption,
        right_table: &Ciboulette2PostgresTable,
    ) -> Result<(), Ciboulette2SqlError> {
        buf.write_all(b" INNER JOIN ")?;
        Self::write_table_info_inner(&mut *buf, &left_table)?;
        buf.write_all(b" ON ")?;
        Self::insert_ident_inner(
            &mut *buf,
            &Ciboulette2PostgresTableField::new_owned(
                Ciboulette2PostgresSafeIdent::try_from(opt.key().as_str())?,
                None,
                None,
            ),
            left_table,
            None,
        )?;
        buf.write_all(b" = ")?;
        Self::insert_ident_inner(
            &mut *buf,
            &Ciboulette2PostgresTableField::new_ref(right_table.id().get_ident(), None, None),
            right_table,
            None,
        )?;
        Ok(())
    }

    /// Gen an inner join between two tables, in case of a many-to-many relationships
    fn gen_inner_join_many_to_many_rel(
        buf: &mut Ciboulette2PostgresBuf,
        state: &Ciboulette2PostgresBuilderState<'a>,
        opt: &'a CibouletteRelationshipManyToManyOption<'a>,
        right_type: &'a CibouletteResourceType<'a>,
        right_table: &Ciboulette2PostgresTable<'a>,
        left_table: &Ciboulette2PostgresTable<'a>,
        left_type: &'a CibouletteResourceType<'a>,
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
        bucket_table: &Ciboulette2PostgresTable<'a>,
        opt: &'a CibouletteRelationshipManyToManyOption<'a>,
        right_type: &'a CibouletteResourceType<'a>,
        right_table: &Ciboulette2PostgresTable<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        buf.write_all(b" INNER JOIN ")?;
        Self::write_table_info_inner(&mut *buf, bucket_table)?;
        buf.write_all(b" ON ")?;
        Self::insert_ident_inner(
            &mut *buf,
            &Ciboulette2PostgresTableField::new_owned(
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
            &Ciboulette2PostgresTableField::new_ref(right_table.id().get_ident(), None, None),
            right_table,
            None,
        )?;
        Ok(())
    }

    /// Gen an inner join between the right table and the bucket, in case of a many-to-many relationship
    fn gen_inner_join_many_to_many_rel_table(
        buf: &mut Ciboulette2PostgresBuf,
        left_table: &Ciboulette2PostgresTable<'a>,
        opt: &'a CibouletteRelationshipManyToManyOption<'a>,
        left_type: &'a CibouletteResourceType<'a>,
        bucket_table: &Ciboulette2PostgresTable<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        buf.write_all(b" INNER JOIN ")?;
        Self::write_table_info_inner(buf, left_table)?;
        buf.write_all(b" ON ")?;
        Self::insert_ident_inner(
            buf,
            &Ciboulette2PostgresTableField::new_ref(left_table.id().get_ident(), None, None),
            left_table,
            None,
        )?;
        buf.write_all(b" = ")?;
        Self::insert_ident_inner(
            buf,
            &Ciboulette2PostgresTableField::new_owned(
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
    fn gen_inner_join_one_to_many_rel_table(
        buf: &mut Ciboulette2PostgresBuf,
        state: &Ciboulette2PostgresBuilderState<'a>,
        left_table: &Ciboulette2PostgresTable<'a>,
        opt: &'a CibouletteRelationshipOneToManyOption<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        let many_table = state.table_store().get(opt.many_table().name().as_str())?;
        buf.write_all(b" INNER JOIN ")?;
        Self::write_table_info_inner(buf, left_table)?;
        buf.write_all(b" ON ")?;
        Self::insert_ident_inner(
            buf,
            &Ciboulette2PostgresTableField::new_ref(left_table.id().get_ident(), None, None),
            left_table,
            None,
        )?;
        buf.write_all(b" = ")?;
        Self::insert_ident_inner(
            buf,
            &Ciboulette2PostgresTableField::new_owned(
                Ciboulette2PostgresSafeIdent::try_from(opt.many_table_key().as_str())?,
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
        state: &Ciboulette2PostgresBuilderState<'a>,
        right_table: &Ciboulette2PostgresTable<'a>,
        opt: &'a CibouletteRelationshipOneToManyOption<'a>,
    ) -> Result<(), Ciboulette2SqlError> {
        let many_table = state.table_store().get(opt.many_table().name().as_str())?;
        buf.write_all(b" INNER JOIN ")?;
        Self::write_table_info_inner(buf, many_table)?;
        buf.write_all(b" ON ")?;
        Self::insert_ident_inner(
            buf,
            &Ciboulette2PostgresTableField::new_ref(
                &Ciboulette2PostgresSafeIdent::try_from(opt.many_table_key().as_str())?,
                None,
                None,
            ),
            many_table,
            None,
        )?;
        buf.write_all(b" = ")?;
        Self::insert_ident_inner(
            buf,
            &Ciboulette2PostgresTableField::new_ref(right_table.id().get_ident(), None, None),
            right_table,
            None,
        )?;
        Ok(())
    }
}
