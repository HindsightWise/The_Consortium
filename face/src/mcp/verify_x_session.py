import asyncio
from playwright.async_api import async_playwright
import json

async def check_session():
    async with async_playwright() as p:
        try:
            # Connect to the existing Chrome instance
            browser = await p.chromium.connect_over_cdp('http://localhost:9222')
            context = browser.contexts[0]
            page = await context.new_page()
            
            # Go to x.com home
            await page.goto('https://x.com/home', wait_until="networkidle")
            
            # Check if we are logged in by looking for the compose button or profile
            title = await page.title()
            
            # Simple check for 'Home' or 'X' in title which usually indicates logged in feed
            is_logged_in = "Home" in title or "X" in title
            
            # Get cookies for future use or to confirm session
            cookies = await context.cookies()
            
            print(json.dumps({
                "success": True,
                "title": title,
                "is_logged_in": is_logged_in,
                "cookie_count": len(cookies)
            }))
            
            await page.close()
        except Exception as e:
            print(json.dumps({
                "success": False,
                "error": str(e)
            }))

if __name__ == "__main__":
    asyncio.run(check_session())
