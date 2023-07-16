use std::{fs::File, io::{BufReader, Read}};

use pgx::prelude::*;

mod parser;
mod repos;

// mod algorithm;

pg_module_magic!();


#[pg_extern]
fn pgia_health_check() -> String {
    "OK".to_string()
}

#[pg_extern]
fn pgia_test() -> String {
    let result = repos::SchemaRepo::get_columns("test_table");
    match result {
        Ok(columns) => {
            let mut result = String::new();
            for column in columns {
                result.push_str(&column);
                result.push_str(", ");
            }
            result
        },
        Err(err) => {
            format!("Error: {}", err)
        }
    }
}

#[pg_extern]
fn pgia_suggest_index(file_path: String) -> String {
    // Attempt to open the file
    let file = match File::open(&file_path) {
        Err(error) => return format!("Couldn't open file: {}", error),
        Ok(file) => file,
    };

    // Create a BufReader to handle reading of the file
    let mut reader = BufReader::new(file);

    // Create a String to store the file contents
    let mut content = String::new();

    // Attempt to read the file into the content String
    match reader.read_to_string(&mut content) {
        Err(error) => return format!("Couldn't read file: {}", error),
        Ok(_) => {},
    };

    // Return the contents of the file
    content
}