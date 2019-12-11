use std::process;

use anyhow::Result;
use crossterm::style::{style, Colorize, Styler};
use structopt::clap::AppSettings;
use structopt::StructOpt;

use sqlsherlock::{IssueType, Violation};

/// Check your SQL database for reserved words and optionally keywords.
#[derive(Debug, StructOpt)]
#[structopt(setting = AppSettings::ColoredHelp)]
struct Opt {
    /// Include keywords (non-reserved) into the scan
    #[structopt(short, long)]
    keywords: bool,
}

fn main() -> Result<()> {
    let opt: Opt = Opt::from_args();

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

    if opt.keywords && !keywords.is_empty() {
        println!("\nKEYWORDS:");
        for v in &keywords {
            println!("{}", style(&v.table).blue().bold());
            for col in &v.columns {
                println!("  {}", style(col).blue());
            }
        }
    }

    if !reserved.is_empty() || opt.keywords && !keywords.is_empty() {
        process::exit(1);
    }

    Ok(())
}
