use pgx::prelude::*;
use sqlparser::ast::{Expr, ObjectName, SelectItem, Statement, Table, TableFactor};
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

mod parser;
mod schema;

pgx::pg_module_magic!();

#[pg_extern]
fn hello_pg_index_advisor() -> &'static str {
    "Hello, pg_index_advisor"
}

struct Statistics {
    table_name: String,
    column_name: String,
    row_count: f64,
    page_count: i32,
    n_distinct: f64,
    most_common_vals: Option<Vec<f64>>,
    most_common_freqs: Option<Vec<f64>>,
}