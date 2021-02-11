#[macro_use]
extern crate clap;

#[macro_use]
extern crate anyhow;

use clap::Arg;
use std::collections::BTreeSet;

fn main() -> anyhow::Result<()> {
    let matches = clap::app_from_crate!()
        .arg(
            Arg::with_name("path")
                .short("p")
                .help("Set the glob file path")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("top-n")
                .short("n")
                .help("Top fragment file")
                .default_value("20")
                .takes_value(true),
        )
        .get_matches();

    let path = matches.value_of("path").unwrap();
    let top_n = matches.value_of("top-n").unwrap().parse::<usize>()?;

    let mut records = BTreeSet::new();

    for entry in glob::glob(path)? {
        let entry = entry?;

        print!("\rIn progress: {}", entry.display());

        let count = fiemap::fiemap(&entry)?.count();
        records.insert((count, entry));
    }

    if records.len() == 0 {
        return Err(anyhow!("no files are scanned."));
    }

    // summary.
    println!("\nTotal scan file: {}", records.len());

    for (count, entry) in records.iter().rev().take(top_n) {
        println!("{:<48}  {}", entry.display(), count);
    }

    Ok(())
}
