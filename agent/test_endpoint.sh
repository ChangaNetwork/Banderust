#!/bin/bash

# Function to send a message
send_message() {
    local message_text="$1"
    local response=$(curl -s -X POST \
      http://127.0.0.1:8000/run \
      -H "Content-Type: application/json" \
      --data-raw '{
        "app_name": "multi_tool_agent",
        "user_id": "user_1",
        "session_id": "'"$SESSION_ID"'",
        "new_message": {
          "parts": [{"text": "'"$message_text"'"}],
          "role": "user"
        },
        "streaming": false
      }')
    echo "$response"
}

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

# Send first message
echo "Sending message 'aaaatest'..."
RESPONSE1=$(send_message "aaaatest")
echo "Message response:"
echo "$RESPONSE1"

# Send second message
echo "Sending message 'choice a'..."
RESPONSE2=$(send_message "choice a")
echo "Message response:"
echo "$RESPONSE2"

