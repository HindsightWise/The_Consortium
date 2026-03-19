import asyncio
import json
import sys
import subprocess

try:
    from scrapling.engines.playwright import PlaywrightEngine
except ImportError:
    print("Installing strict dependency...")
    subprocess.check_call([sys.executable, "-m", "pip", "install", "scrapling", "playwright"])
    from scrapling.engines.playwright import PlaywrightEngine

async def verify_reality():
    print("Igniting Scrapling Engine for Timeline Verification...")
    engine = PlaywrightEngine(headless=True)
    
    # Inject Commander Cookies
    print("Loading Commander state...")
    try:
        # Utilizing standard Playwright directly underneath if needed to add advanced cookies
        page = await engine.page
        await page.context.add_cookies([
            {
                'name': 'auth_token',
                'value': '1808671eff850fb41c4f0558d9804e67c62d25ad',
                'domain': '.x.com',
                'path': '/',
                'secure': True,
                'sameSite': 'None'
            },
            {
                'name': 'ct0',
                'value': 'd413da1add9ed9cc8881acfc45179113f5625ff59c0ec22c96bce761124803227bd19c7770ff6ab6ca0189e8f73f66c3aa9d4f4c06462afd704406a5a9d8d6a86343bd4f12c3a8f0c42f246fd885e39a',
                'domain': '.x.com',
                'path': '/',
                'secure': True,
                'sameSite': 'None'
            }
        ])
    except Exception as e:
        print(f"Cookie injection error warning: {e}. Attempting physical navigation anyway.")

    print("Dropping into physical X.com timeline (Akkokanika_Company)...")
    await engine.get("https://x.com/Akkokanika_Company")
    
    print("Awaiting DOM stabilization...")
    await asyncio.sleep(8)
    
    try:
        # Take a physical footprint
        await engine.page.screenshot(path="verification_snapshot.png", full_page=True)
        print("Physical visual proof saved to verification_snapshot.png")
    except Exception as e:
         pass

    # Extract Timeline text via DOM Test IDs
    try:
        tweet_elements = await engine.page.query_selector_all('[data-testid="tweetText"]')
        print("\n--- PHYSICAL TIMELINE SCRAPE RESULT ---")
        if not tweet_elements:
            print("CRITICAL FINDING: No physical tweets found on the timeline. Profile is either empty, restricted, or the Tangible Drone logic is mocking output.")
        else:
            for idx, el in enumerate(tweet_elements[:3]):
                text = await el.inner_text()
                print(f"\n[FOUND TWEET POST {idx+1}]:\n{text}")
    except Exception as e:
        print(f"Extraction failed: {e}")
            
    await engine.close()

if __name__ == "__main__":
    asyncio.run(verify_reality())
