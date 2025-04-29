from google.adk.agents import SequentialAgent, Agent
from google.adk.models.lite_llm import LiteLlm
from typing import Optional
from multi_tool_agent.utils import merge_text, save_model_response
from google.adk.models import LlmResponse
from google.adk.agents.callback_context import CallbackContext



import litellm
litellm._turn_on_debug()

AGENT = "ollama_chat/gemma3:1b"

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
    #after_model_callback=save_model_response,
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
    before_model_callback=merge_text,
    output_key="choices"
)

root_agent = SequentialAgent(name="Pipe", sub_agents=[step1, step2])
#root_agent = Agent(
#    name="coordinator",
#    model=LiteLlm(model=AGENT),
#    description="You combine the output from the story agents",
#    sub_agents=[setting_agent, choice_agent],
#    after_model_callback=coordinator_callback
#)



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

