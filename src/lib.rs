#[macro_use]
extern crate diesel;

pub mod mysql;

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
