import { useState, useEffect } from "react";
import Modal from "./Modal";
import { useAuthFetch } from "../hooks/useAuthFetch";

/* ---------------- Types ---------------- */
interface WorkoutSession {
    id: number;
    user_id: number;
    title?: string;
    notes?: string;
    date: string; // NaiveDate string
    start_time: string; // NaiveDateTime string
    end_time: string;
}

interface WorkoutSet {
    id: number;
    variation_id: number;
    weight: number;
    reps: number;
}

interface CardioLog {
    id: number;
    cardio_exercise_id: number;
    duration_minutes: number;
}

interface Variation {
    id: number;
    name: string;
}

interface CardioExercise {
    id: number;
    name: string;
}

interface SessionDetails {
    sets: WorkoutSet[];
    cardio_logs: CardioLog[];
}

interface DayDetailsModalProps {
    isOpen: boolean;
    onClose: () => void;
    date: Date | null;
}

export default function DayDetailsModal({ isOpen, onClose, date }: DayDetailsModalProps) {
    const authFetch = useAuthFetch();
    const [sessions, setSessions] = useState<WorkoutSession[]>([]);
    const [sessionDetails, setSessionDetails] = useState<Record<number, SessionDetails>>({});

    // Metadata Maps
    const [variationMap, setVariationMap] = useState<Record<number, string>>({});
    const [cardioMap, setCardioMap] = useState<Record<number, string>>({});

    const [isLoading, setIsLoading] = useState(false);

    useEffect(() => {
        if (isOpen && date) {
            fetchMetaData();
            fetchSessions();
        }
    }, [isOpen, date]);

    const fetchMetaData = async () => {
        try {
            // Fetch Variations
            const vRes = await authFetch(`${process.env.API_URL}api/workouts/variations`, { credentials: 'include' });
            if (vRes.ok) {
                const data: Variation[] = await vRes.json();
                const map: Record<number, string> = {};
                data.forEach(v => map[v.id] = v.name);
                setVariationMap(map);
            }

            // Fetch Cardio Exercises
            const cRes = await authFetch(`${process.env.API_URL}api/workouts/cardio_exercises`, { credentials: 'include' });
            if (cRes.ok) {
                const data: CardioExercise[] = await cRes.json();
                const map: Record<number, string> = {};
                data.forEach(c => map[c.id] = c.name);
                setCardioMap(map);
            }
        } catch (e) {
            console.error("Failed to fetch metadata", e);
        }
    };

    const fetchSessions = async () => {
        if (!date) return;
        setIsLoading(true);
        try {
            // Format YYYY-MM-DD
            // JS toISOString returns UTC, we might want local date part?
            // date is from Heatmap which constructs pure dates (new Date(y, m, d)).
            // Be careful with timezones. 
            // Heatmap uses: new Date(year, month, i+1). This is local 00:00:00.
            // toISOString() might shift it if we are behind UTC.

            // Safe manual format:
            const y = date.getFullYear();
            const m = String(date.getMonth() + 1).padStart(2, '0');
            const d = String(date.getDate()).padStart(2, '0');
            const dateStr = `${y}-${m}-${d}`;

            const res = await authFetch(`${process.env.API_URL}api/workouts/history?start_date=${dateStr}&end_date=${dateStr}&limit=50`, {
                credentials: 'include'
            });

            if (res.ok) {
                const sessionData: WorkoutSession[] = await res.json();
                setSessions(sessionData);

                // Fetch details for each session
                const detailsMap: Record<number, SessionDetails> = {};
                await Promise.all(sessionData.map(async (s) => {
                    try {
                        const detailRes = await authFetch(`${process.env.API_URL}api/workouts/session/${s.id}`, { credentials: 'include' });
                        if (detailRes.ok) {
                            const detailData = await detailRes.json();
                            detailsMap[s.id] = {
                                sets: detailData.sets || [],
                                cardio_logs: detailData.cardio_logs || []
                            };
                        }
                    } catch (err) {
                        console.error(`Failed to fetch details for session ${s.id}`, err);
                    }
                }));
                setSessionDetails(detailsMap);
            } else {
                console.error("Failed to fetch sessions");
            }
        } catch (e) {
            console.error(e);
        } finally {
            setIsLoading(false);
        }
    };

    // Helper to format time
    const formatTime = (ts: string) => {
        if (!ts) return "";
        const d = new Date(ts);
        return d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', hour12: false });
    };

    // Calculate duration
    const getDuration = (start: string, end: string) => {
        const s = new Date(start).getTime();
        const e = new Date(end).getTime();
        const diffM = Math.round((e - s) / 60000);
        return `${diffM}m`;
    };

    return (
        <Modal isOpen={isOpen} onClose={onClose} title={date ? date.toLocaleDateString(undefined, { weekday: 'long', month: 'long', day: 'numeric' }) : "Details"} maxWidth="max-w-xl">
            <div className="space-y-4 max-h-[70vh] overflow-y-auto pr-2 custom-scrollbar">
                {isLoading ? (
                    <div className="flex flex-col gap-4">
                        {[1, 2].map(i => (
                            <div key={i} className="h-32 bg-gray-50 rounded-xl animate-pulse" />
                        ))}
                    </div>
                ) : sessions.length === 0 ? (
                    <div className="text-center py-12 text-gray-500">
                        <p className="text-lg font-medium">Rest Day</p>
                        <p className="text-sm">No workouts logged on this date.</p>
                    </div>
                ) : (
                    sessions.map(session => (
                        <div key={session.id} className="bg-white border border-gray-100 rounded-xl p-5 shadow-[0_2px_10px_rgba(0,0,0,0.03)] hover:shadow-md transition-all">
                            <div className="flex justify-between items-start mb-3">
                                <div>
                                    <h4 className="font-bold text-lg text-gray-900">{session.title || "Untitled Workout"}</h4>
                                    <p className="text-xs text-gray-500 font-medium uppercase tracking-wide">
                                        {formatTime(session.start_time)} - {formatTime(session.end_time)} • {getDuration(session.start_time, session.end_time)}
                                    </p>
                                </div>
                                <div className="bg-blue-50 text-blue-700 text-xs font-bold px-2 py-1 rounded-full">
                                    Completed
                                </div>
                            </div>

                            {session.notes && (
                                <p className="text-sm text-gray-600 bg-gray-50 p-3 rounded-lg italic border border-gray-100 mb-4">
                                    "{session.notes}"
                                </p>
                            )}

                            {/* Sets Table */}
                            {sessionDetails[session.id]?.sets?.length > 0 && (
                                <div className="mt-4 border-t border-gray-100 pt-3">
                                    <h5 className="text-xs font-bold text-gray-400 uppercase tracking-widest mb-2">Strength</h5>
                                    <div className="space-y-2">
                                        {sessionDetails[session.id].sets.map((set, idx) => (
                                            <div key={idx} className="flex justify-between items-center text-sm p-2 hover:bg-gray-50 rounded-md transition-colors">
                                                <span className="font-medium text-gray-700">
                                                    {variationMap[set.variation_id] || `Exercise #${set.variation_id}`}
                                                </span>
                                                <span className="font-mono text-gray-600 text-xs bg-gray-100 px-2 py-1 rounded">
                                                    {set.weight}kg × {set.reps}
                                                </span>
                                            </div>
                                        ))}
                                    </div>
                                </div>
                            )}

                            {/* Cardio Section */}
                            {sessionDetails[session.id]?.cardio_logs?.length > 0 && (
                                <div className="mt-4 border-t border-gray-100 pt-3">
                                    <h5 className="text-xs font-bold text-gray-400 uppercase tracking-widest mb-2">Cardio</h5>
                                    <div className="space-y-2">
                                        {sessionDetails[session.id].cardio_logs.map((log, idx) => (
                                            <div key={idx} className="flex justify-between items-center text-sm p-2 hover:bg-gray-50 rounded-md transition-colors">
                                                <span className="font-medium text-gray-700">
                                                    {cardioMap[log.cardio_exercise_id] || `Cardio #${log.cardio_exercise_id}`}
                                                </span>
                                                <span className="font-mono text-gray-600 text-xs bg-blue-50 text-blue-600 px-2 py-1 rounded">
                                                    {log.duration_minutes} mins
                                                </span>
                                            </div>
                                        ))}
                                    </div>
                                </div>
                            )}
                        </div>
                    ))
                )}
            </div>
        </Modal>
    );
}
