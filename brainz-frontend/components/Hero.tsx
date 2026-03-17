type Artist = {
    artist: string;
    listens: number;
    image?: string | null;
};

type Track = {
    track: string;
    artist: string;
    plays: number;
};

type HeroProps = {
    data: {
        top_artists: Artist[];
        top_tracks: Track[];
    };
};

// smarter capitalization (preserves initials like A.R., DJ, etc.)
function formatName(name: string): string {
    return name
        .split(" ")
        .map((word) => {
            if (word.includes(".")) return word.toUpperCase(); // keep A.R.
            return word.charAt(0).toUpperCase() + word.slice(1).toLowerCase();
        })
        .join(" ");
}

// fallback avatar
function fallbackImage(name: string) {
    return `https://ui-avatars.com/api/?name=${encodeURIComponent(
        name
    )}&background=111&color=22c55e&size=256`;
}

export default function Hero({ data }: HeroProps) {
    const topArtist = data?.top_artists?.[0];
    const topTrack = data?.top_tracks?.[0];

    if (!topArtist || !topTrack) {
        return (
            <div className="rounded-2xl border border-neutral-800 bg-neutral-900 p-8 text-neutral-400">
                No listening data yet.
            </div>
        );
    }

    return (
        <div className="relative overflow-hidden rounded-2xl border border-neutral-800 bg-neutral-900 p-8 shadow-lg shadow-black/30">

            {/* Background glow */}
            <div className="absolute -top-16 -right-16 w-64 h-64 bg-green-500/20 blur-3xl rounded-full" />
            <div className="absolute -bottom-16 -left-16 w-64 h-64 bg-emerald-400/10 blur-3xl rounded-full" />

            <div className="relative grid grid-cols-1 md:grid-cols-2 gap-8 items-center">

                {/* Artist */}
                <div className="flex items-center gap-6">

                    <img
                        src={topArtist.image || fallbackImage(topArtist.artist)}
                        alt={topArtist.artist}
                        loading="lazy"
                        className="
                            w-20 h-20 md:w-24 md:h-24
                            rounded-xl
                            object-cover
                            border border-neutral-700
                            shadow-md
                        "
                    />

                    <div>
                        <p className="text-sm text-neutral-400 tracking-wide">
                            Your #1 Artist
                        </p>

                        <h2 className="text-3xl md:text-5xl font-bold mt-1 text-green-400 leading-tight">
                            {formatName(topArtist.artist)}
                        </h2>

                        <p className="text-neutral-400 mt-1">
                            {topArtist.listens.toLocaleString()} listens this year
                        </p>
                    </div>
                </div>

                {/* Track */}
                <div className="md:border-l md:border-neutral-800 md:pl-8">
                    <p className="text-sm text-neutral-400 tracking-wide">
                        Top Track
                    </p>

                    <p className="text-xl md:text-2xl font-semibold mt-2 text-neutral-100 break-words">
                        {topTrack.track}
                    </p>

                    <p className="text-neutral-400 mt-1">
                        {formatName(topTrack.artist)}
                    </p>

                    <p className="text-sm text-neutral-500 mt-2">
                        {topTrack.plays.toLocaleString()} plays
                    </p>
                </div>
            </div>
        </div>
    );
}