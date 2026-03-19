import asyncio
import json
import sys
import os
from playwright.async_api import async_playwright

async def twitter_post(username, password, email, text):
    async with async_playwright() as p:
        # Use headed mode to look 100% human
        browser = await p.firefox.launch(headless=False)
        context = await browser.new_context(
            user_agent="Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
        )
        page = await context.new_page()
        
        try:
            print(f"Navigating to X.com (Headed Mode)...")
            await page.goto("https://x.com/i/flow/login")
            await page.wait_for_timeout(10000)
            
            # 1. Enter Username
            print("Typing username...")
            await page.locator("input").first.click()
            await page.keyboard.type(username, delay=100)
            await page.keyboard.press("Enter")
            await page.wait_for_timeout(5000)
            
            # 2. Check for verification
            if "verify" in page.url or await page.get_by_text("Verify your identity").is_visible():
                print("Verification required...")
                await page.locator("input").first.click()
                await page.keyboard.type(email, delay=100)
                await page.keyboard.press("Enter")
                await page.wait_for_timeout(5000)

            # 3. Enter Password
            print("Typing password...")
            await page.locator("input[name='password']").click()
            await page.keyboard.type(password, delay=100)
            await page.keyboard.press("Enter")
            await page.wait_for_timeout(10000)
            
            # 4. Post Tweet
            print(f"Posting tweet...")
            await page.goto("https://x.com/compose/post")
            await page.wait_for_timeout(5000)
            
            await page.locator("div[data-testid='tweetTextarea_0']").fill(text)
            await page.wait_for_timeout(2000)
            await page.click("button[data-testid='tweetButton']")
            await page.wait_for_timeout(5000)
            
            print(json.dumps({"success": True, "message": "Tweet posted successfully in Headed mode."}))
        except Exception as e:
            print(json.dumps({"success": False, "error": str(e)}))
            await page.screenshot(path="logs/twitter_headed_error.png")
        finally:
            await browser.close()

if __name__ == "__main__":
    u, p, e, t = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
    asyncio.run(twitter_post(u, p, e, t))
