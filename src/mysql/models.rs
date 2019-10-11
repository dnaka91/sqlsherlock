#[derive(Debug, Clone, Queryable)]
pub struct Version {
    pub sys_version: String,
    pub mysql_version: String,
}

#[derive(Debug, Clone, Queryable)]
pub struct ColumnInfo {
    pub table_schema: String,
    pub table_name: String,
    pub column_name: String,
}
