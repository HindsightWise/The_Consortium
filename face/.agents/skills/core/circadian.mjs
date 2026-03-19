import { GlossopetraeKernel } from "./glossopetrae_kernel.mjs";

/**
 * [🥒] The Circadian Clock
 * Manages Day/Night cycles and triggers agent behaviors.
 */
class CircadianSkill extends GlossopetraeKernel {
  constructor() {
    super("Core/Circadian");
  }

  start() {
    this.log("Clock Ticking...");
    setInterval(() => this.tick(), 1000 * 60 * 15); // Check every 15m
    this.tick(); // Initial check
  }

  tick() {
    const now = new Date();
    const hour = now.getHours();
    const isNight = hour >= 1 && hour < 5; // 1 AM - 5 AM

    this.log(`Heartbeat: ${now.toLocaleTimeString()} (Night Mode: ${isNight})`);

    // Check if we need to switch modes or trigger events
    // Logic from original circadian.js would go here
  }
}

new CircadianSkill().start();
