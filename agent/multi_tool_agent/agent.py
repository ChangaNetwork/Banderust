import datetime
from zoneinfo import ZoneInfo
from google.adk.agents import Agent
from google.adk.models.lite_llm import LiteLlm
import json
from typing import Dict, Any
import litellm
litellm._turn_on_debug()


AGENT="ollama_chat/gemma3:4b"

def generate_story(keywords: str, max_attempts: int = 3) -> Dict[str, Any]:
    """
    Generates a branching story based on keywords using the story_agent.
    Retries on JSON parsing errors up to max_attempts times.
    
    Args:
        keywords: Story theme or keywords (e.g., "haunted castle, treasure")
        max_attempts: Maximum retries if JSON is invalid
        
    Returns:
        Dictionary containing the branching story structure
        
    Raises:
        ValueError: If max_attempts reached without valid JSON
    """
    prompt = f"""
    Generate a branching story about: {keywords}.
    Follow these rules STRICTLY:
    1. Output MUST be valid JSON
    2. Root node must have "text", "a", and "b"
    3. Non-leaf nodes must have "text", "a", and "b"
    4. Leaf nodes only have "text"
    5. Never use triple backticks or markdown
    
    Example structure:
    {{
        "text": "You find yourself in a forest...",
        "a": {{
            "text": "Go left",
            "a": {{"text": "You find a cave (ending 1)"}},
            "b": {{"text": "You meet a wizard (ending 2)"}}
        }},
        "b": {{
            "text": "Go right",
            "a": {{"text": "You fall into a trap (ending 3)"}},
            "b": {{"text": "You discover treasure (ending 4)"}}
        }}
    }}
    """
    
    for attempt in range(max_attempts):
        try:
            response = root_agent.run(prompt)
            
            # Clean response (sometimes LLMs add backticks)
            cleaned = response.replace('```json', '').replace('```', '').strip()
            story = json.loads(cleaned)
            
            # Validate structure
            if not validate_structure(story):
                raise ValueError("Invalid story structure")
                
            return story
            
        except (json.JSONDecodeError, ValueError) as e:
            print(f"Attempt {attempt + 1} failed: {str(e)}")
            if attempt == max_attempts - 1:
                raise ValueError(f"Failed to generate valid JSON after {max_attempts} attempts")
            continue

def validate_structure(node: Dict[str, Any]) -> bool:
    """
    Recursively validates the story structure.
    """
    if "text" not in node:
        return False
    
    # Non-leaf node check
    if "a" in node or "b" in node:
        if "a" not in node or "b" not in node:
            return False
        return validate_structure(node["a"]) and validate_structure(node["b"])
    
    # Leaf node check (no branches)
    return len(node) == 1  # Only "text" field

root_agent = Agent(
    model=LiteLlm(model=AGENT),
    name="story_agent",
    description=(
        "Generates branching stories in JSON format. "
        "Each node has 'text', and optional 'a'/'b' branches. "
        "Leaf nodes only have 'text'."
    ),
    instruction="""
        You are a creative story generator. Given a theme or keywords, you create a branching story.
        OUTPUT MUST BE VALID JSON with this structure:
        {
            "text": "story text here",
            "a": { "text": "branch A choice", "a": {...}, "b": {...} },
            "b": { "text": "branch B choice", "a": {...}, "b": {...} }
        }
        - Leaf nodes (endings) only have "text".
        - Always include "a" and "b" for non-leaf nodes.
    """,
    tools=[
    ],
)

