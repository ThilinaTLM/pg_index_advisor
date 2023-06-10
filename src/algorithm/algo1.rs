use crate::algorithm::*;

pub fn generate_index_suggestion(statement: &sqlparser::ast::Statement, stats: &[&Statistics]) -> Vec<ColIndexObj> {
    let mut col_index_arr: Vec<ColIndexObj> = Vec::new();

    for obj in stats {
        println!("-----------------------------");
        // Check column data type
        match obj.column_dtype.as_str() {
            "array" | "jsonb" | "range" => {
                println!("GIN Index Suggested");
                let index_obj = ColIndexObj {
                    table_name: obj.table_name.clone(),
                    column_name: obj.column_name.clone(),
                    suggested_index: "GIN".to_string(),
                    rating: obj.rating,
                };

                col_index_arr.push(index_obj);
                continue;
            },
            "complex" => {
                println!("GiST Index Suggested");
                let index_obj = ColIndexObj {
                    table_name: obj.table_name.clone(),
                    column_name: obj.column_name.clone(),
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
        const ROW_COUNT_THRESHOLD: i64 = 10000;
        match obj.operator.as_str() {
            "<=" | "=" | "<" | ">=" | "IN" | "BETWEEN" | "IS NOT NULL" | "IS NULL" | "LIKE" | "~" => {
                if obj.row_count >= ROW_COUNT_THRESHOLD {
                    println!("BRIN Index Suggested");
                    let index_obj = ColIndexObj {
                        table_name: obj.table_name.clone(),
                        column_name: obj.column_name.clone(),
                        suggested_index: "BRIN".to_string(),
                        rating: obj.rating,
                    };

                    col_index_arr.push(index_obj);
                } else {
                    println!("B-Tree Index Suggested");
                    let index_obj = ColIndexObj {
                        table_name: obj.table_name.clone(),
                        column_name: obj.column_name.clone(),
                        suggested_index: "B-Tree".to_string(),
                        rating: obj.rating,
                    };

                    col_index_arr.push(index_obj);
                }

                continue;
            },
            _ => {
                println!("B-Tree Index Suggested");
                let index_obj = ColIndexObj {
                    table_name: obj.table_name.clone(),
                    column_name: obj.column_name.clone(),
                    suggested_index: "B-Tree".to_string(),
                    rating: obj.rating,
                };

                col_index_arr.push(index_obj);
            }
        }
    }
    print!("{:?}", col_index_arr);
    col_index_arr // Return the vector
}
