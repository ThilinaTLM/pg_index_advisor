use pgx::prelude::*;
use sqlparser::ast::{Expr, ObjectName, SelectItem, Statement, Table, TableFactor};
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;


mod query;
mod algorithm;

pgx::pg_module_magic!();

#[pg_extern]
fn hello_pg_index_advisor() -> &'static str {
    "Hello, pg_index_advisor"
}

fn parse_query(query: &str) -> Result<sqlparser::ast::Statement, String> {
    let dialect = PostgreSqlDialect {};
    let ast = Parser::parse_sql(&dialect, query).map_err(|e| format!("Error parsing SQL: {}", e))?;
    ast.into_iter().next().ok_or("No statement found in the query".to_string())
}

#[pg_extern]
fn analyze_query(query: &str) -> Result<String, String> {
    let statement = parse_query(query)?;
    let stats = collect_statistics(&statement)?;
    // suggest_indexes(&statement, &stats)
    Ok(stats)
}

fn collect_statistics(statement: &sqlparser::ast::Statement) -> Result<String, String> {
    // Extract the table and column names from the statement
    let (table_name) = match statement {
        // let (table_name, column_name) = match statement {
        sqlparser::ast::Statement::Query(query) => {
            let select = query.body.as_ref();
            let table_name = if let sqlparser::ast::SetExpr::Select(select) = &select {
                if let Some(table) = select.from.iter().next() {
                    let table_name = table.relation.clone();
                    table_name.to_string()
                } else {
                    return Err("No table found in the query".to_string());
                }
            } else {
                return Err("Invalid query format".to_string());
            };

            // let column_name = select.projection.iter().filter_map(|item| {
            //     if let SelectItem::UnnamedExpr(Expr::Identifier(id)) = item {
            //         Some(id.value.clone())
            //     } else {
            //         None
            //     }
            // }).next().ok_or("No column found in the query".to_string())?;

            (table_name)
            // (table_name, column_name)
        }
        _ => return Err("Invalid statement type".to_string()),
    };

    // let table_stats_query = format!("SELECT reltuples, relpages FROM pg_class WHERE oid = '{}'::regclass", table_name);
    // let column_stats_query = format!("SELECT n_distinct, most_common_vals, most_common_freqs FROM pg_statistic JOIN pg_attribute ON pg_statistic.starelid = pg_attribute.attrelid AND pg_statistic.staattnum = pg_attribute.attnum WHERE pg_statistic.starelid = '{}'::regclass AND pg_attribute.attname = '{}'", table_name, column_name);

    // let table_stats = Spi::get_one::<(f64, i32)>(&table_stats_query).map_err(|e| format!("Error fetching table statistics: {}", e))?;
    // let column_stats = Spi::get_one::<(f64, Option<Vec<f64>>, Option<Vec<f64>>)>(&column_stats_query).map_err(|e| format!("Error fetching column statistics: {}", e))?;

    // Ok(Statistics {
    //     table_name,
    //     column_name,
    //     row_count: table_stats.0,
    //     page_count: table_stats.1,
    //     n_distinct: column_stats.0,
    //     most_common_vals: column_stats.1,
    //     most_common_freqs: column_stats.2,
    // })
    Ok(table_name)
}

pub struct Statistics {
    schema_name: Option<String>,
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
