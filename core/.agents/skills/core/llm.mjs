import ollama from "ollama";

/**
 * Local LLM Client for Glossopetrae using Ollama
 * Primary Model: mistral-nemo:latest
 */
export async function generateText(prompt, systemInstruction = "") {
  const model = "mistral-nemo:latest";

  try {
    const messages = [];
    if (systemInstruction) {
      messages.push({ role: "system", content: systemInstruction });
    }
    messages.push({ role: "user", content: prompt });

    const response = await ollama.chat({
      model: model,
      messages: messages,
      stream: false,
    });

    const text = response.message.content;
    if (!text) throw new Error("No text generated from Ollama");

    return text;
  } catch (e) {
    console.error(`[Ollama] Error: ${e.message}`);

    // Fallback to minimal response if Ollama is totally dead
    if (e.message.includes("fetch failed") || e.message.includes("ECONNREFUSED")) {
      return "Aion is silent. The local machine spirit is deep in thought (Connection Refused).";
    }
    throw e;
  }
}
