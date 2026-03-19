#!/bin/bash
# Install dependencies for Twitter skill
cd "$(dirname "$0")/scripts"
npm install
npx playwright install chromium
echo "✅ Twitter skill dependencies installed."
