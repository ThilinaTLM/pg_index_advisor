# PG Index Advisor

![PostgreSQL Logo](https://www.postgresql.org/media/img/about/press/elephant.png)

## Introduction

PostgreSQL Index Advisor Extension is an advanced tool designed to assist with optimizing PostgreSQL database performance. This tool is built using the Rust programming language and the Cargo PGX framework, combining the efficiency and safety of Rust with the practicality of PostgreSQL extension development provided by Cargo PGX.

The Index Advisor extension works by analyzing your queries, identifying potential inefficiencies, and suggesting the most suitable indexes to improve performance. Its ultimate goal is to optimize the query execution process by reducing the time and resources required for data lookup and retrieval.

As an extension developed in Rust, it takes advantage of Rust's features like memory safety, zero-cost abstractions, and fearless concurrency, to ensure that the extension is robust, secure, and efficient.

Cargo PGX, on the other hand, makes PostgreSQL extension development more streamlined and approachable by integrating seamlessly with Cargo, Rust's package manager. This way, developers can focus on writing efficient, high-quality code instead of struggling with boilerplate and setup.

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