from google.adk.agents import Agent
from google.adk.models.lite_llm import LiteLlm
from typing import Optional
from multi_tool_agent.utils import save_model_response

# litellm._turn_on_debug()

AGENT = "ollama_chat/gemma3:1b"


# --- Definizione dell'Agente con Temperatura ---
root_agent = Agent(
    model=LiteLlm(
        model=AGENT,
        temperature=1.0,
        ),
    name="story_agent",
    description=( # Descrizione aggiornata per chiarezza
         "Generates ONE short, action-packed textual adventure game choice, "
 "alternative to 'go left.' Focuses on a clear action with vivid detail. "
 "Example: 'Open the whispering door' or 'Drink the bubbling vial'"
    ),
    instruction="""Generate ONE in-game choice (alternative to 'go left') that:
1. Builds on past choices implied by the input keywords (if any)
2. Starts with a STRONG VERB and is a clear ACTION
3. Uses 1 CONCRETE OBJECT + 1 EVOCATIVE DETAIL
4. MIN 20 words
5. MAX 100 words
6. NO "you", NO "OR", NO explanations 

Bad examples:
- "The path looks dark" (no action)
- "You see a key" (uses "you")
- "Memories linger here" (not playable)
""",
    tools=[],
    after_model_callback=save_model_response,
)

def generate_choice(keywords: str, max_attempts: int = 3) -> Optional[str]:
    """
    Generates a text choice based on keywords using root_agent.
    Retries in case of error or empty answer up to max_attempts times.

    Args:
 keywords: Context or keywords for the choice (e.g., "the sword is broken serves water")
 max_attempts: Maximum number of retries.

    Returns:
 String containing the generated choice, or None if it fails.

    Raises:
 ValueError: If max_attempts is reached without a valid answer.
    """
    print(f"generate_choice with keywords: {keywords}")

    prompt_input = f"Context: {keywords}. Generate a choice."

    for attempt in range(max_attempts):
        print(f"Attempt {attempt + 1}/{max_attempts}...")
        try:
            # Chiama l'agente con l'input specifico
            response = root_agent.run(prompt_input)

            # Estrai il testo dalla risposta
            if (
                response
                and response.content
                and response.content.parts
                and response.content.parts[0].text
            ):
                choice_text = response.content.parts[0].text.strip()
                # Rimuovi eventuali artefatti markdown residui (anche se il prompt lo vieta)
                choice_text = choice_text.replace("```", "").strip()

                if choice_text: # Assicurati che la risposta non sia vuota
                    print(f"Generated choice: {choice_text}")
                    return choice_text
                else:
                    print("Attempt failed: Received empty response.")

            else:
                print("Attempt failed: Invalid response structure.")

        except Exception as e:
            print(f"Attempt {attempt + 1} failed with error: {str(e)}")
            if attempt == max_attempts - 1:
                raise ValueError(
                    f"Impossibile generare una scelta valida dopo {max_attempts} tentativi per keywords: '{keywords}'"
                )
        # Attendi un istante prima di riprovare (opzionale)
        # import time
        # time.sleep(1)

    # Se esce dal ciclo senza successo
    raise ValueError(
        f"Impossibile generare una scelta valida dopo {max_attempts} tentativi per keywords: '{keywords}'"
    )

