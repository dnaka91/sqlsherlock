use anyhow::{Context, Result};
use diesel::prelude::*;
use itertools::Itertools;

use crate::{IssueType, Violation};

use self::models::{ColumnInfo, TableInfo};

#[cfg(all(test, feature = "gen"))]
mod gen;
mod models;
mod schema;
mod words;

pub fn find_violations(db: &str) -> Result<Vec<Violation>> {
    let con = establish_connection(db)?;
    let tables = get_tables(&con)?;

    Ok(tables
        .iter()
        .filter_map(|t| {
            let columns = get_columns(&con, &t.name);
            filter_columns(
                &t.name,
                &columns,
                words::RESERVED_WORDS,
                IssueType::Reserved,
            )
        })
        .collect_vec())
}

fn establish_connection(db: &str) -> Result<SqliteConnection> {
    SqliteConnection::establish(db).with_context(|| format!("Error connecting to {}", db))
}

fn get_tables(con: &SqliteConnection) -> Result<Vec<TableInfo>> {
    use self::schema::sqlite_master::dsl::*;

    sqlite_master
        .filter(type_.eq("table").and(name.not_like("sqlite_%")))
        .load(con)
        .context("Error loading tables")
}

fn get_columns(con: &SqliteConnection, table: &str) -> Vec<ColumnInfo> {
    use diesel::sql_query;

    sql_query(format!("PRAGMA table_info({})", table))
        .load(con)
        .expect("Error loading columns")
}

fn filter_columns(
    table: &str,
    columns: &[ColumnInfo],
    names: &[&str],
    issue_type: IssueType,
) -> Option<Violation> {
    let columns = columns
        .iter()
        .filter(|c| names.contains(&c.name.to_uppercase().as_str()))
        .map(|c| c.name.clone())
        .collect_vec();

    if columns.is_empty() {
        return None;
    }

    Some(Violation {
        issue_type,
        table: table.to_owned(),
        columns,
    })
}
