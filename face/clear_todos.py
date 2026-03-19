import json

path = "/Users/zerbytheboss/The_Consortium/agents.json"
with open(path, "r") as f:
    agents = json.load(f)

for agent in agents.values():
    if "mission_critical_todo" in agent:
        agent["mission_critical_todo"] = []
    if "mood" in agent:
        agent["mood"]["arousal"] = 0.5 # Return to normal baseline
        agent["mood"]["valence"] = 0.5 

with open(path, "w") as f:
    json.dump(agents, f, indent=2)

print("Cleared agent to-do queues and normalized arousal states.")
