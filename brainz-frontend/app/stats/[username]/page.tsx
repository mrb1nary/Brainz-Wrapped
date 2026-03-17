import TopArtists from "@/components/TopArtists";
import TopTracks from "@/components/TopTracks";
import StatsCards from "@/components/StatsCards";
import HourlyChart from "@/components/HourlyChart";
import Hero from "@/components/Hero";
import Heatmap from "@/components/Heatmap";
import RefreshButton from "@/components/RefreshButton";
import WeekdayChart from "@/components/WeekdayChart";
import ListeningAge from "@/components/ListeningAge";

type WeekdayData = [string, number][];

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

type StatsData = {
    hourly: Record<string, number>;
    heatmap: Record<string, number>;
    top_artists: Artist[];
    top_tracks: Track[];
    sessions: {
        total_sessions: number;
        average_session_minutes: number;
        longest_session_minutes: number;
        longest_session_date?: string;
    };
};

type ListeningAgeItem = {
    decade: string;
    listens: number;
};

type ListeningAgeData = ListeningAgeItem[];

/* ---------------- FETCHERS ---------------- */

async function getStats(username: string): Promise<StatsData> {
    const res = await fetch(`http://localhost:3001/stats/${username}`, {
        cache: "no-store",
    });

    if (!res.ok) throw new Error("Failed to fetch stats");
    return res.json();
}

async function getWeekday(username: string): Promise<WeekdayData> {
    const res = await fetch(`http://localhost:3001/weekday/${username}`, {
        next: { revalidate: 3600 },
    });

    if (!res.ok) return [];
    return res.json();
}

async function getListeningAge(username: string): Promise<ListeningAgeData> {
    const res = await fetch(`http://localhost:3001/listening-age/${username}`, {
        cache: "no-store",
    });

    if (!res.ok) return [];
    return res.json();
}

/* ---------------- PAGE ---------------- */

export default async function StatsPage({
                                            params,
                                        }: {
    params: Promise<{ username: string }>;
}) {
    const { username } = await params;

    const [data, weekday, listeningAge] = await Promise.all([
        getStats(username),
        getWeekday(username),
        getListeningAge(username),
    ]);

    return (
        <div className="min-h-screen bg-gradient-to-b from-neutral-950 via-neutral-950 to-neutral-900 text-white px-4 md:px-6 py-12">

            <div className="max-w-5xl mx-auto space-y-14">

                {/* Title + Refresh */}
                <div className="flex items-center justify-between animate-fade-up">
                    <h1 className="text-3xl md:text-4xl font-bold">
                        🎧 {username}'s BrainzWrapped
                    </h1>
                    <RefreshButton />
                </div>

                {/* Hero */}
                <div className="animate-fade-up animate-delay-1">
                    <Hero data={data} />
                </div>

                {/* Stats */}
                <div className="animate-fade-up animate-delay-2">
                    <StatsCards data={data} />
                </div>

                {/* Listening Age */}
                <div className="animate-fade-up animate-delay-3">
                    <ListeningAge data={listeningAge} />
                </div>

                {/* Hourly Chart */}
                <div className="animate-fade-up animate-delay-4">
                    <HourlyChart data={data.hourly} />
                </div>

                {/* Weekday Chart */}
                <div className="animate-fade-up animate-delay-5">
                    <WeekdayChart data={weekday} />
                </div>

                {/* Artists + Tracks */}
                <div className="grid grid-cols-1 md:grid-cols-2 gap-6 animate-fade-up animate-delay-6">
                    <TopArtists artists={data.top_artists} />
                    <TopTracks tracks={data.top_tracks} />
                </div>
            </div>

            {/* Heatmap */}
            <section className="mt-24 w-full flex justify-center animate-fade-up animate-delay-6">
                <div className="w-full max-w-[1600px] px-4">

                    <div className="mb-6 text-center">
                        <h2 className="text-xl md:text-2xl font-semibold text-neutral-200">
                            Your Year in Listening
                        </h2>
                        <p className="text-sm text-neutral-500">
                            Every day you pressed play.
                        </p>
                    </div>

                    <Heatmap data={data.heatmap} />
                </div>
            </section>

        </div>
    );
}