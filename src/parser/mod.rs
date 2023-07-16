#![allow(dead_code)]

use sqlparser::ast::Query;
use sqlparser::ast::{
    Assignment, BinaryOperator, Expr, Select, SelectItem, SetExpr, Statement, TableFactor, Value,
};
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

mod test;

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
    schema: String,
    table: String,
    column: String,
    operation: OperationType,
    weight: i32,
}

pub fn extract(query: &str) -> Vec<ColumnUsage> {
    let ast: Vec<Statement> = Parser::parse_sql(&PostgreSqlDialect {}, &query).unwrap();
    let mut column_usages = Vec::new();
    for statement in ast {
        match statement {
            Statement::Query(query) => {
                let mut select_column_usages = extract_query(query);
                column_usages.append(&mut select_column_usages);
            }
            _ => {
                todo!("Implement this")
            }
        }
    }
    column_usages
}

fn extract_query(query: Box<Query>) -> Vec<ColumnUsage> {
    let mut column_usages = Vec::new();

    // analyze body
    match *query.body {
        SetExpr::Select(select) => {
            let mut select_column_usages = extract_select(select);
            column_usages.append(&mut select_column_usages);
        }
        _ => {
            todo!("Implement this")
        }
    }

    column_usages
}

fn extract_select(query: Box<Select>) -> Vec<ColumnUsage> {
    let mut column_usages = Vec::new();

    // Extract from WHERE clause
    if let Some(selection) = &query.selection {
        extract_from_expr(selection, &mut column_usages, 1);
    }

    // Extract from GROUP BY clause
    for expr in &query.group_by {
        extract_from_expr(expr, &mut column_usages, 1);
    }

    // Extract from HAVING clause
    if let Some(having) = &query.having {
        extract_from_expr(having, &mut column_usages, 1);
    }

    // Extract from SELECT projection expressions
    for item in &query.projection {
        match item {
            SelectItem::UnnamedExpr(expr) => {
                extract_from_expr(&expr, &mut column_usages, 1);
            }
            SelectItem::ExprWithAlias { expr, .. } => {
                extract_from_expr(&expr, &mut column_usages, 1);
            }
            _ => {}
        }
    }

    // You can add more extraction logic for other parts of the SELECT statement here

    column_usages
}

fn extract_from_expr(expr: &Expr, column_usages: &mut Vec<ColumnUsage>, depth: i32) {
    match expr {
        Expr::BinaryOp { left, op, right } => {
            // Recurse into left and right expressions
            extract_from_expr(left, column_usages, depth);
            extract_from_expr(right, column_usages, depth);

            // Assign operations based on your criteria
            let operation = match op {
                BinaryOperator::Plus
                | BinaryOperator::Minus
                | BinaryOperator::Multiply
                | BinaryOperator::Divide => OperationType::Algebraic,
                BinaryOperator::Eq
                | BinaryOperator::NotEq
                | BinaryOperator::Lt
                | BinaryOperator::LtEq
                | BinaryOperator::Gt
                | BinaryOperator::GtEq => OperationType::Comparison,
                BinaryOperator::And | BinaryOperator::Or | BinaryOperator::NotEq => {
                    OperationType::Logical
                }
                _ => OperationType::Other,
            };

            // Weight as a function of depth (in this example, weight is equal to depth)
            let weight = depth;

            // (you can also populate schema and table here, for this example, I am leaving it empty)
            if let Expr::Identifier(identifier) = &**left {
                column_usages.push(ColumnUsage {
                    schema: String::new(),
                    table: String::new(),
                    column: identifier.value.clone(),
                    operation,
                    weight,
                });
            }
        }
        Expr::Nested(nested_expr) => {
            // Recurse into nested expressions, incrementing the depth
            extract_from_expr(nested_expr, column_usages, depth + 1);
        }
        Expr::Subquery(subquery) => {
            // Extract from subquery, incrementing the depth
            let mut subquery_usages = extract_query(subquery.clone());
            for usage in &mut subquery_usages {
                usage.weight += depth; // Or some other function to modify weight based on depth
            }
            column_usages.append(&mut subquery_usages);
        }
        Expr::Identifier(_) => {
            // This is a column identifier
        }
        _ => {
            // Handle other expression types and recurse as needed
        }
    }
}
