use pgx::spi::Spi;
use pgx::prelude::*;

pub struct SchemaRepo;

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
}
