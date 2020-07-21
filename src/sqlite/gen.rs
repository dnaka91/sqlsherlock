use std::process::Command;

use anyhow::Result;
use kuchiki::traits::TendrilSink;

#[test]
fn generate() -> Result<()> {
    let response = ureq::get("https://www.sqlite.org/lang_keywords.html")
        .call()
        .into_string()?;
    let document = kuchiki::parse_html().one(response);
    let mut words = vec![];

    for li in document.select("div.columns > ul > li").unwrap() {
        words.push(li.text_contents().trim().to_owned());
    }

    let tokens = quote::quote! {
        pub const RESERVED_WORDS: &[&str] = &[#(#words),*];
    };

    let file_name = "src/sqlite/words.rs";
    std::fs::write(file_name, tokens.to_string())?;
    Command::new("rustfmt").arg(file_name).output()?;

    Ok(())
}
