"use client";

import { useState, useEffect } from "react";
import { ChevronDownIcon } from "@heroicons/react/24/outline";

/* ---------------- Types ---------------- */

type HeatLevel = 0 | 1 | 2 | 3;
type DayData = { date: Date; level: HeatLevel };

/* ---------------- Constants ---------------- */

const WEEK_DAYS = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];
const MONTHS = [
  "January", "February", "March", "April", "May", "June",
  "July", "August", "September", "October", "November", "December",
];

const FULL_WIDTH = 550;
const FULL_HEIGHT = 500;
const PREVIEW_COLUMNS = 3;

/* ---------------- Helpers ---------------- */

async function getMonthData(year: number, month: number) {
  const firstDay = new Date(year, month, 1);
  const lastDay = new Date(year, month + 1, 0);

  const startWeekday = firstDay.getDay();
  const daysInMonth = lastDay.getDate();

  // Initialize with empty/zero data
  const days: DayData[] = Array.from({ length: daysInMonth }, (_, i) => ({
    date: new Date(year, month, i + 1),
    level: 0 as HeatLevel,
  }));

  try {
    // API uses 1-based month (1-12), JS uses 0-based (0-11)
    const apiMonth = month + 1;

    const response = await fetch(`${process.env.API_URL}api/monthlylevels`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ year, month: apiMonth }),
      credentials: 'include'
    });

    if (response.ok) {
    //   const sessions = await response.json();
      const workoutLevels = await response.json();

      // Create a map of date string to level for easier lookup
      // or simply iterate since we know the structure
      // Backend returns { date: "YYYY-MM-DD", level: number }
      const levelsMap: Record<string, number> = {};

      workoutLevels.forEach((item: any) => {
        if (item.date && typeof item.level === 'number') {
          // item.date is "YYYY-MM-DD"
          levelsMap[item.date] = item.level;
        }
      });

      // Update days with levels from map
      days.forEach(day => {
        const y = day.date.getFullYear();
        const m = String(day.date.getMonth() + 1).padStart(2, '0');
        const d = String(day.date.getDate()).padStart(2, '0');
        const dateKey = `${y}-${m}-${d}`;

        if (levelsMap[dateKey] !== undefined) {
          day.level = levelsMap[dateKey] as HeatLevel;
        }
      });
    }
  } catch (err) {
    console.error("Failed to fetch heatmap data", err);
  }

  return { startWeekday, days };
}

/* ---------------- MonthGrid ---------------- */

type MonthGridProps = {
  year: number;
  month: number;
  clip?: "left" | "right";
  isActive?: boolean;
  disabled?: boolean;
  onClick?: () => void;
  className?: string;
};

function MonthGrid({
  year,
  month,
  clip,
  isActive = false,
  disabled = false,
  onClick,
  className = "",
}: MonthGridProps) {
  const [data, setData] = useState<{ startWeekday: number; days: DayData[] } | null>(null);

  useEffect(() => {
    let isMounted = true;

    const fetchData = async () => {
      const result = await getMonthData(year, month);
      if (isMounted) setData(result);
    };

    fetchData();

    return () => { isMounted = false; };
  }, [year, month]);

  // Updated to Blue Theme
  const colorMap: Record<HeatLevel, string> = {
    0: "bg-gray-100",
    1: "bg-gradient-to-br from-red-100 to-red-300",
    2: "bg-gradient-to-br from-red-300 to-red-500",
    3: "bg-gradient-to-br from-red-500 to-red-700",
  };

  const columnWidth = FULL_WIDTH / 7;
  const visibleWidth = clip ? PREVIEW_COLUMNS * columnWidth : FULL_WIDTH;
  const offsetLeft = clip === "left" ? FULL_WIDTH - visibleWidth : 0;

  const titleAlignment =
    clip === "right"
      ? "text-left pl-2"
      : clip === "left"
        ? "text-right pr-2"
        : "text-center";

  // Default skeleton/loading state if data not yet loaded
  const startWeekday = data?.startWeekday || 0;
  const days = data?.days || [];

  return (
    <div
      onClick={disabled ? undefined : onClick}
      className={`
        relative transform-gpu transition-all duration-500 ease-out
        ${isActive ? "scale-100 opacity-100 z-10" : "scale-[0.96] opacity-60"}
        ${disabled ? "cursor-not-allowed opacity-30" : "cursor-pointer hover:opacity-80"}
        ${className}
      `}
      style={{ width: FULL_WIDTH, height: FULL_HEIGHT }}
    >
      {/* Month title */}
      <div
        className={`
          text-sm mb-4 ${titleAlignment}
          ${!clip ? "font-bold text-gray-900 text-lg" : "font-semibold text-gray-400"}
        `}
      >
        {MONTHS[month]} {year}
      </div>

      {/* Weekday labels only for current month */}
      {!clip && (
        <div
          className="grid text-xs font-bold text-gray-400 mb-2"
          style={{ gridTemplateColumns: "repeat(7, 1fr)", width: FULL_WIDTH }}
        >
          {WEEK_DAYS.map((d) => (
            <div key={d} className="text-center">
              {d}
            </div>
          ))}
        </div>
      )}

      {/* Calendar viewport */}
      <div
        className="relative overflow-hidden rounded-xl"
        style={{ width: visibleWidth, marginLeft: offsetLeft }}
      >
        <div className="grid grid-cols-7 gap-[2px]">
          {/* Empty cells for start padding */}
          {Array.from({ length: startWeekday }).map((_, i) => (
            <div
              key={`e-${i}`}
              className="aspect-square bg-gray-50/50 rounded-md"
            />
          ))}

          {/* Days */}
          {days.length > 0 ? (
            days.map((day) => (
              <div
                key={day.date.toISOString()}
                className={`aspect-square rounded-md ${colorMap[day.level]} flex items-center justify-center transition-all hover:scale-105 hover:shadow-sm`}
              >
                <span
                  className={
                    clip
                      ? "text-[8px] text-gray-400"
                      : `text-[10px] font-bold ${day.level > 1 ? 'text-white' : 'text-gray-700'}`
                  }
                >
                  {day.date.getDate()}
                </span>
              </div>
            ))
          ) : (
            // Simple Loading Skeleton if needed, or just show empty dates until loaded
            // For now we initialized 'days' in getMonthData synchronously partly so we might want to return that structure immediately? 
            // Actually I made getMonthData async entirely.
            // So 'days' is empty array initially.
            // Let's render empty placeholders if loading
            Array.from({ length: new Date(year, month + 1, 0).getDate() }).map((_, i) => (
              <div key={i} className="aspect-square bg-gray-50 rounded-md animate-pulse"></div>
            ))
          )}
        </div>
      </div>
    </div>
  );
}

