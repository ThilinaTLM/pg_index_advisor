
Data models for the parser module.

```rust
#[derive(Debug, PartialEq, Clone)]
enum OperationType {
    Algebraic(String),
    Comparison(String),
    Logical(String),
    StringPattern(String),
    String,
    Regex,
    NullCheck,
    Other,
}

#[derive(Debug, PartialEq, Clone)]
struct ColumnUsage {
    schema: String,
    table: String,
    column: String,
    operation: OperationType,
    weight: usize,
    metadata: Box<dyn any>,
}

#[derive(Debug, PartialEq, Clone)]
enum DataType {
    Integer,
    Float,
    String,
    Date,
    Boolean,
    Complex,
    Other, 
}


trait SchemaRepo {
    fn get_column_data_type(schema: String, table: String, column: String) -> DataType;
}

trait StatRepo {
    fn get_row_count(schema: String, table: String, column: String) -> usize;
    fn get_distinct_count(schema: String, table: String, column: String) -> usize;
    fn get_null_count(schema: String, table: String, column: String) -> usize;
    fn get_page_count(schema: String, table: String, column: String) -> usize;
}

trait QueryStatRepo {
    fn get_query_frequency(schema: String, query: String) -> usize;
}
```