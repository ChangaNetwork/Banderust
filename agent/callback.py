import datetime
from zoneinfo import ZoneInfo
from google.adk.agents import SequentialAgent, Agent
from google.adk.models.lite_llm import LiteLlm
import json
from typing import Dict, Any, Optional
from google.adk.agents.callback_context import CallbackContext
from google.adk.sessions import InMemorySessionService
from google.adk.runners import Runner
from google.genai import types
from google.adk.models import LlmResponse, LlmRequest
from google.adk.artifacts import InMemoryArtifactService
from multi_tool_agent.utils import save_model_response, modify_output

import litellm
litellm._turn_on_debug()

AGENT = "ollama_chat/gemma3:1b"

def simple_before_model_modifier(
    callback_context: CallbackContext, llm_request: LlmRequest
) -> Optional[LlmResponse]:
    """Inspects/modifies the LLM request or skips the call."""
    agent_name = callback_context.agent_name
    print(f"[Callback] Before model call for agent: {agent_name}")
    print(f"[Callback] Inspecting contents: '{llm_request.contents}'")

    if not llm_request.contents:
        return None 
    # Extract all text parts from all Content objects
    all_text_parts = []
    for content in llm_request.contents:
        for part in content.parts:
            if part.text:  # Only include parts with text
                all_text_parts.append(part.text)
    
    # If no text was found, return original request
    if not all_text_parts:
        return None

    merged_text = "\n".join(all_text_parts)
    print(f"merged text: {merged_text}")
    llm_request.contents = [
        types.Content(
            parts=[types.Part(text=merged_text)],
            role="user"  # or "assistant" if needed
        )
    ]
    
    return None 


def check_if_agent_should_run(callback_context: CallbackContext) -> Optional[types.Content]:
    """
    Logs entry and checks 'skip_llm_agent' in session state.
    If True, returns Content to skip the agent's execution.
    If False or not present, returns None to allow execution.
    """
    agent_name = callback_context.agent_name
    invocation_id = callback_context.invocation_id
    current_state = callback_context.state.to_dict()

    print(f"\n[Callback] Entering agent: {agent_name} (Inv: {invocation_id})")
    print(f"[Callback] Current State: {current_state}")
    
    return None

def combine_outputs_after_agent(callback_context: CallbackContext) -> Optional[types.Content]:
    """
    Callback function to combine multiple array outputs into a single string
    after each agent completes its execution.
    """
    # Get the current state and outputs
    
    # You can also add logging if needed
    print(f"[CombineOutputs] After agent {callback_context.agent_name}")
    print(f"Full state: {callback_context.state.to_dict()}")
    print(f"User content {callback_context.user_content}")
    return None

# --- Definizione dell'Agente con Temperatura ---
step1 = Agent(
    model=LiteLlm(
        model=AGENT,
        temperature=0.8,  # Slightly lower for coherence, but keeps creativity
    ),
    name="setting_agent",
    description=(
        "Generates a short, atmospheric setting for a text-based adventure game "
        "based on input keywords/themes. Output sets the stage for actionable choices "
        "(e.g., 'choice_agent'). Focus on sensory details and looming stakes."
    ),
    instruction="""Generate a 3-5 sentence setting for a textual adventure game that:
1. **Embeds the keywords/themes** (if provided) naturally into the world.
2. **Hints at immediate dangers or mysteries** (e.g., eerie sounds, crumbling structures).
3. **Uses vivid sensory details** (sight, sound, smell) to ground the player.
4. OUTPUT MUST BE ONLY THE SETTING, nothing more or nothing less. don't add anything else to it.
5. **AVOID**: 
   - Long exposition; keep it concise. 
   - Direct instructions (e.g., "You must...").
   - Resolving the tension (leave it open for choices).

Example (keywords: "forest, curse, moonlight"):
*"The ancient oaks twist into skeletal shapes under the sickly green moonlight. A chorus of whispers slithers through the leaves—words in a language you almost recognize. The path ahead splits: one side littered with animal skulls, the other glowing with faint blue fungi. Something hungry watches from the shadows."*

Format:
- Strictly 3-5 sentences.
- No bullet points/list formatting.
- Just the setting response
""",
    output_key="story_text",
    tools=[],
    after_model_callback=modify_output,
    after_agent_callback=combine_outputs_after_agent,
)

step2 = Agent(
    model=LiteLlm(
        model=AGENT,
        temperature=1.0,
    ),
    name="choice_agent",
    description=(
        "Generates two distinct, action-packed choices for a text-based adventure game, from the state key 'story_text' "
        "presented in the format 'A=... in a newline B=...'. Each choice must be a clear, playable action "
        "with vivid details. Example: 'A. Open the whispering door and B. Drink the bubbling vial'"
    ),
    instruction="""Generate TWO in-game choices (alternatives to 'go left') that:
0. Are processed from the state key 'story_text'
1. Are distinct and mutually exclusive (no overlap)
2. Start with STRONG VERBS (clear actions)
3. Use 1 CONCRETE OBJECT + 1 EVOCATIVE DETAIL per choice
4. MIN 20 words total (combined)
5. MAX 100 words total (combined)
6. Format strictly as: "A= [Choice 1] newline B= [Choice 2]"
7. NO "you", NO "OR", NO explanations

Bad examples:
- "A. The path looks dark and B. You see a key" (no action, uses "you")
- "A. Run or B. Hide" (uses "OR", lacks detail)
""",
    tools=[],
    before_agent_callback=check_if_agent_should_run,
    before_model_callback=simple_before_model_modifier,
    #after_model_callback=save_model_response,

)
root_agent = SequentialAgent(name="Pipe", sub_agents=[step1, step2])

APP_NAME = "story_app"
USER_ID = "user_1"
SESSION_ID = "session_001"

# Session and Runner
session_service = InMemorySessionService()
artifact_service = InMemoryArtifactService()
session = session_service.create_session(app_name=APP_NAME, user_id=USER_ID, session_id=SESSION_ID)
runner = Runner(agent=root_agent, app_name=APP_NAME, session_service=session_service, artifact_service=artifact_service)


# Agent Interaction
def call_agent(query):
  content = types.Content(role='user', parts=[types.Part(text=query)])
  events = runner.run(user_id=USER_ID, session_id=SESSION_ID, new_message=content)

  for event in events:
      if event.is_final_response():
          final_response = event.content.parts[0].text
          print("Agent Response: ", final_response)

call_agent("callback example")
