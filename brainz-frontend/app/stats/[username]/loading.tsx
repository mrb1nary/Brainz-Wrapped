export default function Loading() {
    return (
        <div className="min-h-screen bg-neutral-950 px-4 py-12 text-white">

            <div className="max-w-5xl mx-auto space-y-10 animate-pulse">

                {/* Title */}
                <div className="h-8 w-64 bg-neutral-800 rounded mx-auto" />

                {/* Hero */}
                <div className="h-40 bg-neutral-900 border border-neutral-800 rounded-2xl" />

                {/* Stats */}
                <div className="grid grid-cols-3 gap-4">
                    {[...Array(3)].map((_, i) => (
                        <div key={i} className="h-28 bg-neutral-900 border border-neutral-800 rounded-xl" />
                    ))}
                </div>

                {/* Chart */}
                <div className="h-72 bg-neutral-900 border border-neutral-800 rounded-xl" />

                {/* Lists */}
                <div className="grid grid-cols-2 gap-4">
                    <div className="h-40 bg-neutral-900 rounded-xl" />
                    <div className="h-40 bg-neutral-900 rounded-xl" />
                </div>

                {/* Heatmap */}
                <div className="h-48 bg-neutral-900 rounded-xl" />

            </div>
        </div>
    );
}