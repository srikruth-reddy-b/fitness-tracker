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

type ExerciseName = "Bench Press" | "Squat" | "Deadlift";

const EXERCISE_COLORS: Record<ExerciseName, string> = {
    "Bench Press": "#111827", // near-black
    Squat: "#2563eb",        // blue
    Deadlift: "#dc2626",     // red
};

function formatDateInput(date: Date) {
    return date.toISOString().split("T")[0];
}

/* ---------------- Base Volumes ---------------- */

const EXERCISES: Record<ExerciseName, number> = {
    "Bench Press": 4200,
    Squat: 6000,
    Deadlift: 5500,
};

/* ---------------- Component ---------------- */

export default function ProgressPerformanceCard() {
    const [isOpen, setIsOpen] = useState(false);
    /* Order matters: last = active / foreground */
    const [selectedExercises, setSelectedExercises] =
        useState<ExerciseName[]>([]);

    const [toDate, setToDate] = useState(new Date());
    const [fromDate, setFromDate] = useState(() => {
        const d = new Date();
        d.setDate(d.getDate() - 7 * 7); // default 8 weeks
        return d;
    });

    /* ---------- SHARED chart data ---------- */
    const [chartData, setChartData] = useState<WeeklyRow[]>([]);

    const EXERCISE_IDS: Record<ExerciseName, number> = {
        "Bench Press": 1,
        "Squat": 2,
        "Deadlift": 3,
    };

    useEffect(() => {
        const fetchData = async () => {
            if (selectedExercises.length === 0) {
                setChartData([]);
                return;
            }

            const fromStr = formatDateInput(fromDate);
            const toStr = formatDateInput(toDate);

            const dataMap: Record<string, WeeklyRow> = {};
            // We'll collect all seen weeks to ensure correct order/union
            // Since backend returns sorted full ranges, the first response determines the structure ideally.
            // But to be safe with Promises, we can collect all.

            try {
                // Fetch data for all selected exercises in parallel
                const responses = await Promise.all(selectedExercises.map(async (ex) => {
                    const id = EXERCISE_IDS[ex];
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
                    return { ex, res };
                }));

                let masterWeeks: string[] = [];

                for (const { ex, res } of responses) {
                    if (res.ok) {
                        const metrics: { week: string; volume: number }[] = await res.json();

                        // If this is the first successful response, set the master weeks list
                        // (Backend guarantees full range sorted)
                        if (masterWeeks.length === 0 && metrics.length > 0) {
                            masterWeeks = metrics.map(m => m.week);
                            masterWeeks.forEach(w => {
                                dataMap[w] = { week: w };
                            });
                        }

                        metrics.forEach((metric) => {
                            // Ensure row exists (in case requests had slightly different ranges? Shouldn't happen)
                            if (!dataMap[metric.week]) {
                                dataMap[metric.week] = { week: metric.week };
                                // If we found a new week not in master (unlikely with same dates), add to master?
                                // For now assume backend consistency.
                            }
                            dataMap[metric.week][ex] = metric.volume;
                        });
                    }
                }

                // Use masterWeeks to ensure consistent sorted order
                const finalData = masterWeeks.map(w => dataMap[w]);
                setChartData(finalData);

            } catch (error) {
                console.error("Failed to fetch performance metrics", error);
            }
        };

        fetchData();
    }, [selectedExercises, fromDate, toDate]);

    /* ---------- Bring clicked line to front ---------- */
    function bringToFront(ex: ExerciseName) {
        setSelectedExercises((prev) => [
            ...prev.filter((e) => e !== ex),
            ex,
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
                        {selectedExercises.length > 0 ? `${selectedExercises.length} Selected` : "Select exercises"}
                        <ChevronDownIcon className="w-4 h-4 text-gray-500" />
                    </button>

                    {isOpen && (
                        <div className="absolute right-0 z-20 mt-2 w-56 rounded-xl border border-gray-100 bg-white shadow-xl animate-fadeIn p-2 space-y-1">
                            {Object.keys(EXERCISES).map((ex) => {
                                const exercise = ex as ExerciseName;
                                const checked = selectedExercises.includes(exercise);

                                return (
                                    <label
                                        key={exercise}
                                        className="flex cursor-pointer items-center gap-3 px-3 py-2 text-sm font-semibold text-gray-700 hover:bg-gray-50 rounded-lg transition-colors select-none"
                                    >
                                        <div className={`w-5 h-5 rounded-md border flex items-center justify-center transition-all ${checked ? 'bg-blue-600 border-blue-600' : 'border-gray-300 bg-white'}`}>
                                            {checked && <svg className="w-3.5 h-3.5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" /></svg>}
                                        </div>
                                        <input
                                            type="checkbox"
                                            checked={checked}
                                            onChange={() => {
                                                setSelectedExercises((prev) => {
                                                    // Prevent removing last exercise
                                                    if (checked && prev.length === 1) return prev;

                                                    if (checked) {
                                                        // remove
                                                        return prev.filter((e) => e !== exercise);
                                                    } else {
                                                        // add (to top / foreground)
                                                        return [...prev, exercise];
                                                    }
                                                });
                                            }}
                                            className="hidden"
                                        />
                                        {exercise}
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
            {selectedExercises.length === 0 ? (
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

                            {selectedExercises.map((ex, index) => {
                                const isActive =
                                    index === selectedExercises.length - 1;

                                return (
                                    <Line
                                        key={ex}
                                        type="monotone"
                                        dataKey={ex}
                                        stroke={EXERCISE_COLORS[ex]}
                                        strokeWidth={isActive ? 3 : 2}
                                        opacity={isActive ? 1 : 0.6}
                                        dot={isActive ? { r: 4, strokeWidth: 2 } : false}
                                        activeDot={isActive ? { r: 6, strokeWidth: 2 } : false}
                                        isAnimationActive={true}
                                        animationDuration={500}
                                        onMouseDown={() => bringToFront(ex)}
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
