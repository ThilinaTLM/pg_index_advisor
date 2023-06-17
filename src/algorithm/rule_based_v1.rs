use std::collections::HashMap;
use crate::algorithm::*;

impl Algorithm for RuleBasedV1{
    fn generate_index_suggestion(&self, statement: &sqlparser::ast::Statement, stats: &[&Statistics]) -> Vec<ColIndexObj> {
        let mut col_index_arr: Vec<ColIndexObj> = Vec::new();

        for obj in stats {
            println!("-----------------------------");
            // Check column data type
            match obj.column_dtype.as_str() {
                "array" | "jsonb" | "range" => {
                    println!("GIN Index Suggested");
                    let index_obj = ColIndexObj {
                        schema_name: obj.schema_name.clone(),
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
                        schema_name: obj.schema_name.clone(),
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
                            schema_name: obj.schema_name.clone(),
                            table_name: obj.table_name.clone(),
                            column_name: obj.column_name.clone(),
                            suggested_index: "BRIN".to_string(),
                            rating: obj.rating,
                        };

                        col_index_arr.push(index_obj);
                    } else {
                        println!("B-Tree Index Suggested");
                        let index_obj = ColIndexObj {
                            schema_name: obj.schema_name.clone(),
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
                        schema_name: obj.schema_name.clone(),
                        table_name: obj.table_name.clone(),
                        column_name: obj.column_name.clone(),
                        suggested_index: "B-Tree".to_string(),
                        rating: obj.rating,
                    };

                    col_index_arr.push(index_obj);
                }
            }
        }
        col_index_arr
        // Read the array and
    }

    fn generate_table_index_obj(&self, col_index_objs: &[ColIndexObj]) -> TableIndexObj {
        let mut table_index_obj = TableIndexObj {
            schema_name: None,
            table_name: String::new(),
            column_indexes: HashMap::new(),
        };

        for col_index_obj in col_index_objs {
            if table_index_obj.table_name.is_empty() {
                table_index_obj.table_name = col_index_obj.table_name.clone();
                table_index_obj.schema_name = col_index_obj.schema_name.clone();
            }

            if col_index_obj.table_name != table_index_obj.table_name
                || col_index_obj.schema_name != table_index_obj.schema_name
            {
                continue;
            }

            let column_name = &col_index_obj.column_name;
            let column_index = table_index_obj.column_indexes.entry(column_name.clone()).or_default();
            let matching_index = column_index.iter_mut().find(|c| &c.column_name == column_name);

            if let Some(existing_index) = matching_index {
                existing_index.suggested_indexes.push(IndexRating {
                    index: col_index_obj.suggested_index.clone(),
                    rating: col_index_obj.rating,
                });
                existing_index.totalRating += col_index_obj.rating;
            } else {
                column_index.push(ColAllIndexObj {
                    schema_name: col_index_obj.schema_name.clone(),
                    column_name: col_index_obj.column_name.clone(),
                    table_name: col_index_obj.table_name.clone(),
                    suggested_indexes: vec![IndexRating {
                        index: col_index_obj.suggested_index.clone(),
                        rating: col_index_obj.rating,
                    }],
                    totalRating: col_index_obj.rating,
                });
            }
        }

        table_index_obj
    }


}

