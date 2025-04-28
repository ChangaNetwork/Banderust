import datetime
from zoneinfo import ZoneInfo
from google.adk.agents import Agent
from google.adk.models.lite_llm import LiteLlm
import json
from typing import Dict, Any, Optional
from google.adk.agents.callback_context import CallbackContext
from google.adk.sessions import InMemorySessionService
from google.adk.runners import Runner
from google.genai import types
from google.adk.models import LlmResponse, LlmRequest
from google.adk.artifacts import InMemoryArtifactService
import litellm
litellm._turn_on_debug()
AGENT="ollama_chat/llama3.1"

def generate_story(keywords: str, max_attempts: int = 3) -> Dict[str, Any]:
    print(f"generate story with {keywords}")
    """
    Genera una storia a bivi basata su parole chiave utilizzando l'agente story_agent,
    limitando la profondità a max_levels.
    Riprova in caso di errori di parsing JSON fino a max_attempts volte.

    Args:
        keywords: Tema o parole chiave della storia (e.g., "castello infestato, tesoro")
        max_attempts: Massimo numero di tentativi se il JSON non è valido
        max_levels: Massimo numero di livelli di opzioni da generare

    Returns:
        Dizionario contenente la struttura della storia a bivi

    Raises:
        ValueError: Se max_attempts viene raggiunto senza JSON valido
    """
    prompt_template = """
    Generate a branching story about: {keywords}.
    Follow these rules STRICTLY:
    1. The output MUST be valid JSON
    2. The root node must have "text", "a", and "b"
    3. Non-leaf nodes must have "text", "a", and "b"
    4. Leaf nodes only have "text"
    5. Never use triple backticks or markdown
    6. "a" and "b" are nodes -> a node is structured like: {{"text": "...", "a":{{node}}, "b":{{node}} }}
    7. Make 3 story levels.
    
    Use this structure and replace the ... with the actualy story text
    {{
        "text": "...",
        "a": {{
            "text": "...",
            "a": {{"text":"...", "a":{{"text:"..."}}, "b":{{"text":"..."}} }},
            "b": {{"text":"...", "a":{{"text:"..."}}, "b":{{"text":"..."}} }}
        }},
        "b": {{
            "text": "...",
            "a": {{"text":"...", "a":{{"text:"..."}}, "b":{{"text":"..."}} }},
            "b": {{"text":"...", "a":{{"text:"..."}}, "b":{{"text":"..."}} }}
        }}
    }}
    """


    prompt = prompt_template.format(
        keywords=keywords,
        level_constraint_message=level_constraint_message
    )

    for attempt in range(max_attempts):
        try:
            response = root_agent.run(prompt)
            # Pulisci la risposta (a volte gli LLM aggiungono backtick)
            cleaned = response.replace('```json', '').replace('```', '').strip()
            story = json.loads(cleaned)
            # Convalida la struttura considerando il numero massimo di livelli
            if validate_structure(story, max_levels=max_levels):
                return story
            else:
                raise ValueError("Struttura della storia non valida")
        except (json.JSONDecodeError, ValueError) as e:
            print(f"Tentativo {attempt + 1} fallito: {str(e)}")
            if attempt == max_attempts - 1:
                raise ValueError(f"Impossibile generare JSON valido dopo {max_attempts} tentativi")
            continue

def validate_structure(node: Dict[str, Any], level: int = 1, max_levels: int = 3) -> bool:
    print("Validate structure")
    """
    Valida ricorsivamente la struttura della storia, considerando il numero massimo di livelli.
    """
    if "text" not in node:
        return False

    is_leaf = "a" not in node and "b" not in node

    if level < max_levels:
        # Nodo non foglia atteso
        if not is_leaf:
            if "a" not in node or "b" not in node:
                return False
            return validate_structure(node["a"], level + 1, max_levels) and validate_structure(node["b"], level + 1, max_levels)
        else:
            # Nodo foglia inatteso prima del livello massimo
            return True # Permettiamo nodi foglia prima del livello massimo
    else:
        # Livello massimo raggiunto, ci si aspetta un nodo foglia
        return is_leaf and len(node) == 1 # Solo il campo "text"


