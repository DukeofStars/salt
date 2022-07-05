use std::{default, str::FromStr};

use clap::Parser;
use derive_more::FromStr;

use saltdb::*;

fn main() {
    let cli = Cli::parse();
    println!("Salt connecting to: {}", cli.db.display());
    let mut db = Database::<Row>::connect(cli.db);
    db.parse();

    for row in db.rows {
        println!("> {:?}", row);
    }
}

// Dummy row
#[derive(Default, FromStr, Debug)]
struct Row {
    name: String,
}

#[derive(Parser)]
struct Cli {
    // File path to the saltdb database
    #[clap(short, long, default_value = "salt.sdb", value_parser)]
    db: std::path::PathBuf,
}
