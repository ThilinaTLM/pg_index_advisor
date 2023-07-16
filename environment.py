import time

class Environment:
    def __init__(self, conn, table_name, execution_threshold):
        self.conn = conn
        self.table_name = table_name
        self.execution_threshold = execution_threshold
        self.cursor = conn.cursor()
        self.indexes = ['btree', 'hash', 'gist', 'spgist', 'gin', 'brin', 'None']

        self.cursor.execute(f"SELECT column_name FROM information_schema.columns WHERE table_name = '{self.table_name}' ORDER BY column_name")
        self.columns = self.cursor.fetchall()
        self.state = [0 for i in self.indexes for _ in self.columns]

    def get_state(self):
        return self.state

    def get_action_space(self):
        action_space = [(column[0], index) for index in self.indexes for column in self.columns]
        return action_space

    def step(self, action, iteration):
        column, index = self.get_action_space()[action]
        i = (action//len(self.columns))*len(self.columns)

        try:
            if index == 'None':
                self.cursor.execute(f"DROP INDEX IF EXISTS idx_{column}")
                for idx in range(i, i+len(self.indexes)):
                    self.state[idx] = 0 
            else:
                self.cursor.execute(f"SELECT indexname FROM pg_indexes WHERE tablename = '{self.table_name}' AND indexdef LIKE '%{column}%'")
                rows = self.cursor.fetchall()
                for row in rows:
                    self.cursor.execute(f"DROP INDEX IF EXISTS {row[0]}")
                    self.conn.commit()

                    # remove the index from the state
                    for idx in range(i, i+len(self.indexes)):
                        self.state[idx] = 0 

                self.cursor.execute(f"CREATE INDEX idx_{column} ON {self.table_name} USING {index} ({column})")
                self.state[action] = 1
            self.conn.commit()

            start_time = time.time()
            
            with open("queries/train/dml_laptop_detail.sql", "r") as f:
                self.cursor.execute(f.read())

            with open("queries/train/query_laptop_detail.sql", "r") as f:
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