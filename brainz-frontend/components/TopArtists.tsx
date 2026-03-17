type Artist = {
    artist: string;
    listens: number;
};

type TopArtistsProps = {
    artists: Artist[];
};

function capitalizeName(name: string): string {
    return name
        .split(" ")
        .map((word) =>
            word.charAt(0).toUpperCase() + word.slice(1).toLowerCase()
        )
        .join(" ");
}

export default function TopArtists({ artists }: TopArtistsProps) {

    const totalListens = artists.reduce((sum, a) => sum + a.listens, 0);
    const maxListens = Math.max(...artists.map((a) => a.listens));

    return (
        <div>
            <h2 className="text-2xl font-semibold mb-5">Top Artists</h2>

            <div className="space-y-3">
                {artists.map((a, i) => {
                    const percentage = totalListens
                        ? ((a.listens / totalListens) * 100).toFixed(1)
                        : "0";

                    const relativeWidth = maxListens
                        ? (a.listens / maxListens) * 100
                        : 0;

                    const isTop = i === 0;

                    return (
                        <div
                            key={`${a.artist}-${i}`}
                            className={`
                                group
                                flex flex-col
                                bg-neutral-900
                                border
                                ${isTop ? "border-green-500/40" : "border-neutral-800"}
                                px-5 py-4
                                rounded-xl
                                transition-all duration-200
                                hover:bg-neutral-800
                                hover:scale-[1.015]
                            `}
                        >

                            {/* Top row */}
                            <div className="flex items-center justify-between">

                                {/* Left */}
                                <div className="flex items-center gap-4 min-w-0">

                                    {/* Rank */}
                                    <span className={`
                                        text-sm w-5
                                        ${isTop ? "text-green-400 font-semibold" : "text-neutral-500"}
                                    `}>
                                        {i + 1}
                                    </span>

                                    {/* Artist */}
                                    <p className={`
                                        font-medium truncate
                                        ${isTop ? "text-green-300" : "text-neutral-100"}
                                    `}>
                                        {capitalizeName(a.artist)}
                                    </p>

                                </div>

                                {/* Right */}
                                <div className="text-right shrink-0">
                                    <p className="text-sm text-neutral-300">
                                        {a.listens}
                                    </p>
                                    <p className="text-xs text-neutral-500">
                                        {percentage}%
                                    </p>
                                </div>

                            </div>

                            {/* Progress bar */}
                            <div className="mt-3 h-1.5 bg-neutral-800 rounded-full overflow-hidden">
                                <div
                                    className={`
                                        h-full
                                        ${isTop ? "bg-green-400" : "bg-green-500/70"}
                                        transition-all duration-500
                                    `}
                                    style={{ width: `${relativeWidth}%` }}
                                />
                            </div>

                        </div>
                    );
                })}
            </div>
        </div>
    );
}