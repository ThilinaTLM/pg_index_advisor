use super::*;

impl SchemaRepo {
    pub fn get_columns(table_name: &str) -> Result<Vec<String>, spi::Error> {
        let mut result: Vec<String> = Vec::new();

        // Prepare the SQL statement
        let sql = format!(
            "SELECT column_name::text FROM information_schema.columns WHERE table_name = $1",
        );

        // Create a SPI connection
        Spi::connect(|client| {
            // Execute the SQL command
            match client.select(&sql, None, Some(vec![(PgBuiltInOids::TEXTOID.oid(), table_name.into_datum())])) {
                Ok(spi_result) => {
                    for row in spi_result {
                        match row.get_by_name::<String, &str>("column_name") {
                            Ok(Some(column_name)) => result.push(column_name),
                            Ok(None) => (),
                            Err(err) => {
                                info!("Error retrieving column name: {}", err);
                                return Err(err);
                            },
                        }
                    }
                    Ok(())
                },
                Err(err) => {
                    info!("Error executing SQL statement: {}", sql);
                    Err(err)
                }
            }
        })?;

        Ok(result)
    }

    pub fn get_index_columns(table_name: &str) -> Result<Vec<String>, spi::Error> {
        let mut result: Vec<String> = Vec::new();
        let sql = format!("SELECT column_name::text FROM information_schema.columns WHERE table_name = $1 AND column_name IN (SELECT column_name FROM pg_index JOIN pg_attribute ON pg_attribute.attnum = ANY(pg_index.indkey) WHERE indrelid = $1::regclass)");
        Spi::connect(|client| {
            match client.select(&sql, None, Some(vec![(PgBuiltInOids::TEXTOID.oid(), table_name.into_datum())])) {
                Ok(spi_result) => {
                    for row in spi_result {
                        match row.get_by_name::<String, &str>("column_name") {
                            Ok(Some(column_name)) => result.push(column_name),
                            Ok(None) => (),
                            Err(err) => {
                                info!("Error retrieving column name: {}", err);
                                return Err(err);
                            },
                        }
                    }
                    Ok(())
                },
                Err(err) => {
                    info!("Error executing SQL statement: {}", sql);
                    Err(err)
                }
            }
        })?;
        Ok(result)
    }


    pub fn get_table_size(table_name: &str) -> Result<i64, spi::Error> {
        todo!("Implement get_table_size")
    }
    

    // These methods require more complex logic, possibly involving EXPLAIN plans or additional configuration,
    // and thus they are provided as stubs here.
    pub fn get_missing_indexes(table_name: &str) -> Result<Vec<String>, spi::Error> {
        todo!("Implement get_missing_indexes")
    }

    pub fn get_unused_indexes(table_name: &str) -> Result<Vec<String>, spi::Error> {
        todo!("Implement get_unused_indexes")
    }

}
