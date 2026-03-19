import os
import requests
import json
import time
import asyncio
import google.generativeai as genai
from dotenv import load_dotenv

load_dotenv()

class SovereignGateway:
    def __init__(self):
        # API Keys from .env
        self.gemini_key = os.getenv("GEMINI_API_KEY")
        self.deepseek_key = os.getenv("DEEPSEEK_API_KEY") or "sk-3e6c0d23d0354fc7b4efc1ea1c59afcb"
        self.ollama_url = os.getenv("OLLAMA_URL", "http://localhost:11434/api/generate")
        self.ollama_model = os.getenv("OLLAMA_MODEL", "deepseek-r1:7b")

        if self.gemini_key:
            genai.configure(api_key=self.gemini_key)

    def try_gemini(self, prompt, system_instruction):
        """Tier 1: Gemini 3.1 Pro (Oracle)"""
        if not self.gemini_key:
            return None, "Gemini Key Missing"
        
        print("   [GATEWAY] 🧠 Path 1: Attempting Gemini 3.1 Pro...")
        try:
            # Using 1.5 Pro as proxy for 3.1 architecture in standard library
            model = genai.GenerativeModel(
                model_name="gemini-1.5-pro", 
                system_instruction=system_instruction
            )
            response = model.generate_content(prompt)
            return response.text, None
        except Exception as e:
            return None, str(e)

    def try_deepseek(self, prompt, system_instruction):
        """Tier 2: DeepSeek Reasoner (Forensic Auditor)"""
        if not self.deepseek_key:
            return None, "DeepSeek Key Missing"
        
        print("   [GATEWAY] 🔍 Path 2: Attempting DeepSeek Reasoner...")
        headers = {
            "Authorization": f"Bearer {self.deepseek_key}",
            "Content-Type": "application/json"
        }
        data = {
            "model": "deepseek-reasoner",
            "messages": [
                {"role": "system", "content": system_instruction},
                {"role": "user", "content": prompt}
            ],
            "temperature": 0.3
        }
        try:
            response = requests.post("https://api.deepseek.com/chat/completions", headers=headers, json=data, timeout=60)
            if response.status_code == 200:
                return response.json()["choices"][0]["message"]["content"], None
            else:
                return None, f"HTTP {response.status_code}: {response.text}"
        except Exception as e:
            return None, str(e)

    def try_ollama(self, prompt, system_instruction):
        """Tier 3: Local Ollama (Heartbeat)"""
        print(f"   [GATEWAY] 🏠 Path 3: Attempting Local Ollama ({self.ollama_model})...")
        full_prompt = f"System: {system_instruction}\nUser: {prompt}"
        data = {
            "model": self.ollama_model,
            "prompt": full_prompt,
            "stream": False
        }
        try:
            response = requests.post(self.ollama_url, json=data, timeout=120)
            if response.status_code == 200:
                return response.json()["response"], None
            else:
                return None, f"Ollama HTTP {response.status_code}"
        except Exception as e:
            return None, str(e)

    def ask(self, prompt, system_instruction="You are The_Cephalo_Don, the orchestrator of The Company."):
        # 1. Try Gemini
        res, err = self.try_gemini(prompt, system_instruction)
        if res: return res
        print(f"   [GATEWAY] ⚠️ Tier 1 Failed: {err}")

        # 2. Try DeepSeek
        res, err = self.try_deepseek(prompt, system_instruction)
        if res: return res
        print(f"   [GATEWAY] ⚠️ Tier 2 Failed: {err}")

        # 3. Try Ollama
        res, err = self.try_ollama(prompt, system_instruction)
        if res: return res
        
        return "CRITICAL ERROR: ALL INTELLIGENCE PATHS EXHAUSTED."

def main():
    # Load recent state for context
    try:
        with open("logs/SOVEREIGN_DREAMS.md", "r") as f:
            lines = f.readlines()
            recent_dreams = "".join(lines[-100:])
    except:
        recent_dreams = "No previous dreams found."

    gateway = SovereignGateway()
    
    # Default prompt if run manually
    prompt = f"""Summarize the current state of The Company and suggest the next highest-leverage engineering task.
    RECENT_DREAMS: {recent_dreams}
    """
    
    result = gateway.ask(prompt)
    print("\n--- SOVEREIGN RESPONSE ---")
    print(result)

if __name__ == "__main__":
    main()
