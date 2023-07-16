import time

class Environment:
    def __init__(self, conn, table_name):
        self.conn = conn
        self.table_name = table_name
        self.cursor = conn.cursor()
        self.indexes = ['B-Tree', 'Hash', 'GiST', 'SP-GiST', 'BRIN', 'GIN', 'None']

    def get_state(self):
        self.cursor.execute(f"SELECT column_name FROM information_schema.columns WHERE table_name = '{self.table_name}'")
        columns = self.cursor.fetchall()
        state = [0 if index == 'None' else 1 for index in self.indexes for _ in columns]
        return state

    def get_action_space(self):
        self.cursor.execute(f"SELECT column_name FROM information_schema.columns WHERE table_name = '{self.table_name}'")
        columns = self.cursor.fetchall()
        action_space = [(column[0], index) for column in columns for index in self.indexes]
        return action_space

    def step(self, action):
        column, index = action
        if index == 'None':
            self.cursor.execute(f"DROP INDEX IF EXISTS idx_{column}")
        else:
            self.cursor.execute(f"CREATE INDEX idx_{column} ON {self.table_name} USING {index}({column})")
        self.conn.commit()

        start_time = time.time()
        # Execute your queries here
        # self.cursor.execute(...)
        execution_time = time.time() - start_time

        reward = -execution_time
        next_state = self.get_state()
        done = False  # Define your termination condition here

        return next_state, reward, done
