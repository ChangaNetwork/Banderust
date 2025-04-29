from typing import  Optional
from google.adk.agents.callback_context import CallbackContext
from google.genai import types
from google.adk.models import LlmResponse, LlmRequest
import json

def merge_text(
    callback_context: CallbackContext, llm_request: LlmRequest
) -> Optional[LlmResponse]:
    """Merges multiple texts if present in content"""
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
    print(f"Merged text: {merged_text}")
    llm_request.contents = [
        types.Content(
            parts=[types.Part(text=merged_text)],
            role="user"
        )
    ]
    
    return None 

def save_json_response(callback_context: CallbackContext) -> Optional[types.Content]:
    current_state = callback_context.state
    #print(f"[save json response]: \nSTORY: {current_state.get("story_text")}\nCHOICES: {current_state.get("choices")}")


    #print("A Part:", current_state.get("choice_a"))
    #print("B Part:", current_state.get("choice_b"))
    #print("Story text:", current_state.get("story_text").split("*\n\n")[1])
    data = {}
    data['text'] = current_state.get("story_text")
    data['a'] = current_state.get("choice_a")
    data['b'] = current_state.get("choice_b")
    #json_data = json.dumps(data)
    filename = f"story_{callback_context.invocation_id}.json"  # Cambiato estensione

    with open(f"output/{filename}", "w", encoding="utf-8") as file:
        json.dump(data, file)
    return None
def save_model_response(
    callback_context: CallbackContext, llm_response: LlmResponse
) -> Optional[LlmResponse]:
    print(f"save_model_response received: {llm_response.content.parts[0].text}")
    try:
        if (
            llm_response
            and llm_response.content
            and llm_response.content.parts
            and llm_response.content.parts[0].text
        ):
            response_text = (
                llm_response.content.parts[0]
                .text.replace(
                    "```json", ""
                )  # Anche se chiedi testo, pulire non fa male
                .replace("```", "")
                .replace(
                    "Story_agent: Initiating sequence.", ""
                )  # Aggiunto per rimuovere l'inizio della sequenza
                .replace("Story_Agent: Processing… Initiating adventure.", "")
                .replace(
                    "Story_Agent: ", ""
                )  # Aggiunto per rimuovere il prefisso Story_Agent
                .replace(
                    "Story_agent:", ""
                )  # Aggiunto per rimuovere il prefisso Story_agent
                .strip()
            )
            print(f"Testo della risposta pulito: {response_text}")

            artifact_content = types.Part(
                inline_data=types.Blob(
                    data=response_text.encode("utf-8"),
                    mime_type="text/plain",  # Cambiato mime_type
                )
            )
            print(f"callback context invocation id: {callback_context.invocation_id}")
            filename = f"story_response_{callback_context.invocation_id}.txt"  # Cambiato estensione
            print(f"filename: {filename}")
            try:
                version = callback_context.save_artifact(
                    filename=filename, artifact=artifact_content
                )
                # Il resto del salvataggio/caricamento può rimanere simile,
                # adattando la lettura se necessario (decode utf-8)
                print(
                    f"Risposta del modello salvata come artefatto di testo '{filename}', versione {version}."
                )
                artifact = callback_context.load_artifact(filename, version)
                if artifact and artifact.inline_data:
                    with open(f"output/{filename}", "w", encoding="utf-8") as f:
                        f.write(artifact.inline_data.data.decode("utf-8"))
                    print(
                        f"Artefatto di testo '{filename}' caricato e salvato in output/{filename}."
                    )
            except ValueError as e:
                print(
                    f"Errore nel salvataggio dell'artefatto di testo: {e}. È configurato ArtifactService?"
                )
            except Exception as e:
                print(
                    f"Errore imprevisto durante il salvataggio dell'artefatto di testo: {e}"
                )
        else:
            print("Nessuna risposta testuale valida da salvare.")
    except Exception as e:
        print(f"Errore durante la gestione della risposta per il salvataggio: {e}")
    return llm_response


