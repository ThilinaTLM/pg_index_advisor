import time
import re

class Environment:
    def __init__(self, conn, table_name, execution_threshold):
        self.conn = conn
        self.table_name = table_name
        self.execution_threshold = execution_threshold
        self.cursor = conn.cursor()
        self.indexes = ['btree', 'hash', 'gist', 'spgist', 'gin', 'brin', 'None']

        self.cursor.execute(f"SELECT column_name FROM information_schema.columns WHERE table_name = '{self.table_name}'")
        self.columns = self.cursor.fetchall()
        self.state = [0 for _ in self.columns for i in self.indexes]

    def get_state(self):
        return self.state

    def get_action_space(self):
        action_space = [(column[0], index) for column in self.columns for index in self.indexes]
        return action_space

    def step(self, action, iteration):
        column, index = self.get_action_space()[action]

        try:
            if index == 'None':
                self.cursor.execute(f"DROP INDEX IF EXISTS idx_{column}")
                self.state[action] = 0
            else:
                self.cursor.execute(f"SELECT indexname, indexdef FROM pg_indexes WHERE tablename = '{self.table_name}' AND indexname LIKE '%{column}'")
                rows = self.cursor.fetchall()
                for row in rows:
                    self.cursor.execute(f"DROP INDEX IF EXISTS {row[0]}")
                    self.conn.commit()

                    # remove the index from the state
                    match = re.search(r'USING (\w+)', row[1])

                    if match:
                        index_type = match.group(1)
                        self.state[len(self.indexes)*self.indexes.index(index_type) + self.columns.index(column)] = 0

                self.cursor.execute(f"CREATE INDEX idx_{column} ON {self.table_name} USING {index} ({column})")
                self.state[action] = 1
            self.conn.commit()

            start_time = time.time()
            
            with open("queries/dml.sql", "r") as f:
                self.cursor.execute(f.read())

            execution_time = time.time() - start_time

            self.conn.rollback()

            reward = -execution_time
            next_state = self.get_state()
            done = (iteration >= 20) or (execution_time < self.execution_threshold)

            print(f"Execution time: {execution_time} seconds")

            return next_state, reward, done
        
        except Exception as e:
            self.conn.rollback()
            return self.get_state(), -999999999, False