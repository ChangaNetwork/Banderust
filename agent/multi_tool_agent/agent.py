import datetime
from zoneinfo import ZoneInfo
from google.adk.agents import Agent
from google.adk.models.lite_llm import LiteLlm
import json
from typing import Dict, Any, Optional
import litellm
from google.adk.agents.callback_context import CallbackContext
from google.genai import types
from google.adk.models import LlmResponse

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

def save_model_response(callback_context: CallbackContext, llm_response: LlmResponse) -> Optional[LlmResponse]:
    """Saves the model response as an artifact (synchronous version)."""
    print(f"save_model_response received: {llm_response.content.parts[0]}")
    print(f"save_model_response received: {llm_response.content.parts[0].text}")
    

    try:
        if llm_response and llm_response.content and llm_response.content.parts and llm_response.content.parts[0].text:
            response_text = llm_response.content.parts[0].text.replace('```json', '').replace('```', '').strip()
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
    tools=[
    ],
    after_model_callback=save_model_response,
)