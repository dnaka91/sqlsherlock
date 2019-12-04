use std::process::Command;

use anyhow::Result;
use isahc::ResponseExt;
use kuchiki::traits::TendrilSink;

#[test]
fn generate() -> Result<()> {
    let response = isahc::get("https://www.sqlite.org/lang_keywords.html")?.text()?;
    let document = kuchiki::parse_html().one(response);
    let mut words = vec![];

    for li in document.select("body > ol > li").unwrap() {
        words.push(li.text_contents().trim().to_owned());
    }

    let tokens = quote::quote! {
        pub const RESERVED_WORDS: &[&str] = &[#(#words),*];
    };

    let file_name = "src/sqlite/words.rs";
    std::fs::write(file_name, format!("{}", tokens))?;
    Command::new("rustfmt").arg(file_name).output()?;

    Ok(())
}
