
```rust
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
struct ColumnUsage {
    schema: String,
    table: String,
    column: String,
    operation: OperationType,
    weight: usize,
    metadata: Box<dyn any>,
}
```