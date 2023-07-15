use sqlparser::ast::{Query, SetExpr, Statement, TableWithJoins};

use crate::parser::*;

#[derive(Debug, PartialEq, Clone)]
pub enum QueryParserError {
    InvalidStatementType,
    UnsupportedOperation,
    ParseError,
    InvalidTableName,
}

pub struct QueryParser {
    usages: Vec<ColumnUsage>,
}

impl QueryParser {
    pub fn new() -> Self {
        QueryParser {
            usages: Vec::new(),
        }
    }

    pub fn record_usage(&mut self, table_name: &str, column_name: &str, operation: OperationType) {
        self.usages.push(ColumnUsage {
            schema: "".to_string(),
            table: table_name.to_string(),
            column: column_name.to_string(),
            operation,
        });
    }

    pub fn parse(&mut self, query: &str) -> Result<Vec<ColumnUsage>, QueryParserError> {
        let ast: Vec<Statement> = Parser::parse_sql(&PostgreSqlDialect {}, query)
            .map_err(|_| QueryParserError::ParseError)?;



        for statement in ast {
            match statement {
                Statement::Query(query) => {
                    match *query.body {
                        SetExpr::Select(select) => {
                            let table_name = match &select.from.first() {
                                Some(TableWithJoins { ref relation, .. }) => {
                                    match relation {
                                        TableFactor::Table { ref name, .. } => name.to_string(),
                                        _ => return Err(QueryParserError::InvalidTableName),
                                    }
                                }
                                _ => return Err(QueryParserError::InvalidTableName),
                            };

                            // Handle selection (WHERE clause)
                            let column_usages = if let Some(expr) = select.selection {
                                self.analyze_where_clause(&expr).unwrap_or(Vec::new())
                            } else {
                                Vec::new()
                            };

                            // self.table_usages.push(TableUsage {
                            //     name: table_name.to_string(),
                            //     usages: column_usages,
                            // });
                        }
                        _ => {}
                    }
                }
                Statement::Insert { table_name, .. } => {
                    // self.table_usages.push(TableUsage {
                    //     name: table_name.to_string(),
                    //     usages: Vec::new(),
                    // });
                }
                Statement::Update { table, selection, .. } => {
                    // Extract table name
                    let table_name = match table.relation {
                        TableFactor::Table { name, .. } => name.to_string(),
                        _ => return Err(QueryParserError::InvalidTableName),
                    };

                    // Handle selection (WHERE clause)
                    let column_usages = if let Some(expr) = selection {
                        self.analyze_where_clause(&expr).unwrap_or(Vec::new())
                    } else {
                        Vec::new()
                    };

                    // self.table_usages.push(TableUsage {
                    //     name: table_name.to_string(),
                    //     usages: column_usages,
                    // });
                }
                _ => return Err(QueryParserError::InvalidStatementType),
            }
        }

        Ok(self.usages.clone())
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
                            schema: "".to_string(),
                            table: "".to_string(),
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
                            schema: "".to_string(),
                            table: "".to_string(),
                            column: column_name,
                            operation: OperationType::Comparison,
                        })
                    }
                    BinaryOperator::StringConcat => {
                        column_usages.push(ColumnUsage {
                            schema: "".to_string(),
                            table: "".to_string(),
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
                        schema: "".to_string(),
                        table: "".to_string(),
                        column: id.to_string(),
                        operation: OperationType::StringPattern,
                    });
                }
            }
            Expr::IsNull(expr)
            | Expr::IsNotNull(expr) => {
                if let Expr::Identifier(id) = expr.as_ref() {
                    column_usages.push(ColumnUsage {
                        schema: "".to_string(),
                        table: "".to_string(),
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