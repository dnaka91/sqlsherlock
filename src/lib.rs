#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![warn(clippy::nursery)]

#[macro_use]
extern crate diesel;

use std::env;

use anyhow::{bail, Context, Result};
use dotenv::dotenv;
use serde::Serialize;

#[cfg(feature = "mysql")]
pub mod mysql;
#[cfg(feature = "postgres")]
pub mod postgres;
#[cfg(feature = "sqlite")]
pub mod sqlite;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum IssueType {
    Reserved,
    Keyword,
}

#[derive(Debug, Serialize)]
pub struct Violation {
    #[serde(skip)]
    pub issue_type: IssueType,
    pub table: String,
    pub columns: Vec<String>,
}

pub fn find_violations(db: Option<String>) -> Result<Vec<Violation>> {
    dotenv().ok();

    let database_url = db
        .or_else(|| env::var("DATABASE_URL").ok())
        .context("DATABASE_URL must be set or a database connection string provided")?;

    if let Some(prefix) = database_url.find(':') {
        return Ok(match &database_url[..prefix] {
            "mysql" => mysql::find_violations(&database_url)?,
            "postgres" => postgres::find_violations(&database_url)?,
            _ => bail!("Unsupported database"),
        });
    }

    sqlite::find_violations(&database_url)
}
