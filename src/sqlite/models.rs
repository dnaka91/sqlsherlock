use diesel::sql_types::Text;

#[derive(Debug, Clone, Queryable)]
pub struct TableInfo {
    pub type_: String,
    pub name: String,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct ColumnInfo {
    #[sql_type = "Text"]
    pub name: String,
}
