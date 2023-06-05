#![allow(dead_code)]

use sqlparser::ast::{Assignment, BinaryOperator, Expr, SelectItem, TableFactor, Value};
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

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

struct ColumnUsage {
    column: String,
    operation: OperationType,
}

struct TableUsage {
    name: String,
    usages: Vec<ColumnUsage>,
}

struct QueryUsageAnalyzer {
    dialect: PostgreSqlDialect,
    table_usages: Vec<TableUsage>,
}

impl QueryUsageAnalyzer {
    pub fn new() -> Self {
        QueryUsageAnalyzer {
            dialect: PostgreSqlDialect {},
            table_usages: Vec::new(),
        }
    }

    fn analyze(&self, query: &str) -> Result<Vec<TableUsage>, String> {
        let ast = Parser::parse_sql(&self.dialect, query).expect("Error parsing SQL");

        for statement in ast {
            match statement {
                sqlparser::ast::Statement::Query(query) => {
                    let select = query.body.as_ref();
                    unimplemented!()
                }
                sqlparser::ast::Statement::Insert { table_name, .. } => {
                    table_usages.push(TableUsage {
                        name: table_name.to_string(),
                        usages: Vec::new(),
                    });
                }
                sqlparser::ast::Statement::Update { table, selection, .. } => {
                    // Extract table name
                    let table_name = match table.relation {
                        TableFactor::Table { name, .. } => name.to_string(),
                        _ => return Err("Invalid table name".to_string()),
                    };

                    // Handle selection (WHERE clause)
                    let column_usages = if let Some(expr) = selection {
                        self.analyze_where_clause(&expr)?
                    } else {
                        Vec::new()
                    };

                    table_usages.push(TableUsage {
                        name: table_name.to_string(),
                        usages: column_usages,
                    });
                }
                _ => return Err("Invalid statement type".to_string()),
            }
        }

        Ok(table_usages)
    }

    fn analyze_where_clause(&self, expr: &Expr) -> Option<Vec<ColumnUsage>> {
        let mut column_usages: Vec<ColumnUsage> = Vec::new();
        match expr {
            Expr::BinaryOp { left, op, right } => {
                let column_name = match left.as_ref() {
                    Expr::Identifier(id) => id.to_string(),
                    _ => return None,
                };
                match op {
                    BinaryOperator::Plus
                    | BinaryOperator::Minus
                    | BinaryOperator::Multiply
                    | BinaryOperator::Divide
                    | BinaryOperator::Modulo => {
                        column_usages.push(ColumnUsage {
                            column: column_name,
                            operation: OperationType::Algebraic,
                        })
                    }
                    BinaryOperator::Gt
                    | BinaryOperator::Lt
                    | BinaryOperator::GtEq
                    | BinaryOperator::LtEq
                    | BinaryOperator::Eq
                    | BinaryOperator::NotEq
                    | BinaryOperator::And
                    | BinaryOperator::Or
                    | BinaryOperator::Xor
                    | BinaryOperator::BitwiseOr
                    | BinaryOperator::BitwiseAnd
                    | BinaryOperator::BitwiseXor => {
                        column_usages.push(ColumnUsage {
                            column: column_name,
                            operation: OperationType::Comparison,
                        })
                    }
                    BinaryOperator::StringConcat => {
                        column_usages.push(ColumnUsage {
                            column: column_name,
                            operation: OperationType::String,
                        })
                    }
                    _ => {}
                }
            }
            Expr::Like { expr, .. }
            | Expr::ILike { expr, .. }
            | Expr::SimilarTo { expr, .. } => {
                if let Expr::Identifier(id) = expr.as_ref() {
                    column_usages.push(ColumnUsage {
                        column: id.to_string(),
                        operation: OperationType::StringPattern,
                    });
                }
            }
            Expr::IsNull(expr)
            | Expr::IsNotNull(expr) => {
                if let Expr::Identifier(id) = expr.as_ref() {
                    column_usages.push(ColumnUsage {
                        column: id.to_string(),
                        operation: OperationType::NullCheck,
                    });
                }
            }
            _ => (),
        }
        Some(column_usages)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_query_001() {
        let query = "SELECT * FROM users;";
        let result = collect_usages_from_query(query);
        assert!(result.is_ok());
    }

    #[test]
    fn test_insert_query_001() {
        let query = "INSERT INTO users (name, age) VALUES ('John', 30);";
        let result = collect_usages_from_query(query);
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_query_001() {
        let query = "UPDATE users SET name = 'John' WHERE id = 1;";
        let result = collect_usages_from_query(query);
        assert!(result.is_ok());
    }
}


