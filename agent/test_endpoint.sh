#!/bin/bash

# First request to create a session
echo "Creating session..."
SESSION_RESPONSE=$(curl -s -X POST \
  http://127.0.0.1:8000/apps/multi_tool_agent/users/user_1/sessions)

# Extract session_id from the response using jq
SESSION_ID=$(echo "$SESSION_RESPONSE" | jq -r '.id')

if [ -z "$SESSION_ID" ]; then
  echo "Failed to create session or get session ID"
  echo "Response: $SESSION_RESPONSE"
  exit 1
fi

echo "Session created with ID: $SESSION_ID"

# Second request using the session_id
echo "Sending message..."
MESSAGE_RESPONSE=$(curl -s -X POST \
  http://127.0.0.1:8000/run \
  -H "Content-Type: application/json" \
  --data-raw '{
    "app_name": "multi_tool_agent",
    "user_id": "user_1",
    "session_id": "'"$SESSION_ID"'",
    "new_message": {
      "parts": [{"text": "aaaatest"}],
      "role": "user"
    },
    "streaming": false
  }')

echo "Message response:"
echo "$MESSAGE_RESPONSE"
