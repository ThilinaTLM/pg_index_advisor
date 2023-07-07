#[cfg(test)]

#[allow(unused_assignments)]

mod tests {
    use super::*;
    use super::super::*;

    #[test]
    fn test_generate_index_suggestion() {
        // Create sample statistics
        let stats1 = Statistics {
            schema_name: None,
            table_name: "table1".to_string(),
            column_name: "column1".to_string(),
            column_dtype: "array".to_string(),
            row_count: 20000,
            page_count: 100,
            n_distinct: 100.0,
            rating: 5,
            operator: "=".to_string(),
            most_common_vals: None,
            most_common_freqs: None,
        };

        let stats2 = Statistics {
            schema_name: None,
            table_name: "table2".to_string(),
            column_name: "column2".to_string(),
            column_dtype: "integer".to_string(),
            row_count: 5000,
            page_count: 50,
            n_distinct: 50.0,
            rating: 3,
            operator: "<=".to_string(),
            most_common_vals: None,
            most_common_freqs: None,
        };

        let stats3 = Statistics {
            schema_name: None,
            table_name: "table2".to_string(),
            column_name: "column3".to_string(),
            column_dtype: "integer".to_string(),
            row_count: 25000,
            page_count: 50,
            n_distinct: 50.0,
            rating: 3,
            operator: "<=".to_string(),
            most_common_vals: None,
            most_common_freqs: None,
        };

        // Create the statement
        let query = "SELECT * FROM table1 WHERE column1 = 123";
        let statement = parse_query(query).unwrap();

        // Create the statistics array
        let stats_arr: Vec<&Statistics> = vec![&stats1, &stats2, &stats3];

        // Call the method
        let result = RuleBasedV1.generate_index_suggestion(&statement, &stats_arr);

        let table_index = RuleBasedV1.generate_table_index_obj( &result);

        // Assert the result
        assert_eq!(result.len(), 3);

        // Check the first index suggestion
        let index1 = &result[0];
        println!("{:?}", index1);
        assert_eq!(index1.table_name, "table1");
        assert_eq!(index1.column_name, "column1");
        assert_eq!(index1.suggested_index, "GIN");
        assert_eq!(index1.rating, 5);

        // Check the second index suggestion
        let index2 = &result[1];
        print!("{:?}", index2);
        assert_eq!(index2.table_name, "table2");
        assert_eq!(index2.column_name, "column2");
        assert_eq!(index2.suggested_index, "B-Tree");
        assert_eq!(index2.rating, 3);

        // Check the third index suggestion
        let index3 = &result[2];
        print!("{:?}", index3);
        assert_eq!(index3.table_name, "table2");
        assert_eq!(index3.column_name, "column3");
        assert_eq!(index3.suggested_index, "BRIN");
        assert_eq!(index3.rating, 3);
    }
}
