use std::str::FromStr;

use saltdb::*;

fn main() {
    let db = "salt.sdb";
    println!("Salt connecting to: {db}");
    let mut db = Database::<Row>::connect(db.into());
    db.parse();

    for row in db.rows {
        println!("> {:?}", row);
    }
}

// Dummy row
#[derive(Default, Debug)]
struct Row {
    _name: String,
    _id: u64,
}

impl FromStr for Row {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let columns = get_columns(s);

        if columns.is_none() {
            return Err(());
        }

        let columns = columns.unwrap();

        let name = columns[0].clone();
        let id = columns[1].parse::<u64>().unwrap();

        Ok(Row {
            _name: name,
            _id: id,
        })
    }
}
