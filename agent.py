import torch
import torch.nn as nn
import torch.optim as optim
import numpy as np
from collections import deque
import random

from helper import get_device
from model import DQN

class Agent:
    def __init__(self, input_dim, output_dim, learning_rate):
        self.dqn = DQN(input_dim, output_dim).to(get_device())
        self.memory = deque(maxlen=2000)
        self.optimizer = optim.Adam(self.dqn.parameters(), lr=learning_rate)
        self.criterion = nn.MSELoss()
        self.gamma = 0.99

    def update_model(self):
        batch = random.sample(self.memory, 32)
        states, actions, rewards, next_states, dones = zip(*batch)
        states = torch.tensor(states, dtype=torch.float).to(get_device())
        actions = torch.tensor(actions, dtype=torch.long).to(get_device())
        rewards = torch.tensor(rewards, dtype=torch.float).to(get_device())
        next_states = torch.tensor(next_states, dtype=torch.float).to(get_device())
        dones = torch.tensor(dones, dtype=torch.float).to(get_device())

        curr_Q = self.dqn(states).gather(1, actions.unsqueeze(1))
        max_next_Q = self.dqn(next_states).detach().max(1)[0]
        expected_Q = rewards + (1 - dones) * self.gamma * max_next_Q

        loss = self.criterion(curr_Q, expected_Q.unsqueeze(1))
        self.optimizer.zero_grad()
        loss.backward()
        self.optimizer.step()

    def get_action(self, state, action_space, epsilon):
        if np.random.rand()*1.5 < epsilon:
            return random.randint(0, len(action_space)-1)
        else:
            return self.dqn(torch.tensor(state, dtype=torch.float).to(get_device())).argmax().item()

    def save_model(self, path):
        torch.save(self.dqn.state_dict(), path)
