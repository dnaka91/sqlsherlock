use std::process::Command;
use std::str::FromStr;

use anyhow::{ensure, Result};
use itertools::{Either, Itertools};
use kuchiki::traits::TendrilSink;
use rayon::prelude::*;

#[derive(Clone)]
struct Entry {
    keyword: String,
    reserved: bool,
}

impl FromStr for Entry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(';').collect();
        ensure!(!parts.is_empty(), "entry must not be empty");

        let keyword = if parts
            .last()
            .map(|p| p.trim().starts_with("remove"))
            .unwrap_or_default()
        {
            ""
        } else {
            parts[0]
        };

        Ok(Self {
            keyword: keyword.split(' ').next().unwrap().to_owned(),
            reserved: keyword.ends_with("(R)"),
        })
    }
}

const VERSIONS: &[f64] = &[8.0, 5.7, 5.6];

#[test]
fn generate() -> Result<()> {
    VERSIONS
        .into_par_iter()
        .map(|version| {
            let url = format!(
                "https://dev.mysql.com/doc/refman/{:.1}/en/keywords.html",
                version
            );
            let response = ureq::get(&url).call().into_string()?;
            let document = kuchiki::parse_html().one(response);
            let mut entries: Vec<Entry> = vec![];

            for div in document.select(".simplesect").unwrap() {
                if div
                    .as_node()
                    .select_first("a[name=keywords-in-current-series]")
                    .is_err()
                {
                    continue;
                }

                for div in div.as_node().select(".listitem > p").unwrap() {
                    entries.push(div.text_contents().parse()?);
                }

                break;
            }

            let (reserved, keyword): (Vec<_>, Vec<_>) = entries
                .iter()
                .filter(|e| !e.keyword.is_empty())
                .partition_map(|e| {
                    if e.reserved {
                        Either::Left(&e.keyword)
                    } else {
                        Either::Right(&e.keyword)
                    }
                });

            let tokens = quote::quote! {
                pub const RESERVED_WORDS: &[&str] = &[#(#reserved),*];

                pub const KEYWORDS: &[&str] = &[#(#keyword),*];
            };

            let file_name = format!(
                "src/mysql/v{}.rs",
                format!("{:.1}", version).replace(".", "_")
            );
            std::fs::write(&file_name, tokens.to_string())?;
            Command::new("rustfmt").arg(&file_name).output()?;
            Ok(())
        })
        .collect::<Result<Vec<()>>>()?;

    Ok(())
}
