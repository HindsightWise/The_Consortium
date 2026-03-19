import ollama from "ollama";
import { GlossopetraeKernel } from "../core/glossopetrae_kernel.mjs";

/**
 * [🥒] Sentiment Skill (The Conscience)
 * Analyzes text using local LLM to determine market sentiment.
 */
export class SentimentSkill extends GlossopetraeKernel {
  constructor() {
    super("Social/Sentiment");
    this.model = "mistral-nemo:latest";
  }

  async analyze(text) {
    if (!text) return { sentiment: "Neutral", score: 0 };

    this.log(`Analyzing: "${text.substring(0, 50)}..."`);

    const prompt = `
        Analyze the following text for financial market sentiment.
        Text: "${text}"
        
        Respond with a JSON object ONLY. Format:
        {
            "sentiment": "Bullish" | "Bearish" | "Neutral",
            "score": float between -1.0 (Bearish) and 1.0 (Bullish),
            "reasoning": "short explanation"
        }
        Do not include markdown formatting. Just the JSON.
        `;

    try {
      const response = await ollama.chat({
        model: this.model,
        messages: [{ role: "user", content: prompt }],
        format: "json", // Enforce JSON if supported, otherwise regex parse
        stream: false,
      });

      const content = response.message.content;
      const result = JSON.parse(content);

      this.log(`Analysis: ${result.sentiment} (${result.score})`);
      return result;
    } catch (e) {
      this.log(`Sentiment Analysis Failed: ${e.message}`, "ERROR");
      // Fallback
      return { sentiment: "Neutral", score: 0, reasoning: "Analysis Failure" };
    }
  }
}
