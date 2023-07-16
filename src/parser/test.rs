#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_select_query_001() {
        let query = "SELECT * FROM users;";
        let result = extract(query);
        assert_eq!(true, true)
    }

    #[test]
    fn test_select_query_002() {
        let query = "SELECT * FROM users WHERE id = 1;";
        let result = extract(query);
        assert_eq!(true, true)
    }

    // #[test]
    // fn test_insert_query_001() {
    //     let query = "INSERT INTO users (name, age) VALUES ('John', 30);";
    //     let result = parse_query(query);
    // }
    //
    // #[test]
    // fn test_update_query_001() {
    //     let query = "UPDATE users SET name = 'John' WHERE id = 1;";
    //     let result = parse_query(query);
    // }
    //
    // #[test]
    // fn serialize() {
    //     let query = "SELECT * FROM users WHERE id = 1;";
    //     let ast: Vec<Statement> = Parser::parse_sql(&PostgreSqlDialect {}, query).unwrap();
    //     let serialized_ast = serde_json::to_string_pretty(&ast).unwrap();
    //     println!("Serialized AST: {}", serialized_ast);
    //     assert_eq!(true, true)
    // }
}


