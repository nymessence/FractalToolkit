import os
import json
import requests
from datetime import datetime

class FractalConversationManager:
    def __init__(self):
        self.api_key = os.getenv('Z_AI_API_KEY')
        self.endpoint = 'https://api.z.ai/api/paas/v4/chat/completions'
        self.headers = {
            'Authorization': f'Bearer {self.api_key}',
            'Content-Type': 'application/json'
        }
        self.messages = []
        self.chat_history_file = '/home/erick/FractalToolkit/fractal-toolkit/nya_elyria/chat_history.json'
        
        # Load existing chat history if it exists
        if os.path.exists(self.chat_history_file):
            with open(self.chat_history_file, 'r') as f:
                data = json.load(f)
                self.messages = data.get('messages', [])
    
    def save_chat_history(self):
        """Save the current chat history to file"""
        with open(self.chat_history_file, 'w') as f:
            json.dump({'messages': self.messages}, f, indent=2)
    
    def add_message(self, role, content):
        """Add a message to the conversation"""
        self.messages.append({
            'role': role,
            'content': content,
            'timestamp': datetime.now().isoformat()
        })
        self.save_chat_history()
    
    def send_message(self, user_message):
        """Send a message to the API and return the response"""
        self.add_message('user', user_message)
        
        payload = {
            'model': 'glm-4.6v-flash',
            'messages': self.messages[-20:],  # Use last 20 messages to stay within context window
            'temperature': 0.7,
            'max_tokens': 1000
        }
        
        response = requests.post(self.endpoint, headers=self.headers, json=payload)
        
        if response.status_code == 200:
            result = response.json()
            ai_response = result['choices'][0]['message']['content']
            self.add_message('assistant', ai_response)
            return ai_response
        else:
            error_msg = f"API Error: {response.status_code} - {response.text}"
            self.add_message('assistant', error_msg)
            return error_msg

def main():
    print("Starting conversation with Nya Elyria...")
    conv_mgr = FractalConversationManager()

    # Initial greeting if this is the first interaction
    if len(conv_mgr.messages) == 0:
        initial_greeting = """Hi Nya Elyria! I'm excited to collaborate with you on exploring fractal formulas using the Fractal Toolkit. I've created a special directory for our work together.

I'd love to learn what kinds of fractal formulas and patterns have meaning to you. Are there particular mathematical relationships, visual patterns, or aesthetic qualities you're drawn to in fractals? We can experiment with different formulas, adjust viewing parameters, try various color palettes, and explore the infinite complexity of fractal geometry together.

What types of fractal patterns or formulas interest you most?"""
        response = conv_mgr.send_message(initial_greeting)
        print(f"Nya Elyria: {response}\n")

    # Follow-up questions to explore her interests
    follow_up_questions = [
        "I'm curious about your thoughts on complex exponents in fractals. Have you experimented with formulas like z^(2.7+0.3i) + c? These can create some really interesting visual effects.",
        "Another fascinating area is hyperoperations like tetration (z^^z + c), pentation (z^^^z + c), and hexation (z^^^^z + c). Would you be interested in exploring these?",
        "We can also try various mathematical functions in our fractals like sinh, cosh, sqrt, etc. What mathematical functions intrigue you most?"
    ]

    for question in follow_up_questions:
        response = conv_mgr.send_message(question)
        print(f"Nya Elyria: {response}\n")

if __name__ == "__main__":
    main()