"use client";

import { useState, useMemo, useEffect } from "react";
import {
    LineChart,
    Line,
    XAxis,
    YAxis,
    CartesianGrid,
    Tooltip,
    ResponsiveContainer,
} from "recharts";
import { ChevronDownIcon } from "@heroicons/react/24/outline";

/* ---------------- Types ---------------- */

type WeeklyRow = {
    week: string;
    [exercise: string]: number | string;
};

function formatDateInput(date: Date) {
    return date.toISOString().split("T")[0];
}

/* ---------------- Component ---------------- */

export default function ProgressPerformanceCard() {
    const [isOpen, setIsOpen] = useState(false);

    const [availableVariations, setAvailableVariations] = useState<{ id: number, name: string }[]>([]);
    const [selectedVariationIds, setSelectedVariationIds] = useState<number[]>([]);

    const [toDate, setToDate] = useState(new Date());
    const [fromDate, setFromDate] = useState(() => {
        const d = new Date();
        d.setDate(d.getDate() - 7 * 4); // default 8 weeks
        return d;
    });

    /* ---------- SHARED chart data ---------- */
    const [chartData, setChartData] = useState<WeeklyRow[]>([]);

    // Color Palette for dynamic assignment
    const COLORS = ["#111827", "#2563eb", "#dc2626", "#059669", "#d97706", "#7c3aed", "#e11d48", "#0ea5e9", "#22c55e", "#f59e0b"];

    useEffect(() => {
        const fetchVariations = async () => {
            try {
                const res = await fetch(`${process.env.API_URL}api/workouts/variations`, { credentials: "include" });
                if (res.ok) {
                    const data = await res.json();
                    setAvailableVariations(data);
                    if (data.length > 0) {
                        const defaults = data.slice(0, 1).map((v: any) => v.id);
                        setSelectedVariationIds(defaults);
                    }
                }
            } catch (e) {
                console.error("Failed to load variations", e);
            }
        };
        fetchVariations();
    }, []);

    useEffect(() => {
        const fetchData = async () => {
            if (selectedVariationIds.length === 0) {
                setChartData([]);
                return;
            }

            const fromStr = formatDateInput(fromDate);
            const toStr = formatDateInput(toDate);

            const dataMap: Record<string, WeeklyRow> = {};

            try {
                const responses = await Promise.all(selectedVariationIds.map(async (id) => {
                    const variation = availableVariations.find(v => v.id === id);
                    if (!variation) return { name: "Unknown", res: null };

                    const res = await fetch(`${process.env.API_URL}api/performancemetrics`, {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({
                            variation_id: id,
                            start_date: fromStr,
                            end_date: toStr
                        }),
                        credentials: 'include'
                    });
                    return { name: variation.name, res };
                }));

                let masterWeeks: string[] = [];

                for (const { name, res } of responses) {
                    if (res && res.ok) {
                        const metrics: { week: string; volume: number }[] = await res.json();

                        metrics.forEach(m => {
                            if (!dataMap[m.week]) {
                                dataMap[m.week] = { week: m.week };
                                if (!masterWeeks.includes(m.week)) masterWeeks.push(m.week);
                            }
                        });

                        metrics.forEach((metric) => {
                            dataMap[metric.week][name] = metric.volume;
                        });
                    }
                }

                const finalData = Object.values(dataMap);
                finalData.sort((a, b) => {
                    return 0;
                });

                setChartData(finalData);

            } catch (error) {
                console.error("Failed to fetch performance metrics", error);
            }
        };

        fetchData();
    }, [selectedVariationIds, fromDate, toDate, availableVariations]);

    /* ---------- Bring clicked line to front ---------- */
    function bringToFront(id: number) {
        setSelectedVariationIds((prev) => [
            ...prev.filter((e) => e !== id),
            id,
        ]);
    }

    return (
        <div className="bg-white p-8 rounded-2xl shadow-[4px_0_20px_rgba(0,0,0,0.05)] border border-gray-100 w-full relative">
            {/* Header */}
            <div className="mb-8 flex justify-between items-start">
                <div>
                    <h2 className="text-xl font-bold text-gray-900">
                        Progress & Performance
                    </h2>
                    <p className="text-sm text-gray-500 font-medium mt-1">Track your volume over time</p>
                </div>

                {/* Exercise Dropdown */}
                <div className="relative">
                    <button
                        onClick={() => setIsOpen((v) => !v)}
                        className="flex items-center gap-2 bg-gray-50 border border-gray-200 rounded-xl px-4 py-2 text-sm font-bold text-gray-900 focus:outline-none focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500 transition-all hover:bg-gray-100"
                    >
                        {selectedVariationIds.length > 0 ? `${selectedVariationIds.length} Selected` : "Select exercises"}
                        <ChevronDownIcon className="w-4 h-4 text-gray-500" />
                    </button>

                    {isOpen && (
                        <div className="absolute right-0 z-20 mt-2 w-56 max-h-64 overflow-y-auto rounded-xl border border-gray-100 bg-white shadow-xl animate-fadeIn p-2 space-y-1">
                            {availableVariations.map((v) => {
                                const checked = selectedVariationIds.includes(v.id);

                                return (
                                    <label
                                        key={v.id}
                                        className="flex cursor-pointer items-center gap-3 px-3 py-2 text-sm font-semibold text-gray-700 hover:bg-gray-50 rounded-lg transition-colors select-none"
                                    >
                                        <div className={`w-5 h-5 rounded-md border flex items-center justify-center transition-all ${checked ? 'bg-blue-600 border-blue-600' : 'border-gray-300 bg-white'}`}>
                                            {checked && <svg className="w-3.5 h-3.5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" /></svg>}
                                        </div>
                                        <input
                                            type="checkbox"
                                            checked={checked}
                                            onChange={() => {
                                                setSelectedVariationIds((prev) => {
                                                    if (checked && prev.length === 1) return prev;
                                                    return checked ? prev.filter(id => id !== v.id) : [...prev, v.id];
                                                });
                                            }}
                                            className="hidden"
                                        />
                                        {v.name}
                                    </label>
                                );
                            })}
                        </div>
                    )}
                </div>
            </div>

            {/* Date Range */}
            <div className="mb-6 flex gap-4">
                <div className="flex flex-col gap-1">
                    <label className="text-xs font-bold text-gray-500 uppercase tracking-wide">From</label>
                    <input
                        type="date"
                        value={formatDateInput(fromDate)}
                        max={formatDateInput(toDate)}
                        onChange={(e) => setFromDate(new Date(e.target.value))}
                        className="bg-gray-50 border border-gray-200 rounded-xl px-4 py-2 text-sm font-bold text-gray-900 focus:outline-none focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500 transition-all cursor-pointer"
                    />
                </div>
                <div className="flex flex-col gap-1">
                    <label className="text-xs font-bold text-gray-500 uppercase tracking-wide">To</label>
                    <input
                        type="date"
                        value={formatDateInput(toDate)}
                        max={formatDateInput(new Date())}
                        onChange={(e) => setToDate(new Date(e.target.value))}
                        className="bg-gray-50 border border-gray-200 rounded-xl px-4 py-2 text-sm font-bold text-gray-900 focus:outline-none focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500 transition-all cursor-pointer"
                    />
                </div>
            </div>

            {/* Chart */}
            {selectedVariationIds.length === 0 ? (
                <div className="flex h-[350px] items-center justify-center text-gray-400 bg-gray-50/50 rounded-xl border border-dashed border-gray-200">
                    <div className="text-center">
                        <p className="font-semibold">Select one or more exercises</p>
                        <p className="text-sm">to compare your progress</p>
                    </div>
                </div>
            ) : (
                <div className="h-[350px] w-full mt-4">
                    <ResponsiveContainer width="100%" height="100%">
                        <LineChart data={chartData} margin={{ top: 10, right: 30, left: 0, bottom: 0 }}>
                            <CartesianGrid strokeDasharray="3 3" stroke="#f3f4f6" vertical={false} />
                            <XAxis
                                dataKey="week"
                                stroke="#9ca3af"
                                tick={{ fontSize: 12, fill: '#6b7280' }}
                                tickLine={false}
                                axisLine={false}
                                dy={10}
                                tickFormatter={(value) => {
                                    const parts = value.split('-');
                                    if (parts.length >= 2) {
                                        const monthsPart = parts[0];
                                        const weekPart = parts[1];

                                        if (monthsPart.includes('/')) {
                                            const [m1, m2] = monthsPart.split('/');
                                            const date1 = new Date(); date1.setMonth(parseInt(m1) - 1);
                                            const name1 = date1.toLocaleString('default', { month: 'short' });

                                            const date2 = new Date(); date2.setMonth(parseInt(m2) - 1);
                                            const name2 = date2.toLocaleString('default', { month: 'short' });

                                            return `${name1}/${name2}-${weekPart.toUpperCase()}`;
                                        } else {
                                            const monthIndex = parseInt(monthsPart, 10) - 1;
                                            const date = new Date();
                                            date.setMonth(monthIndex);
                                            const monthName = date.toLocaleString('default', { month: 'short' });
                                            return `${monthName}-${weekPart.toUpperCase()}`;
                                        }
                                    }
                                    return value;
                                }}
                            />
                            <YAxis
                                stroke="#9ca3af"
                                tick={{ fontSize: 12, fill: '#6b7280' }}
                                tickLine={false}
                                axisLine={false}
                                dx={-10}
                            />
                            <Tooltip
                                contentStyle={{ borderRadius: '12px', border: 'none', boxShadow: '0 4px 20px rgba(0,0,0,0.1)' }}
                            />

                            {selectedVariationIds.map((id, index) => {
                                const variation = availableVariations.find(v => v.id === id);
                                const name = variation ? variation.name : "Unknown";
                                const isActive = index === selectedVariationIds.length - 1;
                                const color = COLORS[index % COLORS.length];

                                return (
                                    <Line
                                        key={id}
                                        type="monotone"
                                        dataKey={name}
                                        stroke={color}
                                        strokeWidth={isActive ? 3 : 2}
                                        opacity={isActive ? 1 : 0.6}
                                        dot={isActive ? { r: 4, strokeWidth: 2 } : false}
                                        activeDot={isActive ? { r: 6, strokeWidth: 2 } : false}
                                        isAnimationActive={true}
                                        animationDuration={500}
                                        onMouseDown={() => bringToFront(id)}
                                        style={{ cursor: "pointer", pointerEvents: "all" }}
                                    />
                                );
                            })}
                        </LineChart>
                    </ResponsiveContainer>
                </div>
            )}

        </div>
    );
}
