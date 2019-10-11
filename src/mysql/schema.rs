table! {
    sys.version(sys_version, mysql_version) {
        sys_version -> Varchar,
        mysql_version -> Varchar,
    }
}

table! {
    information_schema.columns(table_schema, table_name, column_name) {
        table_schema -> Varchar,
        table_name -> Varchar,
        column_name -> Varchar,
    }
}
