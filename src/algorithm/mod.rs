#![allow(dead_code)]
mod algo1;
mod test;

use std::io;
use sqlparser::ast::{Expr, ObjectName, SelectItem, Statement, Table, TableFactor};
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;
use crate::Statistics;
use crate::parse_query;

#[derive(Debug, PartialEq, Clone)]
pub struct ColIndexObj {
    table_name: String,
    column_name: String,
    suggested_index: String,
    rating: i8,
}