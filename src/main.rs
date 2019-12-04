use std::process;

use anyhow::Result;
use crossterm::style::{style, Colorize, Styler};

use sqlsherlock::{IssueType, Violation};

fn main() -> Result<()> {
    let violations = sqlsherlock::find_violations(None)?;
    let (reserved, keywords): (Vec<Violation>, Vec<Violation>) = violations
        .into_iter()
        .partition(|v| v.issue_type == IssueType::Reserved);

    if !reserved.is_empty() {
        println!("\nRESERVED WORDS:");
        for v in &reserved {
            println!("{}", style(&v.table).yellow().bold());
            for col in &v.columns {
                println!("  {}", style(col).yellow());
            }
        }
    }

    if !keywords.is_empty() {
        println!("\nKEYWORDS:");
        for v in &keywords {
            println!("{}", style(&v.table).blue().bold());
            for col in &v.columns {
                println!("  {}", style(col).blue());
            }
        }
    }

    if !reserved.is_empty() || !keywords.is_empty() {
        process::exit(1);
    }

    Ok(())
}
