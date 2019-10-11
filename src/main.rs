use crossterm::{style, Colorize, Styler};

use sqlsherlock::mysql;
use sqlsherlock::IssueType;

fn main() {
    let violations = mysql::find_violations(None);

    println!("\nRESERVED WORDS:");
    for v in violations
        .iter()
        .filter(|v| v.issue_type == IssueType::Reserved)
    {
        println!("{}", style(&v.table).yellow().bold());
        for col in &v.columns {
            println!("  {}", style(col).yellow());
        }
    }

    println!("\nKEYWORDS:");
    for v in violations
        .iter()
        .filter(|v| v.issue_type == IssueType::Keyword)
    {
        println!("{}", style(&v.table).blue().bold());
        for col in &v.columns {
            println!("  {}", style(col).blue());
        }
    }
}
