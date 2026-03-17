"use client";

import ReactECharts from "echarts-for-react";

type Props = {
    data: [string, number][];
};

// backend → lowercase full names
const order = [
    "monday",
    "tuesday",
    "wednesday",
    "thursday",
    "friday",
    "saturday",
    "sunday",
];

const short = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

export default function WeekdayChart({ data }: Props) {

    // 🔥 convert array → map
    const map: Record<string, number> = {};
    data.forEach(([day, count]) => {
        map[day.toLowerCase()] = count;
    });

    // 🔥 ordered values
    const values = order.map((d) => map[d] || 0);

    const max = Math.max(...values);
    const peakIndex = values.findIndex((v) => v === max);

    const option = {
        backgroundColor: "transparent",

        tooltip: {
            trigger: "axis",
            backgroundColor: "#0a0a0a",
            borderColor: "#222",
            textStyle: { color: "#fff" },
            formatter: (params: any) => {
                const p = params[0];
                return `
                    <strong>${p.axisValue}</strong><br/>
                    ${p.value} listens
                `;
            },
        },

        grid: {
            left: 10,
            right: 10,
            top: 20,
            bottom: 20,
            containLabel: true,
        },

        xAxis: {
            type: "category",
            data: short,
            axisLine: { lineStyle: { color: "#333" } },
            axisLabel: { color: "#888" },
            axisTick: { show: false },
        },

        yAxis: {
            type: "value",
            axisLine: { show: false },
            splitLine: { lineStyle: { color: "#1f1f1f" } },
            axisLabel: { color: "#666" },
        },

        series: [
            {
                data: values,
                type: "bar",
                barWidth: "55%",
                itemStyle: {
                    borderRadius: [6, 6, 0, 0],
                    color: (params: any) => {
                        if (params.dataIndex === peakIndex) {
                            return "#4ade80"; // highlight peak
                        }
                        return "#22c55e";
                    },
                },
                animationDuration: 700,
            },
        ],
    };

    return (
        <div className="bg-neutral-900 border border-neutral-800 rounded-xl p-5 shadow-lg shadow-black/20">

            <div className="mb-4">
                <h2 className="text-2xl font-semibold">
                    Listening by Weekday
                </h2>

                <p className="text-sm text-neutral-500">
                    You listened the most on{" "}
                    <span className="text-green-400 font-medium">
                        {short[peakIndex]}
                    </span>
                </p>
            </div>

            <ReactECharts option={option} style={{ height: 280 }} />

        </div>
    );
}