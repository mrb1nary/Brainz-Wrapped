"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { Github } from "lucide-react";

export default function HomePage() {
  const [username, setUsername] = useState("");
  const router = useRouter();

  const handleSubmit = (e?: React.FormEvent) => {
    e?.preventDefault();

    const trimmed = username.trim();
    if (!trimmed) return;

    router.push(`/stats/${trimmed}`);
  };

  return (
      <div className="relative min-h-screen bg-neutral-950 text-white flex items-center justify-center px-6 overflow-hidden">

        {/* Background */}
        <div className="absolute inset-0 -z-10 pointer-events-none">
          <div className="absolute top-[-20%] left-[-10%] w-[600px] h-[600px] bg-green-500/20 blur-3xl rounded-full animate-pulse pointer-events-none" />
          <div className="absolute bottom-[-20%] right-[-10%] w-[600px] h-[600px] bg-emerald-400/10 blur-3xl rounded-full animate-pulse delay-1000 pointer-events-none" />
        </div>

        <div className="max-w-2xl w-full text-center space-y-12 animate-fade-up">

          {/* Title */}
          <div className="space-y-5">
            <h1 className="text-6xl md:text-7xl font-bold tracking-tight bg-gradient-to-r from-green-400 via-emerald-300 to-green-500 bg-clip-text text-transparent">
              BrainzWrapped
            </h1>

            <p className="text-neutral-400 text-xl md:text-2xl">
              Your music, decoded into patterns, eras, and stories
            </p>
          </div>

          {/* Card */}
          <form
              onSubmit={handleSubmit}
              className="
            relative
            bg-neutral-900/80
            backdrop-blur-xl
            border border-neutral-800
            rounded-3xl
            p-8
            shadow-xl shadow-black/40
            space-y-8
          "
          >
            {/* Glow (fixed: no pointer events) */}
            <div className="absolute -top-12 -right-12 w-48 h-48 bg-green-500/20 blur-3xl rounded-full pointer-events-none" />

            {/* Input */}
            <div className="flex gap-4">
              <input
                  type="text"
                  placeholder="Enter your ListenBrainz username"
                  value={username}
                  onChange={(e) => setUsername(e.target.value)}
                  className="
                flex-1
                bg-neutral-950
                border border-neutral-800
                rounded-2xl
                px-5 py-4
                text-lg
                text-white
                placeholder-neutral-500
                focus:outline-none
                focus:ring-2
                focus:ring-green-500
                transition
              "
              />

              <button
                  type="submit"
                  className="
                bg-green-500
                hover:bg-green-400
                active:scale-95
                text-black
                font-semibold
                text-lg
                px-8
                rounded-2xl
                transition-all
                duration-200
                shadow-md shadow-green-500/20
              "
              >
                Generate
              </button>
            </div>

            {/* Trust badges */}
            <div className="flex justify-center gap-5 text-sm text-neutral-300 pt-2 flex-wrap">

              <div className="flex items-center gap-2 px-4 py-2 border border-neutral-800 rounded-full">
                <span className="text-lg">🚫</span>
                <span>No Login Required</span>
              </div>

              <div className="flex items-center gap-2 px-4 py-2 border border-neutral-800 rounded-full">
                <span className="text-lg">🛡️</span>
                <span>No Data Collected</span>
              </div>

              <div className="flex items-center gap-2 px-4 py-2 border border-neutral-800 rounded-full">
                <span className="text-lg">⚡</span>
                <span>Instant Insights</span>
              </div>

            </div>
          </form>

          {/* GitHub Button */}
          <div className="flex justify-center">
            <a
                href="https://github.com/mrb1nary/Brainz-Wrapped"
                target="_blank"
                rel="noopener noreferrer"
                className="
              flex items-center gap-3
              px-6 py-3
              text-base
              bg-neutral-900
              border border-neutral-800
              rounded-xl
              hover:bg-neutral-800
              transition
            "
            >
              <Github className="w-5 h-5" />
              <span>View on GitHub</span>
            </a>
          </div>

          {/* Footer */}
          <p className="text-sm text-neutral-600 tracking-wide">
            Built on ListenBrainz • Open Source
          </p>

        </div>
      </div>
  );
}