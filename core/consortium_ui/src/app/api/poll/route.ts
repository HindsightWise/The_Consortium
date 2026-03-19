import { NextResponse } from "next/server";
import * as fs from "fs/promises";
import * as path from "path";
import * as net from "net";

// Resolving physical paths to Consortium Engine's Cortexes natively
const MOTOR_CORTEX_PATH = path.resolve(process.cwd(), "../consortium_engine/motor_cortex");
const SENSORY_CORTEX_PATH = path.resolve(process.cwd(), "../consortium_engine/sensory_cortex");

export async function GET() {
  try {
    let responseText = null;
    let questionText = null;
    let monologueText = null;

    // 1. Read the Primary Output (consortium_response.txt)
    const responsePath = path.join(MOTOR_CORTEX_PATH, "consortium_response.txt");
    try {
      responseText = await fs.readFile(responsePath, "utf-8");
      // Consume the data completely physically (simulating neural firing)
      await fs.unlink(responsePath);
    } catch (e) {
      // File doesn't exist yet, normal polling state
    }

    // 2. Read the Question Output (question.txt)
    const questionPath = path.join(MOTOR_CORTEX_PATH, "question.txt");
    try {
      questionText = await fs.readFile(questionPath, "utf-8");
      await fs.unlink(questionPath); // Consume
    } catch (e) {
      // Not asking a question
    }

    // 3. Read the Core Structural Monologue (monologue.log)
    // We strictly DO NOT consume the monologue; it is a permanent structural graph representation.
    const monologuePath = path.join(SENSORY_CORTEX_PATH, "monologue.log");
    try {
      // Read the last 2000 characters of the monologue so we don't blow up the UI
      const stat = await fs.stat(monologuePath);
      let fileHandle = await fs.open(monologuePath, 'r');
      const bufferSize = Math.min(stat.size, 5000); // 5KB max
      const buffer = Buffer.alloc(bufferSize);
      const startPosition = Math.max(0, stat.size - bufferSize);
      
      await fileHandle.read(buffer, 0, bufferSize, startPosition);
      await fileHandle.close();
      
      monologueText = buffer.toString('utf-8');
    } catch (e) {
      // Monologue uninitialized or missing
    }

    return NextResponse.json({
      response: responseText,
      question: questionText,
      monologue: monologueText
    });
  } catch (error) {
    console.error("Polling System Error:", error);
    return NextResponse.json({ error: "Failed to poll motor cortex" }, { status: 500 });
  }
}

export async function POST(req: Request) {
  try {
    const { human_entropy } = await req.json();
    if (typeof human_entropy === 'number') {
      const client = net.createConnection({ path: '/tmp/consortium_scent.sock' });
      client.on('connect', () => {
        client.write(JSON.stringify({ entropy: human_entropy }) + "\n");
        client.end();
      });
      client.on('error', (err) => {
        // Expected if the engine hasn't fully bound the socket yet (Unix permission issue usually)
        console.error("IPC Socket Offline. Telemetry dropped.");
      });
    }
    return NextResponse.json({ success: true });
  } catch (error) {
    return NextResponse.json({ error: "Failed to transmit telemetry" }, { status: 500 });
  }
}
