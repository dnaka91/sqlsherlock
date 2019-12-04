#[derive(Debug, Clone, Queryable)]
pub struct ColumnInfo {
    pub table_catalog: String,
    pub table_schema: String,
    pub table_name: String,
    pub column_name: String,
}
