use pgx::prelude::*;

mod parser;
mod repos;

// mod algorithm;

pg_module_magic!();


#[pg_extern]
fn pgia_health_check() -> String {
    "OK".to_string()
}

#[pg_extern]
fn pgia_test() -> String {
    let result = repos::SchemaRepo::get_columns("test_table");
    match result {
        Ok(columns) => {
            let mut result = String::new();
            for column in columns {
                result.push_str(&column);
                result.push_str(", ");
            }
            result
        },
        Err(err) => {
            format!("Error: {}", err)
        }
    }
}


#[cfg(any(test, feature = "pg_test"))]
mod tests {
    // #[pg_test]
    // fn test_health_check() {

    //     assert_eq!("OK", "OK");
    // }
}

/// This module is required by `cargo pgx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
