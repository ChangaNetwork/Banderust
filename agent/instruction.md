# How to run the agent

0. Go to the agent folder
From the root of the project: `cd agent`
1. Create virtual environment
`python -m venv .venv`
3. Activate environment 
`source .venv/bin/activate`
4. Install libraries
`pip install google-adk litellm`
5. Optional declare variables
`export OLLAMA_API_BASE="http://localhost:11434/"`
4. Launch adk 
`adk web`

## Optional
Modify agent configuration at `agent/multi_tool_agent/agent.py` 

[guide reference](https://google.github.io/adk-docs/get-started/installation/)
