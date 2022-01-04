use std::process::Command;
use std::str::FromStr;

use anyhow::Result;
use itertools::{Either, Itertools};
use kuchiki::iter::{Descendants, Elements, Select};
use kuchiki::traits::TendrilSink;
use rayon::prelude::*;

#[derive(Clone)]
struct Reservation(Option<bool>);

impl FromStr for Reservation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "" => Self(None),
            "reserved" | "reserved (can be function or type)" => Self(Some(true)),
            "non-reserved"
            | "non-reserved (can be function or type)"
            | "non-reserved (cannot be function or type)" => Self(Some(false)),
            _ => anyhow::bail!("Unknown reservation type `{}`", s),
        })
    }
}

#[derive(Clone)]
struct Entry {
    keyword: String,
    postgres: Reservation,
    sql_2016: Reservation,
    sql_2011: Reservation,
    sql_92: Reservation,
}

const VERSIONS: &[f64] = &[12.0, 11.0, 10.0, 9.6, 9.5];

#[test]
fn generate() -> Result<()> {
    VERSIONS
        .into_par_iter()
        .map(|version| {
            let url = format!(
                "https://www.postgresql.org/docs/{}/sql-keywords-appendix.html",
                version
            );
            let response = ureq::get(&url).call()?.into_string()?;
            let document = kuchiki::parse_html().one(response);
            let mut entries = vec![];

            let selector = if *version >= 10.0 {
                "#KEYWORDS-TABLE tbody > tr"
            } else {
                "#KEYWORDS-TABLE ~ table > tbody > tr"
            };

            for tr in document.select(selector).unwrap() {
                let mut tds = tr.as_node().select("td").unwrap();
                let next = |tds: &mut Select<Elements<Descendants>>| {
                    tds.next().unwrap().text_contents().trim().to_owned()
                };

                entries.push(Entry {
                    keyword: next(&mut tds),
                    postgres: next(&mut tds).parse()?,
                    sql_2016: next(&mut tds).parse()?,
                    sql_2011: next(&mut tds).parse()?,
                    sql_92: next(&mut tds).parse()?,
                });
            }

            let (reserved, keyword): (Vec<_>, Vec<_>) = entries.iter().partition_map(|e| {
                if e.postgres
                    .0
                    .or(e.sql_2016.0)
                    .or(e.sql_2011.0)
                    .or(e.sql_92.0)
                    .unwrap_or_default()
                {
                    Either::Left(&e.keyword)
                } else {
                    Either::Right(&e.keyword)
                }
            });

            let tokens = quote::quote! {
                pub const RESERVED_WORDS: &[&str] = &[#(#reserved),*];

                pub const KEYWORDS: &[&str] = &[#(#keyword),*];
            };

            let file_name = format!("src/postgres/v{}.rs", version.to_string().replace(".", "_"));
            std::fs::write(&file_name, tokens.to_string())?;
            Command::new("rustfmt").arg(&file_name).output()?;
            Ok(())
        })
        .collect::<Result<Vec<()>>>()?;

    Ok(())
}
