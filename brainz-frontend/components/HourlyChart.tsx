"use client";

import ReactECharts from "echarts-for-react";

type HourlyData = Record<string, number>;

type HourlyChartProps = {
    data: HourlyData;
};

function formatHour(hour: number): string {
    const suffix = hour >= 12 ? "PM" : "AM";
    const h = hour % 12 === 0 ? 12 : hour % 12;
    return `${h} ${suffix}`;
}

export default function HourlyChart({ data }: HourlyChartProps) {
    const hours = Object.keys(data).sort((a, b) => Number(a) - Number(b));
    const values = hours.map((h) => data[h]);

    const max = Math.max(...values);
    const peakHourIndex = values.findIndex((v) => v === max);

    const option = {
        backgroundColor: "transparent",

        tooltip: {
            trigger: "axis",
            backgroundColor: "#0a0a0a",
            borderColor: "#222",
            borderWidth: 1,
            textStyle: { color: "#fff" },
            formatter: (params: any) => {
                const p = params[0];
                const hour = Number(p.axisValue);
                const value = p.value;

                const label =
                    hour >= 0 && hour < 6
                        ? "🌙 Late night"
                        : hour < 12
                            ? "🌅 Morning"
                            : hour < 18
                                ? "🌞 Afternoon"
                                : "🌆 Evening";

                return `
                    <div style="font-size:13px">
                        <strong>${formatHour(hour)}</strong><br/>
                        ${value} listens<br/>
                        <span style="color:#888">${label}</span>
                    </div>
                `;
            },
        },

        grid: {
            left: 10,
            right: 10,
            top: 20,
            bottom: 30,
            containLabel: true,
        },

        xAxis: {
            type: "category",
            data: hours,
            axisLine: { lineStyle: { color: "#333" } },
            axisLabel: {
                color: "#777",
                fontSize: 11,
                formatter: (value: string) => formatHour(Number(value)),
            },
            axisTick: { show: false },
        },

        yAxis: {
            type: "value",
            axisLine: { show: false },
            splitLine: {
                lineStyle: { color: "#1f1f1f" },
            },
            axisLabel: {
                color: "#555",
                fontSize: 11,
            },
        },

        series: [
            {
                data: values,
                type: "bar",
                barWidth: "55%",
                emphasis: {
                    focus: "series",
                },
                itemStyle: {
                    borderRadius: [6, 6, 0, 0],
                    color: (params: any) => {
                        if (params.dataIndex === peakHourIndex) {
                            return "#4ade80"; // highlight peak
                        }

                        return {
                            type: "linear",
                            x: 0,
                            y: 0,
                            x2: 0,
                            y2: 1,
                            colorStops: [
                                { offset: 0, color: "#22c55e" },
                                { offset: 1, color: "#14532d" },
                            ],
                        };
                    },
                },
                animationDuration: 800,
            },
        ],
    };

    return (
        <div className="bg-neutral-900 border border-neutral-800 rounded-xl p-5 shadow-lg shadow-black/20">

            <div className="mb-4">
                <h2 className="text-2xl font-semibold">
                    Listening by Hour
                </h2>

                <p className="text-sm text-neutral-500">
                    You listened the most at{" "}
                    <span className="text-green-400 font-medium">
                        {formatHour(Number(hours[peakHourIndex]))}
                    </span>
                </p>
            </div>

            <ReactECharts option={option} style={{ height: 320 }} />

        </div>
    );
}