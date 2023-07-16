#![allow(dead_code)]
pub mod rule_based_v1;
mod test;

use std::{io, println, vec};
use std::collections::HashMap;
use sqlparser::ast::{Expr, ObjectName, SelectItem, Statement, Table, TableFactor};
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

use crate::Statistics;
use crate::parser;

trait Algorithm {
    fn generate_index_suggestion(&self, statement: &sqlparser::ast::Statement, stats: &[&Statistics]) -> Vec<ColIndexObj>;
    fn generate_table_index_obj(&self, col_index_objs: &[ColIndexObj]) -> TableIndexObj;
}

#[derive(Debug, PartialEq, Clone)]
pub struct ColIndexObj {
    schema_name: Option<String>,
    table_name: String,
    column_name: String,
    suggested_index: String,
    rating: i8,
}

#[derive(Debug, PartialEq, Clone)]
struct IndexRating {
    index: String,
    rating: i8,
}

#[derive(Debug, PartialEq, Clone)]
struct ColAllIndexObj {
    schema_name: Option<String>,
    column_name: String,
    table_name: String,
    suggested_indexes: Vec<IndexRating>,
    totalRating: i8,
}

#[derive(Debug, PartialEq, Clone)]
struct TableIndexObj {
    schema_name: Option<String>,
    table_name: String,
    column_indexes: HashMap<String, Vec<ColAllIndexObj>>,
}

#[derive(Debug, PartialEq, Clone)]
struct SchemaIndexObj {
    schema_name: String,
    table_indexes: Vec<TableIndexObj>,
}

pub struct RuleBasedV1;

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