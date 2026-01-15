"use client";

import { useMemo, useState, useEffect } from "react";

function formatDateInput(date: Date) {
  return date.toISOString().split("T")[0];
}

function startOfWeek(date: Date) {
  const d = new Date(date);
  const day = d.getDay();
  const diff = d.getDate() - day + (day === 0 ? -6 : 1);
  d.setDate(diff);
  d.setHours(0, 0, 0, 0);
  return d;
}

function weeksBetween(from: Date, to: Date) {
  const diffMs = to.getTime() - from.getTime();
  return Math.max(1, Math.ceil(diffMs / (7 * 24 * 60 * 60 * 1000)));
}

function getStatus(avg: number) {
  if (avg < OPTIMAL_MIN)
    return { label: "Maintenance", color: "bg-gray-300" };
  if (avg <= OPTIMAL_MAX)
    return { label: "Optimal", color: "bg-green-500" };
  if (avg < HIGH_RISK)
    return { label: "High", color: "bg-orange-400" };
  return { label: "High Fatigue", color: "bg-red-500" };
}

/* ---------------- Types ---------------- */

type MuscleGroup = string;

type MuscleTotal = {
  muscle: MuscleGroup;
  totalSets: number;
};
/* ---------------- Thresholds ---------------- */

const OPTIMAL_MIN = 10;
const OPTIMAL_MAX = 20;
const HIGH_RISK = 25;
const MAX_SCALE = 30;

/* ---------------- Component ---------------- */

export default function VolumeBars() {
  /* ---------------- State ---------------- */
  const [toDate, setToDate] = useState(new Date());
  const [fromDate, setFromDate] = useState(() => {
    const d = new Date();
    d.setDate(d.getDate() - 7);
    return d;
  });
  const [muscleTotals, setMuscleTotals] = useState<MuscleTotal[]>([]);

  const weeks = useMemo(
    () => weeksBetween(fromDate, toDate),
    [fromDate, toDate]
  );

  /* ---------------- Fetch & Calc ---------------- */
  useEffect(() => {
    const fetchMetadataAndData = async () => {
      try {
        const mgResponse = await fetch(`${process.env.API_URL}api/workouts/muscle_groups`, { credentials: "include" });
        if (!mgResponse.ok) return;
        const muscleGroups: { id: number, name: string }[] = await mgResponse.json();

        const fromStr = formatDateInput(fromDate);
        const toStr = formatDateInput(toDate);
        const ids = muscleGroups.map(mg => mg.id);

        const response = await fetch(`${process.env.API_URL}api/mslegrpsumm`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            muscle_group_ids: ids,
            start_date: fromStr,
            end_date: toStr
          }),
          credentials: 'include'
        });

        if (response.ok) {
          const data: { muscle_group_id: number; total_sets: number }[] = await response.json();

          const newTotals: MuscleTotal[] = muscleGroups.map((mg) => {
            const found = data.find(d => d.muscle_group_id === mg.id);
            return {
              muscle: mg.name, 
              totalSets: found ? found.total_sets : 0
            };
          });

          setMuscleTotals(newTotals);
        }
      } catch (error) {
        console.error("Failed to fetch volume summary", error);
      }
    };

    fetchMetadataAndData();
  }, [fromDate, toDate]);

  const data = useMemo(() => {
    return muscleTotals.map((m) => {
      const avg = m.totalSets / weeks;
      return {
        muscle: m.muscle,
        avgSets: Number(avg.toFixed(1)),
      };
    });
  }, [weeks, muscleTotals]);

  return (
    <div className="bg-white p-8 rounded-2xl shadow-[4px_0_20px_rgba(0,0,0,0.05)] border border-gray-100 w-full relative">
      <div className="mb-8 flex flex-col gap-4 sm:flex-row sm:items-start sm:justify-between">
        <div>
          <h2 className="text-xl font-bold text-gray-900">
            Volume Bars
          </h2>
          <p className="text-sm text-gray-500 font-medium mt-1">
            Weekly set volume per muscle group
          </p>
        </div>

        <div className="flex gap-4">
          <div className="flex flex-col gap-1">
            <label className="text-xs font-bold text-gray-500 uppercase tracking-wide">From</label>
            <input
              type="date"
              value={formatDateInput(fromDate)}
              max={formatDateInput(toDate)}
              onChange={(e) =>
                setFromDate(new Date(e.target.value))
              }
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
      </div>

      {/* Bars */}
      <div className="space-y-6">
        {data.map(({ muscle, avgSets }) => {
          const status = getStatus(avgSets);
          const width = Math.min(
            (avgSets / MAX_SCALE) * 100,
            100
          );

          return (
            <div key={muscle}>
              <div className="mb-2 flex justify-between text-sm items-center">
                <span className="font-bold text-gray-800">
                  {muscle}
                </span>
                <span className="text-gray-500 font-medium bg-gray-50 px-2 py-1 rounded-lg text-xs">
                  <span className="text-gray-900 font-bold">{avgSets}</span> / wk · {status.label}
                </span>
              </div>

              <div className="relative h-3 w-full rounded-full bg-gray-100 overflow-hidden">
                {[OPTIMAL_MIN, OPTIMAL_MAX, HIGH_RISK].map(
                  (t) => (
                    <div
                      key={t}
                      className="absolute top-0 h-full w-[2px] bg-white z-10"
                      style={{
                        left: `${(t / MAX_SCALE) * 100}%`,
                      }}
                    />
                  )
                )}

                <div
                  className={`h-full rounded-full ${status.color} transition-all duration-1000 ease-out`}
                  style={{ width: `${width}%` }}
                />
              </div>
            </div>
          );
        })}
      </div>

      <div className="mt-8 pt-6 border-t border-gray-100 flex flex-wrap gap-x-6 gap-y-2 text-xs font-semibold text-gray-500">
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 rounded-full bg-gray-300"></div>
          <span>&lt;10 Maintenance</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 rounded-full bg-green-500"></div>
          <span className="text-gray-900">10–20 Optimal</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 rounded-full bg-orange-400"></div>
          <span>20–25 High</span>
        </div>
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 rounded-full bg-red-500"></div>
          <span>25+ Fatigue Risk</span>
        </div>
      </div>
    </div>
  );
}

