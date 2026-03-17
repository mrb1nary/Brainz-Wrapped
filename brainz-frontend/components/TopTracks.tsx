export default function TopTracks({ tracks }: any) {
    return (
        <div>
            <h2 className="text-2xl font-semibold mb-5">Top Tracks</h2>

            <div className="space-y-3">
                {tracks.map((t: any, i: number) => (
                    <div
                        key={i}
                        className="
              group
              bg-neutral-900
              border border-neutral-800
              px-5 py-4
              rounded-xl
              transition
              duration-200
              hover:bg-neutral-800
              hover:scale-[1.015]
            "
                    >
                        <div className="flex items-center justify-between">

                            {/* Left side */}
                            <div className="flex items-center gap-4 min-w-0">

                                {/* Rank */}
                                <span className="text-neutral-500 text-sm w-5">
                  {i + 1}
                </span>

                                {/* Track + Artist */}
                                <div className="min-w-0">
                                    <p className="text-neutral-100 font-medium truncate">
                                        {t.track}
                                    </p>

                                    <p className="text-sm text-neutral-400 truncate">
                                        {t.artist}
                                    </p>
                                </div>

                            </div>

                            {/* Plays */}
                            <span className="text-sm text-neutral-500 shrink-0">
                {t.plays} plays
              </span>

                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
}