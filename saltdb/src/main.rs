use std::str::FromStr;

use saltdb::*;

fn main() {
    let db = "salt.sdb";
    println!("Salt connecting to: {db}");
    let mut db = Database::<Row>::connect(db.into());
    db.parse();

    // Example of a simple query that finds the first john in the database.
    let _john = db.query(|row| row.name == "John");

    // Get correct id
    let mut id = 0;
    let last = db.last();
    if last.is_some() {
        id = db.get(last.unwrap()).unwrap().id + 1;
    }

    // Insert rows is very simple
    db.insert(Row {
        name: "John".into(),
        id,
    });

    // print all rows
    for row in &db.rows {
        println!("> {}: {}", row.id, row.name);
    }

    // Delete last row
    let last = db.last().unwrap();
    let _ = db.delete(last);

    // Save database
    db.save();
}

// Dummy row
#[derive(Default, Debug, PartialEq)]
struct Row {
    name: String,
    id: u64,
}

impl FromStr for Row {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let columns = get_columns(s);

        if columns.is_none() {
            return Err(());
        }

        let columns = columns.unwrap();

        let mut name = String::new();
        if columns.len() >= 1 {
            name = columns[0].clone();
        }
        let mut id = 0;
        if columns.len() >= 2 {
            id = columns[1].parse::<u64>().unwrap();
        }

        Ok(Row { name, id })
    }
}

impl ToStr for Row {
    fn to_str(&self) -> String {
        format!("\"{}\";{}", self.name, self.id.to_string())
    }
}
