use std::str::FromStr;

use saltdb::*;

fn main() {
    let db = "salt.sdb";
    println!("Salt connecting to: {db}");
    let mut db = Database::<Row>::connect(db.into());
    db.parse();

    db.insert(Row {
        name: "John".into(),
        id: 0, // TODO: change id each time
    });

    for row in &db.rows {
        println!("> {}: {}", row.id, row.name);
    }

    db.save();
}

// Dummy row
#[derive(Default, Debug)]
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

        let name = columns[0].clone();
        let id = columns[1].parse::<u64>().unwrap();

        Ok(Row { name, id })
    }
}

impl ToStr for Row {
    fn to_str(&self) -> String {
        format!("\"{}\";{}", self.name, self.id.to_string())
    }
}
