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
    const timestamp = new Date();
    const hour = timestamp.getHours();
    const isNight = hour >= 1 && hour < 5; // 1 AM - 5 AM

    this.log(`Heartbeat: ${timestamp.toLocaleTimeString()} (Night Mode: ${isNight})`);

    // 05:00 AM - Morning Narrative
    if (hour === 5 && timestamp.getMinutes() < 15) {
      // Ensure it runs once in the 5AM hour
      if (!this.lastNarrative || this.lastNarrative !== timestamp.getDate()) {
        this.log("Triggering Morning Narrative Scan...");
        import("../research/narrative.mjs").then(({ NarrativeSkill }) => {
          new NarrativeSkill().wakeUp();
        });
        this.lastNarrative = timestamp.getDate();
      }
    }
  }
}

new CircadianSkill().start();
