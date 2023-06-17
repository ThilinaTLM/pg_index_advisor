use pgx::prelude::*;
use sqlparser::ast::{Expr, ObjectName, SelectItem, Statement, Table, TableFactor};
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

mod parser;
mod schema;
mod algorithm;

pgx::pg_module_magic!();

#[pg_extern]
fn hello_pg_index_advisor() -> &'static str {
    "Hello, pg_index_advisor"
}

pub struct Statistics {
    table_name: String,
    column_name: String,
    column_dtype: String,
    row_count: i64,
    page_count: i32,
    n_distinct: f64,
    rating: i8,
    operator: String,
    most_common_vals: Option<Vec<f64>>,
    most_common_freqs: Option<Vec<f64>>,
}

fn suggest_indexes(statement: &sqlparser::ast::Statement, stats: &[&Statistics]) -> Result<String, String> {
    // Implement your index suggestion logic based on the collected statistics
    // For example, you can decide to suggest an index if the selectivity is above a certain threshold
    // This is a simple example and may not be sufficient for all use cases

    let selectivity_threshold = 0.1; // You can adjust this based on your requirements

    let mut suggestions: Vec<String> = Vec::new();

    // Calculate the estimated selectivity (in this simple example, we assume uniform distribution)
    // let selectivity = 1.0 / stats.n_distinct;

    // if selectivity > selectivity_threshold {
    //     Ok(format!("Suggest creating an index on column '{}' of table '{}'", stats.column_name, stats.table_name))
    // } else {
    //     Ok("No index suggestion".to_string())
    // }

    if !suggestions.is_empty() {
        let suggestion_string = suggestions.join("\n");
        Ok(suggestion_string)
    } else {
        Ok("No index suggestions".to_string())
    }
}


#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::prelude::*;

    #[pg_test]
    fn test_hello_pg_index_advisor() {
        assert_eq!("Hello, pg_index_advisor", crate::hello_pg_index_advisor());
    }
}

/// This module is required by `cargo pgx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}





