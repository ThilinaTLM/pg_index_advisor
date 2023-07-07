#![allow(dead_code)]

use sqlparser::ast::{Assignment, BinaryOperator, Expr, SelectItem, TableFactor, Value};
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

mod test;
mod parser;

#[derive(Debug, PartialEq, Clone)]
enum OperationType {
    Algebraic,
    Comparison,
    Logical,
    StringPattern,
    String,
    Regex,
    NullCheck,
    Other,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ColumnUsage {
    column: String,
    operation: OperationType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TableUsage {
    name: String,
    usages: Vec<ColumnUsage>,
}

pub struct QueryUsageParser {
    dialect: PostgreSqlDialect,
    table_usages: Vec<TableUsage>,
}


pub fn parse_query(query: &str) -> Result<Vec<TableUsage>, ()> {
    let mut parser = QueryUsageParser::new();
    let table_usage = parser.parse(query).unwrap();
    Ok(table_usage)
}
