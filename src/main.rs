use std::io;

struct ColInfoObj {
    table_name: String,
    column_name: String,
    column_dtype: String,
    operator: String,
    row_count: i32,
    rating: i8,
}

struct ColIndexObj {
    table_name: String,
    column_name: String,
    suggested_index: String,
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
        let row_count: i32 = get_input_as_number("Enter row count: ");
        let rating: i8 = get_input_as_number("Enter rating: ");

        let obj = ColInfoObj {
            table_name,
            column_name,
            column_dtype,
            operator,
            row_count,
            rating,
        };

        col_info_arr.push(obj);
    }

    let mut col_index_arr: Vec<ColIndexObj> = Vec::new();

    for obj in col_info_arr {
        println!("-----------------------------");
        // Check column data type
        match obj.column_dtype.as_str() {
            "array" | "jsonb" | "range" => {
                println!("GIN Index Suggested");
                let index_obj = ColIndexObj {
                    table_name: obj.table_name,
                    column_name: obj.column_name,
                    suggested_index: "GIN".to_string(),
                    rating: obj.rating,
                };

                col_index_arr.push(index_obj);
                continue;
            },
            "complex" => {
                println!("GiST Index Suggested");
                let index_obj = ColIndexObj {
                    table_name: obj.table_name,
                    column_name: obj.column_name,
                    suggested_index: "GiST".to_string(),
                    rating: obj.rating,
                };

                col_index_arr.push(index_obj);
                continue;
            },
            _ => {
                println!("Passed to Operator Type Check->");
            }
        }

        // Check operator type
        const ROW_COUNT_THRESHOLD: i32 = 10000;
        match obj.operator.as_str() {
            "<=" | "=" | "<" | ">=" | "IN" | "BETWEEN" | "IS NOT NULL" | "IS NULL" | "LIKE" | "~" => {
                if obj.row_count >= ROW_COUNT_THRESHOLD {
                    println!("BRIN Index Suggested");
                    let index_obj = ColIndexObj {
                        table_name: obj.table_name,
                        column_name: obj.column_name,
                        suggested_index: "BRIN".to_string(),
                        rating: obj.rating,
                    };

                    col_index_arr.push(index_obj);
                } else {
                    println!("B-Tree Index Suggested");
                    let index_obj = ColIndexObj {
                        table_name: obj.table_name,
                        column_name: obj.column_name,
                        suggested_index: "B-tree".to_string(),
                        rating: obj.rating,
                    };

                    col_index_arr.push(index_obj);
                }

                continue;
            },
            _ => {
                println!("B-Tree Index Suggested");
                let index_obj = ColIndexObj {
                    table_name: obj.table_name,
                    column_name: obj.column_name,
                    suggested_index: "B-Tree".to_string(),
                    rating: obj.rating,
                };

                col_index_arr.push(index_obj);
            }
        }
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