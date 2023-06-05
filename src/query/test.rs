#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_select_query_001() {
        let query = "SELECT * FROM users;";
        let mut analyzer = QueryUsageAnalyzer::new();
        let result = analyzer.analyze(query);
        assert_eq!(result, Ok(vec![TableUsage {
            name: "users".to_string(),
            usages: Vec::new(),
        }]));
    }

    #[test]
    fn test_select_query_002() {
        let query = "SELECT * FROM users WHERE id = 1;";
        let mut analyzer = QueryUsageAnalyzer::new();
        let result = analyzer.analyze(query);
        assert_eq!(result, Ok(vec![TableUsage {
            name: "users".to_string(),
            usages: vec![ColumnUsage {
                column: "id".to_string(),
                operation: OperationType::Comparison,
            }],
        }]));
    }

    #[test]
    fn test_insert_query_001() {
        let query = "INSERT INTO users (name, age) VALUES ('John', 30);";
        let mut analyzer = QueryUsageAnalyzer::new();
        let result = analyzer.analyze(query);
        assert_eq!(result, Ok(vec![TableUsage {
            name: "users".to_string(),
            usages: Vec::new(),
        }]));
    }

    #[test]
    fn test_update_query_001() {
        let query = "UPDATE users SET name = 'John' WHERE id = 1;";
        let mut analyzer = QueryUsageAnalyzer::new();
        let result = analyzer.analyze(query);
        assert_eq!(result, Ok(vec![TableUsage {
            name: "users".to_string(),
            usages: vec![ColumnUsage {
                column: "id".to_string(),
                operation: OperationType::Comparison,
            }],
        }]));
    }
}


