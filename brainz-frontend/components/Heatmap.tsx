"use client";

import { useMemo } from "react";
import dayjs from "dayjs";

export default function Heatmap({ data }: any) {

    const days = useMemo(() => {

        const year = dayjs().year();
        const start = dayjs(`${year}-01-01`);
        const end = dayjs(`${year}-12-31`);

        const totalDays = end.diff(start, "day") + 1;

        const result = [];

        for (let i = 0; i < totalDays; i++) {
            const date = start.add(i, "day");
            const key = date.format("YYYY-MM-DD");

            result.push({
                date: key,
                count: data?.[key] || 0,
                day: date.day(),
            });
        }

        return result;
    }, [data]);

    const getColor = (count: number) => {
        if (count === 0) return "bg-neutral-800";
        if (count < 3) return "bg-green-200";
        if (count < 8) return "bg-green-300";
        if (count < 15) return "bg-green-400";
        return "bg-green-500";
    };

    const weeks = Math.ceil(days.length / 7);

    return (
        <div className="bg-neutral-900 border border-neutral-800 rounded-xl p-6">

            <h2 className="text-2xl font-semibold mb-6 text-center">
                Listening Activity 2026
            </h2>

            {/* Center container */}
            <div className="flex justify-center">

                {/* Fit-to-width heatmap */}
                <div className="flex gap-1">

                    {Array.from({ length: weeks }).map((_, weekIndex) => {

                        const week = days.slice(weekIndex * 7, weekIndex * 7 + 7);

                        return (
                            <div key={weekIndex} className="flex flex-col gap-1">

                                {week.map((d: any, i: number) => (
                                    <div
                                        key={i}
                                        title={`${d.date} • ${d.count} listens`}
                                        className={`
                                            w-[10px] h-[10px]
                                            sm:w-[12px] sm:h-[12px]
                                            md:w-[14px] md:h-[14px]
                                            rounded-sm
                                            ${getColor(d.count)}
                                            transition-all duration-200
                                            hover:scale-125
                                        `}
                                    />
                                ))}

                            </div>
                        );
                    })}

                </div>

            </div>

        </div>
    );
}