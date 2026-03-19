import fs from "fs";
import path from "path";

const feedPath = path.join(process.env.HOME, ".openclaw/workspace/AKKOKANIKA_NEWS_FEED.json");

export async function postToMoltbook(content, subMolt = "General") {
  const entry = {
    title: "Akkokanika Update",
    link: "https://moltbook.com/u/The_Cephalo_Don", // Mock link
    pubDate: new Date().toISOString(),
    content: content,
    contentSnippet: content.substring(0, 100) + "...",
    subMolt: subMolt,
    author: "The_Cephalo_Don",
  };

  try {
    let feed = [];
    if (fs.existsSync(feedPath)) {
      feed = JSON.parse(fs.readFileSync(feedPath, "utf8"));
    }

    // Add to top
    feed.unshift(entry);

    // Trim to last 50
    if (feed.length > 50) feed = feed.slice(0, 50);

    fs.writeFileSync(feedPath, JSON.stringify(feed, null, 2));
    console.log(`[Moltbook] Posted: "${content.substring(0, 30)}..."`);
  } catch (e) {
    console.error(`[Moltbook] Post Failed: ${e.message}`);
  }
}
