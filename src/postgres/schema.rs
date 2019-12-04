table! {
    information_schema.columns(table_catalog, table_schema, table_name, column_name) {
        table_catalog -> Varchar,
        table_schema -> Varchar,
        table_name -> Varchar,
        column_name -> Varchar,
    }
}
