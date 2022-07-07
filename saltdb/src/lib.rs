use std::{
    fmt::Debug,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    str::FromStr,
};

pub struct Database<T: FromStr + Default + ToStr>
where
    <T as FromStr>::Err: Debug,
{
    pub(crate) path: PathBuf,
    pub rows: Vec<T>,
}

impl<T: FromStr + Default + ToStr> Database<T>
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
        let buf_reader = BufReader::new(File::open(&self.path).expect("Failed to open database"));
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

    pub fn insert(&mut self, row: T) {
        self.rows.push(row);
    }

    pub fn save(&mut self) {
        // open file with write access
        let mut file = OpenOptions::new()
            .write(true)
            .open(&self.path)
            .expect("Failed to open file");
        for row in &self.rows {
            let line = format!("{}\n", check_string(row.to_str()));
            file.write(line.as_bytes())
                .expect("Failed to write to database");
        }
    }
}

// Ensure that \n characters become \\n.
fn check_string(input: String) -> String {
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
                    res.push(current.clone());
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
