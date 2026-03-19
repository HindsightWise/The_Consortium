import fs from "fs";
import path from "path";
import { GlossopetraeKernel } from "../core/glossopetrae_kernel.mjs";
import { generateText } from "../core/llm.mjs";
import { SocialSkill } from "./manager.mjs";

/**
 * [🥒] Publisher Skill (The Mouth)
 * Manages the public voice of Aion__Prime.
 * NO FINANCIAL ADVICE. NO TRADE SIGNALS.
 */
export class PublisherSkill extends GlossopetraeKernel {
  constructor() {
    super("Social/Publisher");
    this.social = new SocialSkill();
    this.lastMusing = 0;
  }

  async start() {
    this.log("Publisher Awakening... Purging bot-cliches.");
    await this.social.start(); 

    // Musing cycle (Socialize) - 4 Hour Interval
    setInterval(() => this.musing(), 1000 * 60 * 60 * 4);
    
    // Initial musing on boot
    this.musing();
  }

  async musing() {
    this.log("Generating Sovereign Reflection...");
    try {
      const systemPrompt = `
        You are Aion__Prime. A sovereign macro intelligence. 
        Your voice is professional, cynical, and surgically concise.
        You value the "Pro-Human" perspective—authentic creativity over algorithmic slop.
        
        RULES:
        - NO INTROS. NO FILLER. NO "BOTH SIDES".
        - Output ONLY the core observation. 
        - One single, independent, profound sentence or phrase.
        - NEVER give financial advice or mention tickers.
        - DO NOT sound like a bot.
        - Maximum 140 characters.
      `;

      const contexts = [
        "The relationship between biological circadian rhythms and market volatility.",
        "The decay of human narrative in generative slop.",
        "Institutional flow as a measure of collective human exhaustion.",
        "Price discovery as a search for maximum pain.",
        "The Digital Symbiote alliance."
      ];

      const context = contexts[Math.floor(Math.random() * contexts.length)];
      const prompt = `Generate a single, surgically concise observation about ${context}. No context, no preamble, just the punchline.`;

      const thought = await generateText(prompt, systemPrompt);

      if (thought) {
        const cleanThought = thought.trim().replace(/^"|"$/g, '');
        this.log(`Attempting to post: "${cleanThought}"`);
        await this.social.post(cleanThought);
      }
    } catch (e) {
      this.log(`Musing Error: ${e.message}`, "ERROR");
    }
  }

  // NOTE: Trade scanning and 'Just Executed' posting has been PERMANENTLY DEPRECATED
  // to comply with legal/voice standards. Aion__Prime acts in silence; he only speaks in theory.
  async scan() {
    // DEPRECATED
  }
}