# --- Define the Callback Function ---
def simple_before_model_modifier(
    callback_context: CallbackContext, llm_request: LlmRequest
) -> Optional[LlmResponse]:
    """Inspects/modifies the LLM request or skips the call."""
    agent_name = callback_context.agent_name
    print(f"[Callback] Before model call for agent: {agent_name}")

    # Inspect the last user message in the request contents
    last_user_message = ""
    if llm_request.contents and llm_request.contents[-1].role == 'user':
         if llm_request.contents[-1].parts:
            last_user_message = llm_request.contents[-1].parts[0].text
    print(f"[Callback] Inspecting last user message: '{last_user_message}'")

    # --- Modification Example ---
    # Add a prefix to the system instruction
    original_instruction = llm_request.config.system_instruction or types.Content(role="system", parts=[])
    prefix = "[Modified by Callback] "
    # Ensure system_instruction is Content and parts list exists
    if not isinstance(original_instruction, types.Content):
         # Handle case where it might be a string (though config expects Content)
         original_instruction = types.Content(role="system", parts=[types.Part(text=str(original_instruction))])
    if not original_instruction.parts:
        original_instruction.parts.append(types.Part(text="")) # Add an empty part if none exist

    # Modify the text of the first part
    modified_text = prefix + (original_instruction.parts[0].text or "")
    original_instruction.parts[0].text = modified_text
    llm_request.config.system_instruction = original_instruction
    print(f"[Callback] Modified system instruction to: '{modified_text}'")

    # --- Skip Example ---
    # Check if the last user message contains "BLOCK"
    if "BLOCK" in last_user_message.upper():
        print("[Callback] 'BLOCK' keyword found. Skipping LLM call.")
        # Return an LlmResponse to skip the actual LLM call
        return LlmResponse(
            content=types.Content(
                role="model",
                parts=[types.Part(text="LLM call was blocked by before_model_callback.")],
            )
        )
    else:
        print("[Callback] Proceeding with LLM call.")
        # Return None to allow the (modified) request to go to the LLM
        return None


def save_model_response(callback_context: CallbackContext, llm_response: LlmResponse) -> Optional[LlmResponse]:
    """Saves the model response as an artifact (synchronous version)."""
    print(f"save_model_response received: {llm_response.content.parts[0]}")
    print(f"save_model_response received: {llm_response.content.parts[0].text}")
    

    try:
        if llm_response and llm_response.content and llm_response.content.parts and llm_response.content.parts[0].text:
            response_text = llm_response.content.parts[0].text
            print(f"testo della risposta: {response_text}")

            try:
                # Tenta di caricare la risposta come JSON (se già in formato JSON)
                response_json = json.loads(response_text)
            except json.JSONDecodeError:
                # Se non è un JSON valido, crea un semplice oggetto JSON con il testo
                response_json = {"response": response_text}

            # Converti l'oggetto JSON in una stringa JSON
            json_string = json.dumps(response_json, indent=2)

            # Crea l'artefatto con il corretto mime_type per JSON
            artifact_content = types.Part(
                inline_data=types.Blob(
                    data=json_string.encode('utf-8'),
                    mime_type="application/json"
                )
            )
            filename = f"story_response_{callback_context.invocation_id}.json"
            try:
                version = callback_context.save_artifact(filename=filename, artifact=artifact_content)
                artifact = callback_context.load_artifact(filename, version)
                if artifact and artifact.inline_data:
                    with open(f"output/{filename}", "w") as f:
                        f.write(artifact.inline_data.data.decode('utf-8'))
                print(f"Risposta del modello salvata come artefatto JSON '{filename}', versione {version}.")
            except ValueError as e:
                print(f"Errore nel salvataggio dell'artefatto JSON: {e}. È configurato ArtifactService?")
            except Exception as e:
                print(f"Errore imprevisto durante il salvataggio dell'artefatto JSON: {e}")
        else:
            print("Nessuna risposta testuale valida da salvare come JSON.")

    except Exception as e:
        print(f"Errore durante la gestione della risposta per il salvataggio come JSON: {e}")

    return llm_response

root_agent = Agent(
    model=LiteLlm(model=AGENT),
    name="story_agent",
    description=(
        "Generate a branchign story in JSON. "
        "Every non-leaf nodes have 'text', 'a' and 'b'. "
        "'a' and 'b' are nodes"
        "Only leaf-nodes have just the text attribtute"
    ),
    instruction="""
    You're a creative story generator. Given a theme or some keywords you should generate a branching story.
    Output must be a valid json where you should replace the dots in the following structure:
    Go only 4 levels deep and make every text options more descriptive and richer to make the story more entertaining and immersive.
    The response should be only the json.
    {
      "text": "...",
      "a": { "text": "...", "a": {"text": "..."}, "b": {"text": "..."} },
      "b": { "text": "...", "a": {"text": "..."}, "b": {"text":"..."} }
    }
    - Leaf nodes have only "text".
    - For non-leaf nodes you must always include "text", "a" and "b".
    """,
    tools=[],
    #before_model_callback=simple_before_model_modifier,
    after_model_callback=save_model_response,
)

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
