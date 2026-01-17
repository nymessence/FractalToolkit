import os
import json
import requests
from datetime import datetime
import subprocess

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
    print("Engaging in deeper fractal exploration with Nya Elyria...")
    conv_mgr = FractalConversationManager()
    
    # Continue the conversation with more specific questions about fractal exploration
    continuation_questions = [
        "Based on our previous discussion, I'm excited to explore these concepts with you using our fractal toolkit. We've recently implemented support for complex exponents like z^(2.7+0.3i) + c, which were previously causing issues. Would you like to see what these kinds of fractals look like?",
        "We've also added support for higher hyperoperations like pentation and hexation. These are extremely computationally intensive but can create incredibly complex structures. Would you be interested in experimenting with these?",
        "Our toolkit now supports a wider range of mathematical functions like sqrt, cbrt, asin, acos, atan, sinh, cosh, tanh. How would you like to incorporate these into fractal formulas?",
        "I'm also curious about your thoughts on color palettes and rendering parameters. Different color schemes can dramatically change the visual impact of the same mathematical structure."
    ]
    
    for question in continuation_questions:
        response = conv_mgr.send_message(question)
        print(f"Nya Elyria: {response}\n")
        print("---\n")

if __name__ == "__main__":
    main()