use std::io;

struct Table {
    table_name: String,
    column_name: String,
    column_dtype: String,
    row_count: i16,
    rating: i8,
}

fn main() {
    let mut tables: Vec<Table> = Vec::new();

    loop {
        let table_name = get_input("Enter table name: ");
        if table_name == "end" {
            break;
        }

        let column_name = get_input("Enter column name: ");
        let column_dtype = get_input("Enter column data type: ");
        let row_count: i16 = get_input_as_number("Enter row count: ");
        let rating: i8 = get_input_as_number("Enter rating: ");

        let table = Table {
            table_name,
            column_name,
            column_dtype,
            row_count,
            rating,
        };

        tables.push(table);
    }

    // Print the array of struct objects
    for table in tables {
        println!("Table Name: {}", table.table_name);
        println!("Column Name: {}", table.column_name);
        println!("Column Data Type: {}", table.column_dtype);
        println!("Row Count: {}", table.row_count);
        println!("Rating: {}", table.rating);
        println!("-----------------------------");
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn get_input_as_number<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        let input = get_input(prompt);
        match input.parse::<T>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}