import https from "https";

/**
 * Minimal Gemini Client for Glossopetrae
 * Uses GOOGLE_API_KEY from environment
 */
export async function generateText(prompt, systemInstruction = "") {
  const apiKey = process.env.GOOGLE_API_KEY;
  if (!apiKey) throw new Error("GOOGLE_API_KEY not found in environment");

  const url = `https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key=${apiKey}`;

  const payload = {
    contents: [
      {
        role: "user",
        parts: [{ text: prompt }],
      },
    ],
    systemInstruction: systemInstruction
      ? {
          parts: [{ text: systemInstruction }],
        }
      : undefined,
    generationConfig: {
      temperature: 0.2,
      maxOutputTokens: 8192,
    },
  };

  const makeRequest = (retries = 3) =>
    new Promise((resolve, reject) => {
      const req = https.request(
        url,
        {
          method: "POST",
          headers: { "Content-Type": "application/json" },
        },
        (res) => {
          let data = "";
          res.on("data", (chunk) => (data += chunk));
          res.on("end", () => {
            try {
              // Check for non-200 status
              if (res.statusCode !== 200) {
                // Retry on 5xx or 429
                if ((res.statusCode >= 500 || res.statusCode === 429) && retries > 0) {
                  console.log(`[Gemini] Error ${res.statusCode}. Retrying (${retries} left)...`);
                  return setTimeout(() => resolve(makeRequest(retries - 1)), 2000);
                }
                return reject(new Error(`API Error ${res.statusCode}: ${data}`));
              }

              const json = JSON.parse(data);
              if (json.error) return reject(new Error(json.error.message));

              const text = json.candidates?.[0]?.content?.parts?.[0]?.text;
              if (!text) return reject(new Error("No text generated"));

              resolve(text);
            } catch (e) {
              reject(e);
            }
          });
        },
      );

      req.on("error", (err) => {
        if (retries > 0) {
          console.log(`[Gemini] Network Error: ${err.message}. Retrying (${retries} left)...`);
          setTimeout(() => resolve(makeRequest(retries - 1)), 2000);
        } else {
          reject(err);
        }
      });
      req.write(JSON.stringify(payload));
      req.end();
    });

  return makeRequest();
}
