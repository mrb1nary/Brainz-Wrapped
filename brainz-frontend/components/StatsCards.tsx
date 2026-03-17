type Sessions = {
    total_sessions: number;
    average_session_minutes: number;
    longest_session_minutes: number;
    longest_session_date?: string;
};

type Streak = {
    length: number;
    endDate: string | null;
};

type StatsData = {
    sessions: Sessions;
};

type StatsCardsProps = {
    data: StatsData;
    streak?: Streak;
};

function formatDate(date?: string | null): string {
    if (!date) return "";
    const d = new Date(date);
    return d.toLocaleDateString(undefined, {
        day: "numeric",
        month: "short",
    });
}

export default function StatsCards({ data, streak }: StatsCardsProps) {
    const sessions = data.sessions;

    const items = [
        {
            label: "Sessions",
            value: sessions.total_sessions,
            sub: "Total listening sessions",
            icon: "🎧",
        },
        {
            label: "Avg Session",
            value: `${sessions.average_session_minutes} min`,
            sub: "Average session length",
            icon: "⏱️",
        },
        {
            label: "Longest",
            value: `${sessions.longest_session_minutes} min`,
            sub: sessions.longest_session_date
                ? `on ${formatDate(sessions.longest_session_date)}`
                : "Longest single session",
            icon: "🔥",
        },
    ];

    return (
        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-5">
            {items.map((item, i) => (
                <div
                    key={i}
                    className={`
                        relative
                        overflow-hidden
                        rounded-xl
                        p-6
                        border
                        ${
                        item.highlight
                            ? "bg-neutral-900 border-green-500/30"
                            : "bg-neutral-900 border-neutral-800"
                    }
                        transition-all duration-200
                        hover:bg-neutral-800
                        hover:scale-[1.03]
                        shadow-lg shadow-black/20
                    `}
                >
                    <div className={`
                        absolute -top-8 -right-8 w-24 h-24 blur-3xl rounded-full
                        ${item.highlight ? "bg-green-500/20" : "bg-green-500/10"}
                    `} />

                    <div className="relative space-y-2">
                        <div className="flex items-center justify-between">
                            <p className="text-sm text-neutral-400 tracking-wide">
                                {item.label}
                            </p>
                            <span className="text-lg opacity-80">
                                {item.icon}
                            </span>
                        </div>

                        <p className={`
                            text-3xl font-bold
                            ${item.highlight ? "text-green-400" : "text-white"}
                        `}>
                            {item.value}
                        </p>

                        <p className="text-xs text-neutral-500">
                            {item.sub}
                        </p>
                    </div>
                </div>
            ))}
        </div>
    );
}