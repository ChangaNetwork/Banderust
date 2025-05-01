# Start server for local development 
`trunk serve --no-autoreload true`
- We need autoreload set to true, otherwise when receiving the response from the agent the page will refresh and will lose the session and the response.
- You must first start the agent, see the file `agent/instruction.md`.