/* ---------------- HeatmapCard ---------------- */

export default function HeatmapCard() {
  const today = new Date();
  const currentYear = today.getFullYear();
  const currentMonth = today.getMonth();

  const [year, setYear] = useState(currentYear);
  const [month, setMonth] = useState(currentMonth);
  const [direction, setDirection] = useState<"prev" | "next" | null>(null);

  const isFutureMonth = (y: number, m: number) =>
    y > currentYear || (y === currentYear && m > currentMonth);

  const prev =
    month === 0 ? { y: year - 1, m: 11 } : { y: year, m: month - 1 };
  const next =
    month === 11 ? { y: year + 1, m: 0 } : { y: year, m: month + 1 };

  const nextIsFuture = isFutureMonth(next.y, next.m);

  return (
    <div className="bg-white p-8 rounded-2xl shadow-[4px_0_20px_rgba(0,0,0,0.05)] border border-gray-100 overflow-hidden w-full">
      {/* Header */}
      <div className="flex items-center justify-between mb-8">
        <div>
          <h3 className="text-xl font-bold text-gray-900">
            Activity Heatmap
          </h3>
          <p className="text-sm text-gray-500 font-medium mt-1">Your consistency streak</p>
        </div>

        {/* Month / Year selectors */}
        <div className="flex gap-3">
          <div className="relative">
            <select
              value={month}
              onChange={(e) => {
                const m = Number(e.target.value);
                if (isFutureMonth(year, m)) return;
                setDirection(null);
                setMonth(m);
              }}
              className="appearance-none bg-gray-50 border border-gray-200 rounded-xl px-4 py-2 pr-10 text-sm font-bold text-gray-900 focus:outline-none focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500 transition-all cursor-pointer"
            >
              {MONTHS.map((m, i) => (
                <option key={m} value={i} disabled={isFutureMonth(year, i)}>
                  {m}
                </option>
              ))}
            </select>
            <ChevronDownIcon className="w-4 h-4 text-gray-500 absolute right-3 top-1/2 -translate-y-1/2 pointer-events-none" />
          </div>

          <div className="relative">
            <select
              value={year}
              onChange={(e) => {
                const y = Number(e.target.value);
                if (isFutureMonth(y, month)) return;
                setDirection(null);
                setYear(y);
              }}
              className="appearance-none bg-gray-50 border border-gray-200 rounded-xl px-4 py-2 pr-10 text-sm font-bold text-gray-900 focus:outline-none focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500 transition-all cursor-pointer"
            >
              {Array.from({ length: 6 }, (_, i) => currentYear - i).map((y) => (
                <option key={y} value={y} disabled={isFutureMonth(y, month)}>
                  {y}
                </option>
              ))}
            </select>
            <ChevronDownIcon className="w-4 h-4 text-gray-500 absolute right-3 top-1/2 -translate-y-1/2 pointer-events-none" />
          </div>
        </div>
      </div>

      {/* Animated months */}
      <div
        className={`
          flex justify-center items-center py-4
        `}
      >
        <MonthGrid
          year={prev.y}
          month={prev.m}
          clip="right"
          className="-mr-[250px]"
          onClick={() => {
            setDirection("prev");
            setYear(prev.y);
            setMonth(prev.m);
          }}
        />

        <MonthGrid year={year} month={month} isActive />

        <MonthGrid
          year={next.y}
          month={next.m}
          clip="left"
          disabled={nextIsFuture}
          className="-ml-[250px]"
          onClick={
            nextIsFuture
              ? undefined
              : () => {
                setDirection("next");
                setYear(next.y);
                setMonth(next.m);
              }
          }
        />
      </div>
    </div>
  );
}

