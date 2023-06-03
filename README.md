# PG Index Advisor

## Introduction


## Test

1. Run following command to start pgx managed instance
    ```bash
    cargo pgx run
    ```

2. Install extension
    ```sql
    CREATE EXTENSION pg_index_advisor;
    ```

3. Run following command to run tests
    ```bash
    SELECT hello_pg_index_advisor();
    ```