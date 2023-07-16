import psycopg2
import time
import torch
import torch.nn as nn
import random
import numpy as np
import re

from helper import execute_ddl

# Define the Q-Network
class DQN(nn.Module):
    def __init__(self, input_dim, output_dim):
        super(DQN, self).__init__()
        self.fc1 = nn.Linear(input_dim, 128)
        self.fc2 = nn.Linear(128, 256)
        self.fc3 = nn.Linear(256, 256)
        self.fc4 = nn.Linear(256, output_dim)

    def forward(self, x):
        x = torch.relu(self.fc1(x))
        x = torch.relu(self.fc2(x))
        x = torch.relu(self.fc3(x))
        return self.fc4(x)

# Define the agent
class Agent:
    def __init__(self, input_dim, output_dim):
        self.dqn = DQN(input_dim, output_dim)
        self.dqn.load_state_dict(torch.load("model/model.pth"))

    def get_action(self, state, action_space, epsilon):
        if np.random.rand() < epsilon:
            return random.randint(0, len(action_space)-1)
        else:
            return self.dqn(torch.tensor(state, dtype=torch.float)).argmax().item()

# Define the environment
class Environment:
    def __init__(self, conn, table_name):
        self.conn = conn
        self.table_name = table_name
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
    
    def get_initial_execution_time(self):
        start_time = time.time()
        
        with open("queries/test/dml_laptop_detail.sql", "r") as f:
            self.cursor.execute(f.read())

        with open("queries/train/query_laptop_detail.sql", "r") as f:
            self.cursor.execute(f.read())

        execution_time = time.time() - start_time

        self.conn.rollback()

        return execution_time

    def step(self, action, iteration, execution_threshold):
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
            
            with open("queries/test/dml_laptop_detail.sql", "r") as f:
                self.cursor.execute(f.read())

            with open("queries/test/query_laptop_detail.sql", "r") as f:
                self.cursor.execute(f.read())

            execution_time = time.time() - start_time

            self.conn.rollback()

            reward = -execution_time
            next_state = self.get_state()
            done = (iteration >= 50) or (execution_time < execution_threshold)

            print(f"Execution time: {execution_time} seconds")

            return next_state, reward, execution_time, done
        
        except Exception as e:
            self.conn.rollback()
            return self.get_state(), -999999999, None, False

# Execute the DDL
execute_ddl()

# Connect to the PostgreSQL database
conn = psycopg2.connect(database="pgx_advisor", user="yasith", password="21717", host="127.0.0.1", port="5432")

# Initialize the environment and the agent
env = Environment(conn, "laptop_detail")
agent = Agent(len(env.get_state()), len(env.get_action_space()))

# Evaluate the agent
state = env.get_state()
done = False
iteration = 0
initial_execution_time = env.get_initial_execution_time()
execution_threshold = initial_execution_time

while not done:
    iteration += 1
    epsilon = max(0.1, 0.5 - 0.05*(iteration))
    action = agent.get_action(state, env.get_action_space(), epsilon)
    next_state, reward, final_execution_time, done = env.step(action, iteration, execution_threshold)
    state = next_state

print("\nSuggeted Indexes:")
cur = conn.cursor()
cur.execute("SELECT indexname, indexdef FROM pg_indexes WHERE tablename = 'laptop_detail'")
rows = cur.fetchall()
for row in rows:
    match_c = re.search(r'idx_(\w+)', row[0])
    match_i = re.search(r'USING (\w+)', row[1])

    if match_c and match_i:
        print(f"{match_c.group(1)}: {match_i.group(1)}")

print(f"Performance Gain: {((initial_execution_time - final_execution_time)/final_execution_time)*100}%")

conn.close()
