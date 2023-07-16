import psycopg2
import torch

def execute_ddl():
    con = psycopg2.connect(database="pgx_advisor", user="yasith", password="21717", host="localhost")

    # drop table if exists
    con.cursor().execute("DROP TABLE IF EXISTS title_akas")
    con.commit()

    with open("./queries/ddl.sql", "r") as f:
        con.cursor().execute(f.read())
        print("Table Created!\n")

    con.commit()
    con.cursor().close()
    con.close()

def get_device():
    return torch.device("cuda" if torch.cuda.is_available() else "cpu")