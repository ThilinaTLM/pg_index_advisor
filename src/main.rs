use std::io;

struct Statistics {
    table_name: String,
    column_name: String,
    column_data_type: String,
    operator: String,
    queryFrequency: i32,
    row_count: f64,
    page_count: i32,
    n_distinct: f64,
    most_common_vals: Option<Vec<f64>>,
    most_common_freqs: Option<Vec<f64>>,
}

#[derive(Debug)]
struct ColIndexObject {
    table_name: String,
    column_name: String,
    suggested_index: String,
    rating: i32,
}

fn suggest_indexes( stats: &Statistics) -> Result<String, String> {

    // Check if the column date type is array or jsonb if so suggest GIN index 
    if stats.column_data_type == "array" || stats.column_data_type == "jsonb" {
        let col_index_obj: ColIndexObject = ColIndexObject {
            table_name: stats.table_name.clone(),
            column_name: stats.column_name.clone(),
            suggested_index: "GIN".to_string(),
            rating: stats.queryFrequency,
        };     

        println!("{:?}", col_index_obj);
        // return Ok(col_index_obj);
    }

    Err("No suggested index for the given data type".to_string())
}

fn main() {
    // Run suggest_indexes function
    let statement = "SELECT * FROM foo WHERE bar = 1";
    
    let stats = Statistics {
        table_name: "foo".to_string(),
        column_name: "bar".to_string(),
        column_data_type: "integer".to_string(),
        operator: "=".to_string(),
        queryFrequency: 1,
        row_count: 1000.00,
        page_count: 100,
        n_distinct: 100.0,
        most_common_vals: None,
        most_common_freqs: None,
    };

    let result = suggest_indexes(&stats);
    println!("{:?}", result);
}


struct Table {
    table_name: String,
    column_name: String,
    column_dtype: String,
    row_count: i16,
    rating: i8,
}

// fn main() {
//     let mut tables: Vec<Table> = Vec::new();

//     loop {
//         let table_name = get_input("Enter table name: ");
//         if table_name == "end" {
//             break;
//         }

//         let column_name = get_input("Enter column name: ");
//         let column_dtype = get_input("Enter column data type: ");
//         let row_count: i16 = get_input_as_number("Enter row count: ");
//         let rating: i8 = get_input_as_number("Enter rating: ");

//         let table = Table {
//             table_name,
//             column_name,
//             column_dtype,
//             row_count,
//             rating,
//         };

//         tables.push(table);
//     }

//     // Print the array of struct objects
//     for table in tables {
//         println!("Table Name: {}", table.table_name);
//         println!("Column Name: {}", table.column_name);
//         println!("Column Data Type: {}", table.column_dtype);
//         println!("Row Count: {}", table.row_count);
//         println!("Rating: {}", table.rating);
//         println!("-----------------------------");
//     }
// }

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