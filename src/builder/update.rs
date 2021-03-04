use super::*;
use crate::graph_walker::main::Ciboulette2PostgresMain;
use crate::graph_walker::relationships::Ciboulette2PostgresRelationships;

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
        let mut param_ident: Vec<(
            Ciboulette2PostgresSafeIdent,
            Option<Ciboulette2PostgresSafeIdent>,
            Option<Ciboulette2PostgresSafeIdent>,
        )> = Vec::with_capacity(params.len());
        let mut param_value: Vec<Ciboulette2SqlValue<'_>> = Vec::with_capacity(params.len());
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
        } = crate::graph_walker::main::gen_query_insert(
            &ciboulette_store,
            request.resource_type(),
            request.data().attributes(),
            request.data().relationships(),
        )?;
        se.gen_update_normal(&main_table, main_update_values, &request, true)?;
        se.buf.write_all(b"), ")?;
        se.write_table_info(&main_cte_data)?;
        se.buf.write_all(b" AS (")?;
        se.gen_select_cte_final(&main_cte_update, &main_type, &request.query(), true)?;
        se.buf.write_all(b")")?;
        println!("KEYS {:#?}", main_single_relationships);
        let main_single_relationships_iter = main_single_relationships.into_iter();

        for key in main_single_relationships_iter {
            println!("KEY {}", key);
            se.buf.write_all(b", ")?;
            let rel_table = ciboulette_table_store.get(key)?;
            let rel_table_cte =
                rel_table.to_cte(Cow::Owned(format!("cte_{}_data", rel_table.name())))?;
            let rel_type = main_type.get_relationship(&ciboulette_store, key)?;
            se.write_table_info(&rel_table_cte)?;
            se.buf.write_all(b" AS (")?;
            se.gen_select_cte_single_rel(
                &rel_table,
                &rel_type,
                &request.query(),
                &main_cte_update,
                &Ciboulette2PostgresSafeIdent::try_from(key)?,
            )?;
            se.buf.write_all(b")")?;
            table_list.push(rel_table_cte);
        }
        let rels = crate::graph_walker::relationships::gen_query_insert(
            &ciboulette_store,
            request.resource_type(),
            request.data().relationships(),
        )?;

        se.gen_select_rel_routine(
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
}
