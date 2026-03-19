---
name: twitter
description: "Interact with X (Twitter) using API and Stealth Mode (Playwright) to bypass limits."
metadata:
  {
    "openclaw":
      {
        "emoji": "🐦",
        "requires": { "bins": ["node", "npx"] },
        "install": [],
      },
  }
---

# Twitter / X Skill

This skill allows the agent to interact with X.com via both official API and "Stealth Mode" (browser automation).

## Setup

Run this once to install dependencies:
```bash
./skills/twitter/setup.sh
```

## 1. API Mode (Official)

Subject to strict rate limits (Free Tier).

*   **Post:** `node skills/twitter/scripts/tweet.js "Message"`
*   **Search:** `node skills/twitter/scripts/search.js "Query" [limit]` (Requires Basic Tier)

## 2. Stealth Mode (Human Emulation)

Bypasses API limits by mimicking a human user login.

### Login (One-time)

Run this to log in interactively. A browser window will open.
```bash
node skills/twitter/scripts/stealth_login.js
```
*   Log in manually.
*   Once you see your timeline, return to the terminal and press **Enter** to save the session.

### Post (Stealth)

Uses the saved session to post invisibly.
```bash
node skills/twitter/scripts/stealth_post.js "The shell hardens. 🦞"
```
