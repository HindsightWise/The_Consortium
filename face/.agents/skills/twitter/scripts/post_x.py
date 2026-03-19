import undetected_chromedriver as uc
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
import time
import sys
import json
import os

def main():
    if len(sys.argv) < 2:
        print('Usage: python3 post_x_uc.py "Tweet Text"')
        sys.exit(1)
        
    text = sys.argv[1]

    # Load secrets
    secrets_path = os.path.abspath(os.path.join(os.path.dirname(__file__), '../../../../secrets.json'))
    try:
        with open(secrets_path, 'r') as f:
            secrets = json.load(f)
            username = secrets['twitter']['username']
            password = secrets['twitter']['password']
            email = secrets['twitter']['email']
    except Exception as e:
        print(f"FAILED TO LOAD SECRETS: {e}")
        sys.exit(1)

    print('🦞 [UC-Stealth] Launching undetected Chrome browser...')
    
    # Needs to be headless to run in background.
    # UC handles headless masking, but we use the specific UC `headless_new=True` flag or standard `headless=True`.
    # `headless=True` in UC removes most fingerprinting vulnerabilities.
    options = uc.ChromeOptions()
    options.add_argument('--no-sandbox')
    options.headless = True
    
    driver = uc.Chrome(options=options)
    
    try:
        print('   [UC-Stealth] Navigating to X.com login...')
        driver.get("https://x.com/i/flow/login")
        wait = WebDriverWait(driver, 15)
        
        print('   [UC-Stealth] Waiting for username field...')
        user_input = wait.until(EC.visibility_of_element_located((By.CSS_SELECTOR, 'input[autocomplete="username"]')))
        
        print('   [UC-Stealth] Entering username...')
        user_input.send_keys(username)
        
        # Click Next
        next_button = driver.find_element(By.XPATH, '//span[text()="Next"]/ancestor::button | //button[contains(., "Next")]')
        next_button.click()
        
        # Unusual activity check
        try:
            email_wait = WebDriverWait(driver, 5)
            email_input = email_wait.until(EC.visibility_of_element_located((By.CSS_SELECTOR, 'input[name="text"]')))
            print('   [UC-Stealth] Unusual activity detected, entering email...')
            email_input.send_keys(email)
            next_button = driver.find_element(By.XPATH, '//span[text()="Next"]/ancestor::button | //button[contains(., "Next")]')
            next_button.click()
        except:
            pass # No unusual activity
            
        print('   [UC-Stealth] Waiting for password field...')
        password_input = wait.until(EC.visibility_of_element_located((By.CSS_SELECTOR, 'input[name="password"]')))
        print('   [UC-Stealth] Entering password...')
        password_input.send_keys(password)
        
        login_button = driver.find_element(By.XPATH, '//span[text()="Log in"]/ancestor::button | //button[@data-testid="LoginForm_Login_Button"]')
        login_button.click()
        
        print('   [UC-Stealth] Waiting for timeline to load...')
        wait.until(EC.visibility_of_element_located((By.CSS_SELECTOR, '[data-testid="SideNav_NewTweet_Button"], [data-testid="tweetButtonInline"], [data-testid="AppTabBar_Home_Link"]')))
        
        print('   [UC-Stealth] Navigating to compose window...')
        driver.get('https://x.com/compose/post')
        
        wait.until(EC.visibility_of_element_located((By.CSS_SELECTOR, '[data-testid="tweetTextarea_0"]')))
        print('   [UC-Stealth] Typing tweet...')
        
        tweet_box = driver.find_element(By.CSS_SELECTOR, '[data-testid="tweetTextarea_0"]')
        # Click first to focus, then send keys
        tweet_box.click()
        time.sleep(1)
        tweet_box.send_keys(text)
        
        time.sleep(1)
        
        print('   [UC-Stealth] Clicking Post...')
        post_btn = driver.find_element(By.CSS_SELECTOR, '[data-testid="tweetButton"]')
        post_btn.click()
        
        time.sleep(4)
        print('   [UC-Stealth] ✅ Successfully posted to X.com using Python Undetected_Chromedriver.')

    except Exception as e:
        print(f"   [UC-Stealth] ❌ Posting failed: {e}")
        driver.save_screenshot('uc_error.png')
        with open('error_uc_dom.html', 'w') as f:
            f.write(driver.page_source)
        sys.exit(1)
    finally:
        driver.quit()

if __name__ == "__main__":
    main()
