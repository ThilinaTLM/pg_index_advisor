import psycopg2

from helper import execute_ddl, get_device
from environment import Environment
from agent import Agent

learning_rate = 0.001
execution_threshold = 0.05
epoches = 100

# Execute the DDL
execute_ddl()

# Connect to the PostgreSQL database
conn = psycopg2.connect(database="pgx_advisor", user="yasith", password="21717", host="127.0.0.1", port="5432")

# Initialize the environment and the agent
env = Environment(conn, "title_akas", execution_threshold)
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

    if epoch % 10 == 0:
        agent.save_model("model/model.pth")

conn.close()
