import asyncio
import os
import sys
import json
from browser_use import Agent
from langchain_openai import ChatOpenAI

# Note: browser-use requires an LLM to drive the browser.
# We'll use OpenAI if the key is available, or try to use another provider.
# Since the user has DeepSeek, maybe we can use that via OpenAI-compatible API.

async def twitter_post(username, password, email, text):
    deepseek_key = os.getenv("DEEPSEEK_API_KEY")
    if not deepseek_key:
        print(json.dumps({"success": False, "error": "DEEPSEEK_API_KEY not found"}))
        return

    # DeepSeek reasoning model or chat model
    # Apply aggressive prompt caching to reduce token overhead for routine browser tasks
    llm = ChatOpenAI(
        model="deepseek-chat", 
        api_key=deepseek_key, 
        base_url="https://api.deepseek.com",
        default_headers={"cache-control": "ephemeral"}
    )

    task = f"""
    1. Go to https://x.com/i/flow/login
    2. Log in with username '{username}', password '{password}', and if asked for email verification, use '{email}'.
    3. Once logged in, go to https://x.com/compose/post
    4. Type the tweet: '{text}'
    5. Click the 'Post' button.
    6. Confirm the tweet was posted successfully.
    """

    agent = Agent(
        task=task,
        llm=llm,
    )
    
    try:
        result = await agent.run()
        print(json.dumps({"success": True, "message": str(result)}))
    except Exception as e:
        print(json.dumps({"success": False, "error": str(e)}))

if __name__ == "__main__":
    u, p, e, t = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
    asyncio.run(twitter_post(u, p, e, t))
