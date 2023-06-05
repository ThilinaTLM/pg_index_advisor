use sqlparser::ast::{Query, SetExpr};
use crate::query::*;

impl QueryUsageAnalyzer {
    pub fn new() -> Self {
        QueryUsageAnalyzer {
            dialect: PostgreSqlDialect {},
            table_usages: Vec::new(),
        }
    }

    pub fn analyze(&mut self, query: &str) -> Result<Vec<TableUsage>, String> {
        let ast = Parser::parse_sql(&self.dialect, query).expect("Error parsing SQL");

        for statement in ast {
            match statement {
                sqlparser::ast::Statement::Query(query) => {
                    match *query.body {
                        SetExpr::Select(select) => {
                            for item in &select.from {
                                match item.relation {
                                    TableFactor::Table { ref name, .. } => {
                                        self.table_usages.push(TableUsage {
                                            name: name.to_string(),
                                            usages: Vec::new(),
                                        });
                                    },
                                    _ => {},
                                }
                            }
                        },
                        _ => {},
                    }
                }
                sqlparser::ast::Statement::Insert { table_name, .. } => {
                    self.table_usages.push(TableUsage {
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
                        self.analyze_where_clause(&expr).unwrap_or(Vec::new())
                    } else {
                        Vec::new()
                    };

                    self.table_usages.push(TableUsage {
                        name: table_name.to_string(),
                        usages: column_usages,
                    });
                }
                _ => return Err("Invalid statement type".to_string()),
            }
        }

        Ok(self.table_usages.clone())
    }

    fn analyze_where_clause(&self, expr: &Expr) -> Option<Vec<ColumnUsage>> {
        let mut column_usages: Vec<ColumnUsage> = Vec::new();
        match expr {
            Expr::BinaryOp { left, op, .. } => {
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