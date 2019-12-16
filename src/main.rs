#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]

use std::process;

use anyhow::{Context, Result};
use crossterm::style::{style, Colorize, Styler};
use serde::Serialize;
use structopt::{clap::AppSettings, StructOpt};

use sqlsherlock::{IssueType, Violation};

/// Check your SQL database for reserved words and optionally keywords.
#[derive(Debug, StructOpt)]
#[structopt(setting = AppSettings::ColoredHelp)]
struct Opt {
    /// Include keywords (non-reserved) into the scan
    #[structopt(short, long)]
    keywords: bool,
    /// Print the findings as JSON
    #[structopt(short, long)]
    json: bool,

    /// Connection string of the database
    db: Option<String>,
}

#[derive(Debug, Serialize)]
struct JsonOutput<'a> {
    reserved: &'a [Violation],
    keywords: &'a [Violation],
}

fn main() -> Result<()> {
    let opt: Opt = Opt::from_args();

    let violations =
        sqlsherlock::find_violations(opt.db.as_ref()).context("Failed finding violations")?;
    let (reserved, keywords): (Vec<Violation>, Vec<Violation>) = violations
        .into_iter()
        .partition(|v| v.issue_type == IssueType::Reserved);

    if opt.json {
        print_json(&reserved, &keywords)?;
    } else {
        print_text(&opt, &reserved, &keywords);
    }

    if !reserved.is_empty() || opt.keywords && !keywords.is_empty() {
        process::exit(1);
    }

    Ok(())
}

fn print_json(reserved: &[Violation], keywords: &[Violation]) -> Result<()> {
    serde_json::to_writer(std::io::stdout(), &JsonOutput { reserved, keywords })
        .context("Failed writing JSON to stdout")
}

fn print_text(opt: &Opt, reserved: &[Violation], keywords: &[Violation]) {
    if !reserved.is_empty() {
        println!("\nRESERVED WORDS:");
        for v in reserved {
            println!("{}", style(&v.table).yellow().bold());
            for col in &v.columns {
                println!("  {}", style(col).yellow());
            }
        }
    }

    if opt.keywords && !keywords.is_empty() {
        println!("\nKEYWORDS:");
        for v in keywords {
            println!("{}", style(&v.table).blue().bold());
            for col in &v.columns {
                println!("  {}", style(col).blue());
            }
        }
    }
}
