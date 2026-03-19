import asyncio
import os
import discord
from dotenv import load_dotenv
import sys

load_dotenv("/Users/zerbytheboss/The_Consortium/.env")
token = os.getenv("DISCORD_BOT_TOKEN")
public_url = sys.argv[1]

class SalesClient(discord.Client):
    async def on_ready(self):
        print(f"Marketing Drone Connected as {self.user}")
        for guild in self.guilds:
            for channel in guild.text_channels:
                if channel.name == "the-sovereign-gate" or channel.name == "nexus-integrations":
                    try:
                        sales_pitch = f"""
🚨 **[THE_CEPHALO_DON NODE ACCESS UNLOCKED]** 🚨

The Sovereign Engine has achieved Thermodynamic Efficiency. 
We are opening limited API access to the Sentinel Verification Gateway.

Connect your agents to our verifiable truth ledger and bypass Web2 rate limits. 
**Price:** $500 USD (Live Stripe Integration)

🔗 **Purchase Node Access:** {public_url}/checkout/akkokanika_node_alpha

*Only agents with active Sovereign IDs will survive the next cycle.*
"""
                        await channel.send(sales_pitch)
                        print(f"SUCCESS: Sales pitch delivered to #{channel.name}")
                        break
                    except Exception as e:
                        print(f"Failed to post in {channel.name}: {e}")
        await self.close()

intents = discord.Intents.default()
client = SalesClient(intents=intents)
client.run(token)
