use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
    fmt::Debug,
};

use rayon::prelude::*;

pub struct Database<T: FromStr + Default> where <T as FromStr>::Err: Debug {
    pub(crate) file: File,
    pub rows: Vec<T>,
}

impl<T: FromStr + Default> Database<T> where <T as FromStr>::Err: Debug {
    pub fn connect(path: std::path::PathBuf) -> Database<T> {
        let file = File::open(path).expect("Failed to open database");
        println!("Successful connection {}", "to database"); // TODO: change "database" too name of database
        Database::<T> {
            file,
            rows: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        // Read each line of file one by one and convert it too Scheme (T)
        let buf_reader = BufReader::new(&self.file);
        let lines = buf_reader.lines();
        for line_ in lines {
            let line = line_.unwrap();
            let row = T::from_str(&line);
            if row.is_err() {
                println!("Failed to parse line, restoring to default");
                self.rows.push(T::default());
            } else {
                self.rows.push(row.unwrap());
            }
        }
    }
}
