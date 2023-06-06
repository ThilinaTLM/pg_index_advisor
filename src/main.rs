use std::io;

struct ColInfoObj {
    table_name: String,
    column_name: String,
    column_dtype: String,
    operator: String,
    row_count: i16,
    rating: i8,
}

fn main() {
    let mut col_info_arr: Vec<ColInfoObj> = Vec::new();

    loop {
        println!("-----------------------------");
        let table_name = get_input("Enter table name (\"end\" to terminate): ");
        if table_name == "end" {
            break;
        }

        let column_name = get_input("Enter column name: ");
        let column_dtype = get_input("Enter column data type: ");
        let operator = get_input("Enter operator type: ");
        let row_count: i16 = get_input_as_number("Enter row count: ");
        let rating: i8 = get_input_as_number("Enter rating: ");

        let table = ColInfoObj {
            table_name,
            column_name,
            column_dtype,
            operator,
            row_count,
            rating,
        };

        col_info_arr.push(table);
    }

    // Print the array of struct objects
    for obj in col_info_arr {
        println!("-----------------------------");
        println!("Table Name: {}", obj.table_name);
        println!("Column Name: {}", obj.column_name);
        println!("Column Data Type: {}", obj.column_dtype);
        println!("Operator: {}", obj.operator);
        println!("Row Count: {}", obj.row_count);
        println!("Rating: {}", obj.rating);
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