use std::{
    fmt::Debug,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    str::FromStr,
};

pub struct RowHandle(usize);

pub struct Database<T: FromStr + Default + ToStr + PartialEq>
where
    <T as FromStr>::Err: Debug,
{
    pub(crate) path: PathBuf,
    pub rows: Vec<T>,
}

impl<T: FromStr + Default + ToStr + PartialEq> Database<T>
where
    <T as FromStr>::Err: Debug,
{
    pub fn connect(path: std::path::PathBuf) -> Database<T> {
        println!("Successful connection {}", "to database"); // TODO: change "database" too name of database
        Database::<T> {
            path,
            rows: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        // Read each line of file one by one and convert it too Schema (T)
        let mut file = File::open(&self.path);
        if file.is_err() {
            println!("Creating new database");
            file = File::create(&self.path);
        }
        let buf_reader = BufReader::new(file.expect("Failed to create database"));
        let lines = buf_reader.lines();
        for line in lines {
            let line = line.unwrap();
            let row = T::from_str(&line);
            if row.is_err() {
                println!("Failed to parse line, restoring to default");
                self.rows.push(T::default());
            } else {
                self.rows.push(row.unwrap());
            }
        }
    }

    pub fn insert(&mut self, row: T) {
        self.rows.push(row);
    }

    pub fn query<F: Fn(&T) -> bool + Send + Sync>(&mut self, expr: F) -> Option<RowHandle> {
        for (index, row) in self.rows.iter().enumerate() {
            if expr(row) {
                return Some(RowHandle(index));
            }
        }
        None
    }

    pub fn delete(&mut self, row_handle: RowHandle) -> Result<(), ()> {
        let row = self.get(row_handle).expect("Invalid handle");
        let index = self.rows.iter().position(|r| r == row);
        if index.is_none() {
            return Err(());
        }
        self.rows.remove(index.unwrap());
        Ok(())
    }

    pub fn last(&mut self) -> Option<RowHandle> {
        if self.rows.is_empty() {
            return None;
        }
        Some(RowHandle(self.rows.len() - 1))
    }

    pub fn save(&mut self) {
        // open file with write access
        let mut file = OpenOptions::new()
            .write(true)
            .open(&self.path)
            .expect("Failed to open file");
        for row in &self.rows {
            let line = format!("{}\n", check_string(row.to_str().as_str()));
            file.write(line.as_bytes())
                .expect("Failed to write to database");
        }
    }

    pub fn get(&self, row: RowHandle) -> Option<&T> {
        self.rows.get(row.0)
    }
}

// Ensure that \n characters become \\n.
fn check_string(input: &str) -> String {
    let mut res = String::new();
    for char in input.chars() {
        match char {
            '\n' => {
                res.push_str("\\n");
            }
            _ => {
                res.push(char);
            }
        }
    }

    res
}

// Helper function to parse lines.
pub fn get_columns(line: &str) -> Option<Vec<String>> {
    let mut res = Vec::new();

    // parse char by char
    let mut current = String::new();
    let mut is_quoted = false;
    let mut is_escaped = false;
    for char in line.chars() {
        if is_escaped {
            match char {
                '\\' => {
                    current.push('\\');
                }
                'n' => {
                    current.push('\n');
                }
                _ => {
                    current.push('\\');
                    current.push(char);
                }
            }
            is_escaped = false;
        } else if is_quoted {
            match char {
                '\\' => {
                    is_escaped = true;
                }
                '"' => {
                    is_quoted = false;
                }
                _ => {
                    current.push(char);
                }
            }
        } else {
            match char {
                // push current to res and reset current.
                ';' => {
                    res.push(current);
                    current = String::new();
                }
                '"' => {
                    is_quoted = true;
                }
                _ => {
                    current.push(char);
                }
            }
        }
    }

    // push last current to res
    res.push(current);

    Some(res)
}

pub trait ToStr {
    fn to_str(&self) -> String;
}
