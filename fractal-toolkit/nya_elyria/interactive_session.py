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
    
    def generate_fractal(self, formula, bounds="-2,2,-2,2", dimensions="16,16", max_iterations=32, output_filename=None):
        """Generate a fractal using the fractal toolkit"""
        if output_filename is None:
            timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
            output_filename = f"fractal_{timestamp}.png"
        
        cmd = [
            "cargo", "run", "--release", "--bin", "ftk-mandel",
            "--",
            f"--bounds={bounds}",
            f"--dimensions={dimensions}",
            f"--max-iterations={max_iterations}",
            f"--formula='{formula}'",
            f"--output='/home/erick/FractalToolkit/fractal-toolkit/nya_elyria/{output_filename}'",
            "--spawn=0,0",
            "--color-pallette='[(#000000,0.0),(#00FF00,0.5),(#FFFFFF,1.0)]'",
            "--bailout=16"
        ]
        
        try:
            result = subprocess.run(
                " ".join(cmd),
                shell=True,
                cwd="/home/erick/FractalToolkit/fractal-toolkit",
                capture_output=True,
                text=True,
                timeout=60
            )
            
            if result.returncode == 0:
                return f"Fractal generated successfully: {output_filename}", output_filename
            else:
                return f"Error generating fractal: {result.stderr}", None
        except subprocess.TimeoutExpired:
            return "Timeout generating fractal", None
        except Exception as e:
            return f"Exception generating fractal: {str(e)}", None

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

    # Interactive loop
    print("Starting interactive session. Type 'quit' to exit.")
    while True:
        try:
            user_input = input("You: ")
            if user_input.lower() == 'quit':
                print("Ending conversation...")
                break

            # Send message to Nya Elyria
            response = conv_mgr.send_message(user_input)
            print(f"\nNya Elyria: {response}\n")

            # Check if Nya Elyria wants to generate a fractal
            if "generate" in user_input.lower() or "fractal" in user_input.lower() or \
               "create" in user_input.lower() or "make" in user_input.lower():
                print("Would you like me to generate a fractal based on our discussion? (yes/no)")
                try:
                    generate_response = input("> ")
                    if generate_response.lower() in ['yes', 'y']:
                        print("Enter a formula to generate (e.g., 'z^2 + c', 'z^(2.7+0.3i) + c', etc.):")
                        formula = input("> ")
                        if formula.strip():
                            print("Generating fractal...")
                            result, filename = conv_mgr.generate_fractal(formula=formula)
                            print(result)
                            if filename:
                                print(f"Fractal saved as: {filename}")
                                # Ask Nya Elyria about the result
                                result_msg = f"We just generated a fractal with the formula '{formula}'. The result is saved as {filename}. Would you like to see it or try a different formula?"
                                response = conv_mgr.send_message(result_msg)
                                print(f"\nNya Elyria: {response}\n")
                except EOFError:
                    print("\nInput interrupted. Continuing...")
                    continue
        except EOFError:
            print("\nInput interrupted. Ending conversation...")
            break

if __name__ == "__main__":
    main()