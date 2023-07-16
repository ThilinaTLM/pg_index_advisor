import psycopg2
import time
import torch
import torch.nn as nn
import torch.optim as optim
import numpy as np
from collections import deque
import random

# Define the Q-Network
class DQN(nn.Module):
    def __init__(self, input_dim, output_dim):
        super(DQN, self).__init__()
        self.fc1 = nn.Linear(input_dim, 128)
        self.fc2 = nn.Linear(128, 256)
        self.fc3 = nn.Linear(256, output_dim)

    def forward(self, x):
        x = torch.relu(self.fc1(x))
        x = torch.relu(self.fc2(x))
        return self.fc3(x)

# Define the agent
class Agent:
    def __init__(self, input_dim, output_dim, learning_rate):
        self.dqn = DQN(input_dim, output_dim)
        self.memory = deque(maxlen=2000)
        self.optimizer = optim.Adam(self.dqn.parameters(), lr=learning_rate)
        self.criterion = nn.MSELoss()
        self.gamma = 0.99

    def update_model(self):
        batch = random.sample(self.memory, 32)
        states, actions, rewards, next_states, dones = zip(*batch)
        states = torch.tensor(states, dtype=torch.float)
        actions = torch.tensor(actions, dtype=torch.long)
        rewards = torch.tensor(rewards, dtype=torch.float)
        next_states = torch.tensor(next_states, dtype=torch.float)
        dones = torch.tensor(dones, dtype=torch.float)

        curr_Q = self.dqn(states).gather(1, actions.unsqueeze(1))
        max_next_Q = self.dqn(next_states).detach().max(1)[0]
        expected_Q = rewards + (1 - dones) * self.gamma * max_next_Q

        loss = self.criterion(curr_Q, expected_Q.unsqueeze(1))
        self.optimizer.zero_grad()
        loss.backward()
        self.optimizer.step()

    def get_action(self, state, action_space, epsilon):
        if np.random.rand() < epsilon:
            print("Random action")
            return random.randint(0, len(action_space)-1)
        else:
            print("Greedy action")
            return self.dqn(torch.tensor(state, dtype=torch.float)).argmax().item()

    def save_model(self, path):
        torch.save(self.dqn.state_dict(), path)

# Define the environment
class Environment:
    def __init__(self, conn, table_name):
        self.conn = conn
        self.table_name = table_name
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
        
        with open("queries/test.sql", "r") as f:
            self.cursor.execute(f.read())

        execution_time = time.time() - start_time

        self.conn.rollback()

        reward = -execution_time
        next_state = self.get_state()
        done = (iteration >= 20) or (execution_time < execution_threshold)

        print(f"Execution time: {execution_time} seconds")

        return next_state, reward, done

learning_rate = 0.001
execution_threshold = 1.7
epoches = 50

# Connect to the PostgreSQL database
conn = psycopg2.connect(database="pgx_advisor", user="yasith", password="21717", host="127.0.0.1", port="5432")

# Initialize the environment and the agent
env = Environment(conn, "title_akas")
agent = Agent(len(env.get_state()), len(env.get_action_space()), learning_rate)

# Train the agent
for epoch in range(epoches):
    print(f"Epoch: {epoch+1} ---------------------------------------")
    state = env.get_state()
    epsilon = max(0.1, 0.8 - 0.01*(epoch))  # Linearly decreasing epsilon
    done = False
    iteration = 0

    while not done:
        iteration += 1
        action = agent.get_action(state, env.get_action_space(), epsilon)
        next_state, reward, done = env.step(action, iteration)
        agent.memory.append((state, action, reward, next_state, done))
        state = next_state

        if len(agent.memory) >= 32:
            agent.update_model()

    if episode % 10 == 0:
        agent.save_model("model/model.pth")

conn.close()
