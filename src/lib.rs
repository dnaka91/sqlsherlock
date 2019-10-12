#[macro_use]
extern crate diesel;

use std::env;

use dotenv::dotenv;

#[cfg(feature = "mysql")]
pub mod mysql;
#[cfg(feature = "sqlite")]
pub mod sqlite;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum IssueType {
    Reserved,
    Keyword,
}

#[derive(Debug)]
pub struct Violation {
    pub issue_type: IssueType,
    pub table: String,
    pub columns: Vec<String>,
}

pub fn find_violations(db: Option<String>) -> Vec<Violation> {
    dotenv().ok();

    let database_url = db
        .or_else(|| env::var("DATABASE_URL").ok())
        .expect("DATABASE_URL must be set or a database connection string provided");

    if let Some(prefix) = database_url.find(':') {
        return match &database_url[..prefix] {
            "mysql" => mysql::find_violations(&database_url),
            "postgres" => unimplemented!(),
            _ => panic!("Unsupported database"),
        };
    }

    sqlite::find_violations(&database_url)
}
