import psycopg2
import time

con = psycopg2.connect(database="pgx_advisor", user="yasith", password="21717", host="localhost")

with open("./queries/title_akas.sql", "r") as f:
    start = time.time()
    con.cursor().execute(f.read())
    print(f"Execution time: {time.time() - start} seconds")

con.commit()
con.cursor().close()
con.close()