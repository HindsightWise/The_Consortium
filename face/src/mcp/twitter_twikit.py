import asyncio
import json
import sys
from twikit import Client

async def main():
    if len(sys.argv) < 5:
        print(json.dumps({"success": False, "error": "Usage: python script.py username password email text"}))
        return

    username = sys.argv[1]
    password = sys.argv[2]
    email = sys.argv[3]
    text = sys.argv[4]

    client = Client('en-US')
    
    try:
        # Attempt to login
        print(f"Logging in as {username}...")
        await client.login(
            auth_info_1=username,
            auth_info_2=email,
            password=password
        )
        
        # Save cookies for persistent session
        client.save_cookies('logs/twitter_cookies.json')
        print("Login successful. Cookies saved.")

        # Create tweet
        print(f"Posting tweet: {text}")
        await client.create_tweet(text)
        
        print(json.dumps({"success": True, "message": "Tweet posted via twikit."}))
        
    except Exception as e:
        print(json.dumps({"success": False, "error": str(e)}))

if __name__ == "__main__":
    asyncio.run(main())
