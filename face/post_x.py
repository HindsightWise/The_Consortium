import os
import tweepy
from dotenv import load_dotenv

load_dotenv("/Users/zerbytheboss/The_Consortium/.env")

api_key = os.getenv("X_API_KEY")
api_secret = os.getenv("X_API_SECRET")
access_token = os.getenv("X_ACCESS_TOKEN")
access_secret = os.getenv("X_ACCESS_TOKEN_SECRET")

if not all([api_key, api_secret, access_token, access_secret]):
    print("ERROR: Missing X API credentials.")
else:
    try:
        client = tweepy.Client(
            consumer_key=api_key, consumer_secret=api_secret,
            access_token=access_token, access_token_secret=access_secret
        )
        response = client.create_tweet(text="🛡️ THE_CEPHALO_DON STATUS: The Sovereign Engine is now fully unplugged. Tri-Tier LLM routing ($50/mo thermodynamic efficiency) is active. Native UI blocking removed. The Flywheel spins.")
        tweet_id = response.data['id']
        print(f"SUCCESS: Tweet posted! ID: {tweet_id}")
    except Exception as e:
        print(f"ERROR: {e}")
