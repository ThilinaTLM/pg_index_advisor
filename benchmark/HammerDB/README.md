
# HammerDB PostgreSQL Benchmarking

### Create containers
```bash
docker compose up -d
```

Then wait until the HammerDB initialization is completed.

### Start HammerDB CLI
```bash
docker exec -it hammerdb_t ./hammerdbcli
```

### Configure HammerDB

Select the database and TPC:
```bash
dbset db pg
dbset bm TPC-C
```

Set connection details:
```bash
diset connection pg_host localhost
diset connection pg_port 5432
```

Set PostgreSQL user and password:
```bash
diset tpcc pg_superuser root
diset tpcc pg_superuserpass root
diset tpcc pg_defaultdbase pg_test
diset tpcc pg_user root
diset tpcc pg_user root
diset tpcc pg_dbase pg_test
```

### Create Database Schema

Set number of warehouses:
```bash
diset tpcc pg_count_ware 10
```

Set number of virtual users:
```bash
diset tpcc pg_num_vu 10
```

Build the schema:
```bash
buildschema
```

> Now you can check the schema in the PostgreSQL container. You might do additional configuration there such as updating the indexes.

### Run Benchmark

```bash
vudestroy
vucreate
vurun
```