use diesel::prelude::*;
use itertools::Itertools;

use crate::{IssueType, Violation};

use self::models::{ColumnInfo, Version};

mod models;
mod schema;
mod v5_7;
mod v8_0;

pub fn find_violations(db: &str) -> Vec<Violation> {
    let con = establish_connection(db);
    let version = get_version(&con).mysql_version;

    let (reserved, keywords) = if version.starts_with("5.7.") {
        (v5_7::RESERVED_WORDS, v5_7::KEYWORDS)
    } else if version.starts_with("8.0.") {
        (v8_0::RESERVED_WORDS, v8_0::KEYWORDS)
    } else {
        panic!("Unsupported MySQL version {}", version)
    };

    let columns = get_columns(&con);

    itertools::concat(vec![
        filter_columns(&columns, reserved, IssueType::Reserved),
        filter_columns(&columns, keywords, IssueType::Keyword),
    ])
}

fn establish_connection(db: &str) -> MysqlConnection {
    MysqlConnection::establish(db).unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

fn get_version(con: &MysqlConnection) -> Version {
    use self::schema::version::dsl::*;

    version
        .limit(1)
        .load::<Version>(con)
        .expect("Error loading version")
        .into_iter()
        .next()
        .expect("No version found")
}

fn get_columns(con: &MysqlConnection) -> Vec<ColumnInfo> {
    use self::schema::columns::dsl::*;

    columns
        .filter(table_schema.eq("rssp"))
        .load::<ColumnInfo>(con)
        .expect("Error loading columns")
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
