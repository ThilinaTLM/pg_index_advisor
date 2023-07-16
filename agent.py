import torch
import torch.nn as nn
import torch.optim as optim
import numpy as np
from collections import deque
import random
from model import DQN

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
            return random.choice(action_space)
        else:
            return self.dqn(torch.tensor(state, dtype=torch.float)).argmax().item()

    def save_model(self, path):
        torch.save(self.dqn.state_dict(), path)
