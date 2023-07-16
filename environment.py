import time

class Environment:
    def __init__(self, conn, table_name, execution_threshold):
        self.conn = conn
        self.table_name = table_name
        self.execution_threshold = execution_threshold
        self.cursor = conn.cursor()
        self.indexes = ['btree', 'hash', 'gist', 'spgist', 'gin', 'brin', 'None']

        self.cursor.execute(f"SELECT column_name FROM information_schema.columns WHERE table_name = '{self.table_name}'")
        columns = self.cursor.fetchall()
        self.state = [0 for _ in columns for i in self.indexes]

    def get_state(self):
        return self.state

    def get_action_space(self):
        self.cursor.execute(f"SELECT column_name FROM information_schema.columns WHERE table_name = '{self.table_name}'")
        columns = self.cursor.fetchall()
        action_space = [(column[0], index) for column in columns for index in self.indexes]
        return action_space

    def step(self, action, iteration):
        column, index = self.get_action_space()[action]
        if index == 'None':
            print("None")
            self.cursor.execute(f"DROP INDEX IF EXISTS idx_{column}")
            self.state[action] = 0
        else:
            print("Not None")
            self.cursor.execute(f"DROP INDEX IF EXISTS idx_{column}")
            self.conn.commit()
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