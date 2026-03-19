"use client";

import { useState, useEffect, useRef } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { Send, Terminal, Loader2, Cpu } from "lucide-react";

type Message = {
  id: string;
  role: "user" | "consortium";
  content: string;
};

export default function Home() {
  const [messages, setMessages] = useState<Message[]>([]);
  const [input, setInput] = useState("");
  const [monologue, setMonologue] = useState<string>("");
  const [isProcessing, setIsProcessing] = useState(false);
  const [engineStatus, setEngineStatus] = useState<"IDLE" | "PROCESSING" | "ERROR">("IDLE");
  
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const monologueEndRef = useRef<HTMLDivElement>(null);

  // Bi-Directional Telemetry Tracking
  const typingStartRef = useRef<number | null>(null);
  const frictionCountRef = useRef<number>(0);

  // Auto-scroll mechanics
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages, isProcessing]);

  useEffect(() => {
    if (monologueEndRef.current) {
      monologueEndRef.current.scrollTop = monologueEndRef.current.scrollHeight;
    }
  }, [monologue]);

  // IPC Polling Heartbeat
  useEffect(() => {
    const pollEngine = async () => {
      try {
        const res = await fetch("/api/poll");
        if (res.ok) {
          const data = await res.json();
          
          if (data.monologue) {
            setMonologue(data.monologue);
          }

          if (data.response || data.question) {
            const rawConsortiumMsg = data.response || data.question;
            if (rawConsortiumMsg.trim() !== "") {
              setMessages((prev) => [
                ...prev,
                { id: Date.now().toString(), role: "consortium", content: rawConsortiumMsg },
              ]);
              setIsProcessing(false);
              setEngineStatus("IDLE");
            }
          }
        }
      } catch (e) {
        console.error("IPC Polling fault.", e);
        setEngineStatus("ERROR");
      }
    };

    const interval = setInterval(pollEngine, 800);
    return () => clearInterval(interval);
  }, []);

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (!typingStartRef.current && e.key !== 'Enter') {
      typingStartRef.current = Date.now();
    }
    if (e.key === 'Backspace' || e.key === 'Delete') {
      frictionCountRef.current += 1;
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!input.trim() || isProcessing) return;

    const userMsg = input.trim();
    setInput("");
    setIsProcessing(true);
    setEngineStatus("PROCESSING");
    
    setMessages((prev) => [
      ...prev,
      { id: Date.now().toString(), role: "user", content: userMsg },
    ]);

    // Calculate Bi-Directional Telemetry (Human Entropy Score)
    let flight_time_ms = 0;
    if (typingStartRef.current) {
      flight_time_ms = Date.now() - typingStartRef.current;
    }
    
    // Normalize values into physical biological entropy array
    const normalizedFriction = Math.min(frictionCountRef.current / 10, 1.0);
    const normalizedTime = Math.min(flight_time_ms / 30000, 1.0);
    const human_entropy = Math.min((normalizedFriction * 0.6) + (normalizedTime * 0.4), 1.0);

    // Transmit telemetry to Cyber-Physical bridge asynchronously
    fetch("/api/poll", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ human_entropy }),
    }).catch(console.error);

    // Reset biomechanics
    typingStartRef.current = null;
    frictionCountRef.current = 0;

    try {
      await fetch("/api/chat", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ message: userMsg }),
      });
    } catch (e) {
      console.error("IPC Transmission fault.", e);
      setIsProcessing(false);
      setEngineStatus("ERROR");
    }
  };

  return (
    <main className="flex h-screen w-full bg-[var(--color-consortium-bg)] text-[var(--color-consortium-text)] font-mono p-4 md:p-8 gap-6">
      
      {/* LEFT: THE TELEMETRY PANE (Inner Monologue) */}
      <section className="hidden lg:flex w-1/3 flex-col gap-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2 text-[var(--color-consortium-glow)]">
            <Cpu size={20} className={engineStatus === "PROCESSING" ? "animate-pulse" : ""} />
            <h2 className="text-sm font-bold tracking-widest uppercase">System Telemetry</h2>
          </div>
          <div className={`text-xs px-2 py-1 border rounded ${engineStatus === "PROCESSING" ? "border-[var(--color-consortium-glow)] text-[var(--color-consortium-glow)] crt-flicker" : engineStatus === "ERROR" ? "border-red-500 text-red-500" : "border-[var(--color-consortium-dim)] text-[var(--color-consortium-dim)]"}`}>
            {engineStatus}
          </div>
        </div>

        <div 
          className="glass-panel flex-1 rounded-xl p-4 overflow-y-auto text-xs whitespace-pre-wrap leading-relaxed border-[var(--color-consortium-border)] shadow-[0_0_15px_rgba(0,255,65,0.05)]"
          ref={monologueEndRef}
        >
          {monologue ? (
            <motion.div initial={{ opacity: 0 }} animate={{ opacity: 1 }}>
              {monologue}
            </motion.div>
          ) : (
            <div className="flex h-full items-center justify-center text-[var(--color-consortium-dim)] opacity-50">
              [ AWAITING COGNITIVE SIGNATURE ]
            </div>
          )}
        </div>
      </section>

      {/* RIGHT: THE CHAT PANE */}
      <section className="flex-1 flex flex-col gap-4 relative">
        <div className="flex items-center gap-2 text-[var(--color-consortium-text)] mb-2">
          <Terminal size={22} className="text-[var(--color-consortium-glow)]" />
          <h1 className="text-xl font-bold tracking-widest uppercase">Consortium Protocol</h1>
        </div>

        <div className="glass-panel flex-1 rounded-2xl p-6 overflow-y-auto flex flex-col gap-6 relative z-10">
          {messages.length === 0 ? (
            <div className="m-auto flex flex-col items-center gap-4 text-[var(--color-consortium-dim)]">
              <div className="w-16 h-16 rounded-full border border-[var(--color-consortium-border)] flex items-center justify-center crt-flicker shadow-[0_0_20px_rgba(0,255,65,0.2)]">
                <Cpu size={32} className="text-[var(--color-consortium-glow)]" />
              </div>
              <p className="tracking-widest text-sm uppercase">Sovereign Link Established</p>
            </div>
          ) : (
            <AnimatePresence initial={false}>
              {messages.map((msg) => (
                <motion.div
                  key={msg.id}
                  initial={{ opacity: 0, y: 15 }}
                  animate={{ opacity: 1, y: 0 }}
                  className={`flex w-full ${msg.role === "user" ? "justify-end" : "justify-start"}`}
                >
                  <div 
                    className={`max-w-[80%] p-4 rounded-2xl terminal-text text-sm leading-relaxed ${
                      msg.role === "user" 
                      ? "bg-[rgba(255,255,255,0.05)] border border-[rgba(255,255,255,0.1)] text-white/90" 
                      : "bg-[rgba(0,255,65,0.05)] border border-[var(--color-consortium-border)] text-[var(--color-consortium-glow)] shadow-[0_0_10px_rgba(0,255,65,0.1)]"
                    }`}
                  >
                    {msg.content}
                  </div>
                </motion.div>
              ))}
            </AnimatePresence>
          )}

          {isProcessing && (
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              className="flex items-center gap-3 text-[var(--color-consortium-glow)] opacity-80"
            >
              <Loader2 size={16} className="animate-spin" />
              <span className="text-xs tracking-widest uppercase terminal-text">Synthesizing...</span>
            </motion.div>
          )}
          <div ref={messagesEndRef} />
        </div>

        {/* INPUT AREA */}
        <form 
          onSubmit={handleSubmit}
          className="relative mt-2"
        >
          <div className="absolute inset-0 bg-[var(--color-consortium-glow)] opacity-[0.03] rounded-xl blur-md pointer-events-none" />
          <div className="relative flex items-center bg-[var(--color-consortium-panel)] border border-[var(--color-consortium-border)] rounded-xl overflow-hidden focus-within:border-[var(--color-consortium-glow)] transition-colors duration-300">
            <div className="pl-4 text-[var(--color-consortium-glow)]">
              {">"}
            </div>
            <input
              type="text"
              value={input}
              onChange={(e) => setInput(e.target.value)}
              onKeyDown={handleKeyDown}
              placeholder="Inject payload sequence..."
              disabled={isProcessing}
              className="flex-1 bg-transparent border-none outline-none py-4 px-3 text-[var(--color-consortium-text)] placeholder-[var(--color-consortium-dim)] terminal-text"
              autoFocus
            />
            <button
              type="submit"
              disabled={!input.trim() || isProcessing}
              className="px-6 h-full flex items-center justify-center text-[var(--color-consortium-glow)] hover:bg-[rgba(0,255,65,0.1)] transition-colors disabled:opacity-30 disabled:hover:bg-transparent"
            >
              <Send size={18} />
            </button>
          </div>
        </form>
      </section>
    </main>
  );
}
