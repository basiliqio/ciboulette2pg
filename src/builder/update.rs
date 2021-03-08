use super::*;
use crate::graph_walker::main::Ciboulette2PostgresMain;

impl<'a> Ciboulette2PostgresBuilder<'a> {
    fn gen_update_normal(
        &mut self,
        table: &Ciboulette2PostgresTableSettings,
        params: Vec<(&str, Ciboulette2SqlValue<'a>)>,
        query: &'a CibouletteUpdateRequest<'a>,
        returning: bool,
    ) -> Result<(), Ciboulette2SqlError> {
        self.buf.write_all(b"UPDATE ")?;
        self.write_table_info(table)?;
        self.buf.write_all(b" SET ")?;
        let mut iter = params.into_iter().peekable();
        while let Some((n, v)) = iter.next() {
            self.insert_ident(
                &(Ciboulette2PostgresSafeIdent::try_from(n)?, None, None),
                &table,
            )?;
            self.buf.write_all(b" = ")?;
            self.insert_params(v, &table)?;

            if iter.peek().is_some() {
                self.buf.write_all(b", ")?;
            }
        }
        self.buf.write_all(b" WHERE ")?;
        self.insert_ident(&(table.id_name().clone(), None, None), &table)?;
        self.buf.write_all(b" = ")?;
        self.insert_params(
            Ciboulette2SqlValue::Text(Some(Cow::Borrowed(query.resource_id().as_ref()))),
            &table,
        )?;
        if returning {
            self.buf.write_all(b" RETURNING *")?;
        }
        Ok(())
    }

    pub fn gen_update_main(
        ciboulette_store: &'a CibouletteStore<'a>,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        request: &'a CibouletteUpdateRequest<'a>,
    ) -> Result<Self, Ciboulette2SqlError> {
        let mut se = Self::default();
        let mut table_list: Vec<Ciboulette2PostgresTableSettings<'_>> = Vec::with_capacity(128);
        let main_type = request.resource_type();
        let main_attrs = match request.data() {
            CibouletteUpdateRequestType::MainType(attr) => attr,
            CibouletteUpdateRequestType::Relationship(_) => {
                return Err(Ciboulette2SqlError::UpdatingRelationships)
            }
        };
        let main_table = ciboulette_table_store.get(main_type.name().as_str())?;
        let main_cte_update =
            main_table.to_cte(Cow::Owned(format!("cte_{}_update", main_table.name())))?;
        let main_cte_data =
            main_table.to_cte(Cow::Owned(format!("cte_{}_data", main_table.name())))?;
        table_list.push(main_cte_data.clone());
        // WITH
        se.buf.write_all(b"WITH \n")?;
        // WITH "cte_main_update"
        se.write_table_info(&main_cte_update)?;
        se.buf.write_all(b" AS (")?;
        let Ciboulette2PostgresMain {
            insert_values: main_update_values,
            single_relationships: main_single_relationships,
        } = crate::graph_walker::main::gen_query(
            &ciboulette_store,
            request.resource_type(),
            main_attrs.attributes(),
            main_attrs.relationships(),
            true,
        )?;
        se.gen_update_normal(&main_table, main_update_values, &request, true)?;
        se.buf.write_all(b"), ")?;
        se.write_table_info(&main_cte_data)?;
        se.buf.write_all(b" AS (")?;
        se.gen_select_cte_final(&main_cte_update, &main_type, &request.query(), true)?;
        se.buf.write_all(b")")?;
        se.gen_select_single_rel_routine(
            &ciboulette_store,
            &ciboulette_table_store,
            request.query(),
            &mut table_list,
            &main_type,
            &main_cte_update,
            main_single_relationships,
        )?;
        let rels = crate::graph_walker::relationships::gen_query(
            &ciboulette_store,
            request.resource_type(),
            main_attrs.relationships(),
        )?;
        se.gen_select_multi_rel_routine(
            &ciboulette_table_store,
            &request.query(),
            &mut table_list,
            &main_cte_data,
            rels,
        )?;
        se.buf.write_all(b" ")?;
        se.gen_union_select_all(&table_list)?;
        Ok(se)
    }

    pub fn gen_update_rel(
        ciboulette_store: &'a CibouletteStore<'a>,
        ciboulette_table_store: &'a Ciboulette2PostgresTableStore<'a>,
        request: &'a CibouletteUpdateRequest<'a>,
    ) -> Result<Self, Ciboulette2SqlError> {
        let mut se = Self::default();
        let mut table_list: Vec<Ciboulette2PostgresTableSettings<'_>> = Vec::with_capacity(128);
        let rels = match request.data() {
            CibouletteUpdateRequestType::MainType(_) => {
                return Err(Ciboulette2SqlError::UpdatingMainObject)
            }
            CibouletteUpdateRequestType::Relationship(rels) => rels,
        };
        let mut relationship: BTreeMap<Cow<'a, str>, CibouletteRelationshipObject<'a>> =
            BTreeMap::new();
        relationship.insert(
            Cow::Borrowed(rels.type_().name().as_str()),
            CibouletteRelationshipObject {
                data: rels.value().clone(),
                ..Default::default()
            },
        );
        let main_type = request.resource_type();
        let main_table = ciboulette_table_store.get(main_type.name().as_str())?;
        let main_cte_update =
            main_table.to_cte(Cow::Owned(format!("cte_{}_update", main_table.name())))?;
        let main_cte_data =
            main_table.to_cte(Cow::Owned(format!("cte_{}_data", main_table.name())))?;
        table_list.push(main_cte_data.clone());
        // WITH
        se.buf.write_all(b"WITH \n")?;
        // WITH "cte_main_update"
        se.write_table_info(&main_cte_update)?;
        let Ciboulette2PostgresMain {
            insert_values: rel_values,
            single_relationships,
        } = crate::graph_walker::relationships::gen_query_rel(
            &ciboulette_store,
            &request.resource_type(),
            &rels,
        )?;
        se.buf.write_all(b" AS (")?;

        se.gen_update_normal(&main_table, rel_values, &request, true)?;
        se.buf.write_all(b"), ")?;
        se.write_table_info(&main_cte_data)?;
        se.buf.write_all(b" AS (")?;
        se.gen_select_cte_final(&main_cte_update, &main_type, &request.query(), true)?;
        se.buf.write_all(b")")?;
        se.gen_select_single_rel_routine(
            &ciboulette_store,
            &ciboulette_table_store,
            request.query(),
            &mut table_list,
            &main_type,
            &main_cte_update,
            single_relationships,
        )?;
        se.buf.write_all(b" ")?;
        se.gen_union_select_all(&table_list)?;
        Ok(se)
    }
}
