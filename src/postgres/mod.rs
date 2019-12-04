use anyhow::{bail, Context, Result};
use diesel::prelude::*;
use itertools::Itertools;

use crate::{IssueType, Violation};

use self::models::ColumnInfo;

mod functions;
#[cfg(all(test, feature = "gen"))]
mod gen;
mod models;
mod schema;
mod v10;
mod v11;
mod v12;
mod v9_5;
mod v9_6;

pub fn find_violations(db: &str) -> Result<Vec<Violation>> {
    let con = establish_connection(db)?;
    let version = get_version(&con)?;

    let (reserved, keywords) = if version.starts_with("PostgreSQL 9.5.") {
        (v9_5::RESERVED_WORDS, v9_5::KEYWORDS)
    } else if version.starts_with("PostgreSQL 9.6.") {
        (v9_6::RESERVED_WORDS, v9_6::KEYWORDS)
    } else if version.starts_with("PostgreSQL 10.") {
        (v10::RESERVED_WORDS, v10::KEYWORDS)
    } else if version.starts_with("PostgreSQL 11.") {
        (v11::RESERVED_WORDS, v11::KEYWORDS)
    } else if version.starts_with("PostgreSQL 12.") {
        (v12::RESERVED_WORDS, v12::KEYWORDS)
    } else {
        bail!("Unsupported PostgreSQL version {}", version)
    };

    let columns = get_columns(&con)?;

    Ok(itertools::concat(vec![
        filter_columns(&columns, reserved, IssueType::Reserved),
        filter_columns(&columns, keywords, IssueType::Keyword),
    ]))
}

fn establish_connection(db: &str) -> Result<PgConnection> {
    PgConnection::establish(db).with_context(|| format!("Error connecting to {}", db))
}

fn get_version(con: &PgConnection) -> Result<String> {
    use self::functions::version;
    use diesel::select;

    select(version)
        .get_result(con)
        .context("Error loading version")
}

fn get_columns(con: &PgConnection) -> Result<Vec<ColumnInfo>> {
    use self::schema::columns::dsl::*;

    columns
        .filter(table_catalog.eq("rssp").and(table_schema.eq("public")))
        .load::<ColumnInfo>(con)
        .context("Error loading columns")
}

fn filter_columns(columns: &[ColumnInfo], names: &[&str], issue_type: IssueType) -> Vec<Violation> {
    columns
        .iter()
        .filter(|c| names.contains(&c.column_name.to_uppercase().as_str()))
        .group_by(|c| c.table_name.as_str())
        .into_iter()
        .map(|(k, v)| Violation {
            issue_type,
            table: k.to_owned(),
            columns: v.map(|c| c.column_name.clone()).collect(),
        })
        .collect()
}
