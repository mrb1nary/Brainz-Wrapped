type ListeningAge = {
    decade: string;
    listens: number;
};

type Props = {
    data: ListeningAge[];
};

export default function ListeningAge({ data }: Props) {
    if (!data || data.length === 0) return null;

    const total = data.reduce((sum, d) => sum + d.listens, 0);

    const top = data[0];

    return (
        <div className="relative overflow-hidden rounded-2xl border border-neutral-800 bg-neutral-900 p-6 shadow-lg shadow-black/30">

            {/* glow */}
            <div className="absolute -top-12 -right-12 w-48 h-48 bg-green-500/20 blur-3xl rounded-full" />

            <div className="relative space-y-6">

                {/* Header */}
                <div>
                    <p className="text-sm text-neutral-400 tracking-wide">
                        Listening Age
                    </p>

                    <h2 className="text-3xl font-bold text-green-400 mt-1">
                        {top.decade}
                    </h2>

                    <p className="text-sm text-neutral-400 mt-1">
                        Your most listened decade
                    </p>
                </div>

                {/* Bars */}
                <div className="space-y-3">
                    {data.map((d) => {
                        const percent = ((d.listens / total) * 100).toFixed(1);

                        return (
                            <div key={d.decade}>
                                <div className="flex justify-between text-xs text-neutral-400 mb-1">
                                    <span>{d.decade}</span>
                                    <span>{percent}%</span>
                                </div>

                                <div className="w-full h-2 bg-neutral-800 rounded-full overflow-hidden">
                                    <div
                                        className="h-full bg-green-500 transition-all duration-500"
                                        style={{ width: `${percent}%` }}
                                    />
                                </div>
                            </div>
                        );
                    })}
                </div>

                {/* Insight line */}
                <p className="text-xs text-neutral-500 pt-2">
                    You mostly live in the <span className="text-green-400">{top.decade}</span>,
                    with echoes from {data[1]?.decade || "other eras"}.
                </p>
            </div>
        </div>
    );
}