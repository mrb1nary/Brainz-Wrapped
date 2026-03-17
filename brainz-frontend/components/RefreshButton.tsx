"use client";

import { useRouter } from "next/navigation";
import { useState } from "react";

export default function RefreshButton() {
    const router = useRouter();
    const [loading, setLoading] = useState(false);

    const handleRefresh = async () => {
        setLoading(true);

        await new Promise((r) => setTimeout(r, 300));

        router.refresh();
        setLoading(false);
    };

    return (
        <button
            onClick={handleRefresh}
            title="Fetch latest data (bypasses cache)"
            className={`
                group
                relative
                flex items-center gap-2
                px-4 py-2
                rounded-lg
                border border-neutral-700
                bg-neutral-900
                text-sm font-medium
                text-neutral-200
                transition-all duration-200
                hover:bg-neutral-800
                hover:border-green-500/40
                hover:text-green-400
                active:scale-95
                disabled:opacity-60
            `}
            disabled={loading}
        >
            {/* Icon */}
            <span
                className={`
                    transition-transform duration-500
                    ${loading ? "animate-spin" : "group-hover:rotate-180"}
                `}
            >
                ↻
            </span>

            {/* Text */}
            <span>
                {loading ? "Refreshing..." : "Refresh"}
            </span>

            {/* Tooltip */}
            <span
                className="
                    absolute -bottom-10 left-1/2 -translate-x-1/2
                    whitespace-nowrap
                    text-xs
                    bg-neutral-800
                    text-neutral-300
                    px-2 py-1
                    rounded-md
                    opacity-0
                    group-hover:opacity-100
                    transition
                    pointer-events-none
                "
            >
                Fetch latest data (ignore cache)
            </span>
        </button>
    );
}