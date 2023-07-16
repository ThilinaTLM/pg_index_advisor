import psycopg2
import time

def execute_ddl():
    con = psycopg2.connect(database="pgx_advisor", user="yasith", password="21717", host="localhost")

    # drop table if exists
    con.cursor().execute("DROP TABLE IF EXISTS title_akas")
    con.commit()

    with open("./queries/ddl.sql", "r") as f:
        start = time.time()
        con.cursor().execute(f.read())
        print("Table Created!\n")

    con.commit()
    con.cursor().close()
    con.close()