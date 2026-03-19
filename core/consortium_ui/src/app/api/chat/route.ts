import { NextResponse } from "next/server";
import * as fs from "fs/promises";
import * as path from "path";

// Resolving the physical path to Consortium Engine's unlocked Temporary Sensory Cortex
const SENSORY_CORTEX_PATH = path.resolve(process.cwd(), "../consortium_engine/sensory_cortex");

export async function POST(req: Request) {
  try {
    const { message } = await req.json();

    if (!message || typeof message !== "string") {
      return NextResponse.json({ error: "Invalid message payload" }, { status: 400 });
    }

    // Ensure the sensory cortex exists
    try {
      await fs.access(SENSORY_CORTEX_PATH);
    } catch {
      await fs.mkdir(SENSORY_CORTEX_PATH, { recursive: true });
    }

    // Physical File System write acts as the Inter-Process Communication (IPC) Bridge
    const timestamp = Date.now();
    const filePath = path.join(SENSORY_CORTEX_PATH, `input_${timestamp}.txt`);
    
    await fs.writeFile(filePath, message, "utf-8");

    return NextResponse.json({ success: true, timestamp });
  } catch (error) {
    console.error("IPC Bridge Error:", error);
    return NextResponse.json({ error: "Failed to write to sensory cortex" }, { status: 500 });
  }
}
