"use client";

import { useEffect, useState } from "react";
import { format } from "date-fns";
import { CalendarIcon, ClockIcon, PencilSquareIcon, TrashIcon } from "@heroicons/react/24/outline";
import Popup from "../../components/Popup";
import EditSessionModal from "../../components/EditSessionModal";
import Modal from "../../components/Modal";
import { useAuthFetch } from "../../hooks/useAuthFetch";

interface WorkoutSession {
    id: number;
    user_id: number;
    date: string;
    start_time: string;
    end_time: string;
    title: string | null;
    notes: string | null;
}

export default function RecordsPage() {
    const authFetch = useAuthFetch();
    const [sessions, setSessions] = useState<WorkoutSession[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [popupMessage, setPopupMessage] = useState("");
    const [editingSessionId, setEditingSessionId] = useState<number | null>(null);
    const [deletingSessionId, setDeletingSessionId] = useState<number | null>(null);
    const [startDate, setStartDate] = useState("");
    const [endDate, setEndDate] = useState("");

    useEffect(() => {
        fetchHistory();
    }, [startDate, endDate]);

    const fetchHistory = async () => {
        setLoading(true);
        try {
            const params = new URLSearchParams();
            params.append("limit", "50");
            if (startDate) params.append("start_date", startDate);
            if (endDate) params.append("end_date", endDate);

            const res = await authFetch(`${process.env.API_URL}api/workouts/history?${params.toString()}`, {
                credentials: "include"
            });
            if (!res.ok) throw new Error("Failed to fetch history");
            const data = await res.json();
            setSessions(data);
        } catch (err) {
            setError("Could not load workout history");
            console.error(err);
        } finally {
            setLoading(false);
        }
    };

    const calculateDuration = (start: string, end: string) => {
        const s = new Date(start).getTime();
        const e = new Date(end).getTime();
        const diff = e - s;
        const mins = Math.floor(diff / 60000);
        return `${mins} min`;
    };

    const handleDeleteClick = (id: number) => {
        setDeletingSessionId(id);
    };

    const confirmDelete = async () => {
        if (!deletingSessionId) return;
        try {
            const res = await authFetch(`${process.env.API_URL}api/workouts/session/${deletingSessionId}`, {
                method: "DELETE",
                credentials: "include"
            });
            if (res.ok) {
                setSessions(prev => prev.filter(s => s.id !== deletingSessionId));
                setPopupMessage("Workout deleted successfully");
            } else {
                setPopupMessage("Failed to delete session");
            }
        } catch (err) {
            console.error(err);
            setPopupMessage("Error deleting session");
        } finally {
            setDeletingSessionId(null);
        }
    };

    if (loading) return <div className="p-8 text-center text-gray-500">Loading history...</div>;
    if (error) return <div className="p-8 text-center text-red-500">{error}</div>;

    return (
        <div className="max-w-4xl mx-auto p-6">
            <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 mb-8">
                <div>
                    <h1 className="text-3xl font-bold text-gray-900">Workout Records</h1>
                </div>

                {/* Filters */}
                <div className="bg-white p-2 rounded-xl shadow-sm border border-gray-100 flex gap-3 items-center">
                    <div className="flex items-center gap-2 px-2">
                        <CalendarIcon className="w-5 h-5 text-gray-400" />
                    </div>
                    <input
                        type="date"
                        value={startDate}
                        onChange={e => setStartDate(e.target.value)}
                        className="border border-gray-200 text-gray-900 rounded-lg px-2 py-1.5 text-sm outline-none focus:border-blue-500"
                    />
                    <span className="text-gray-400">-</span>
                    <input
                        type="date"
                        value={endDate}
                        onChange={e => setEndDate(e.target.value)}
                        className="border border-gray-200 text-gray-900 rounded-lg px-2 py-1.5 text-sm outline-none focus:border-blue-500"
                    />
                    {(startDate || endDate) && (
                        <button
                            onClick={() => { setStartDate(""); setEndDate(""); }}
                            className="text-xs text-red-500 font-bold hover:underline px-2"
                        >
                            Clear
                        </button>
                    )}
                </div>
            </div>

            {sessions.length === 0 ? (
                <div className="bg-gray-50 rounded-xl p-10 text-center text-gray-500">
                    No workouts recorded yet.
                </div>
            ) : (
                <div className="grid gap-4">
                    {sessions.map((session) => (
                        <div
                            key={session.id}
                            className="bg-white p-6 rounded-2xl shadow-sm border border-gray-100 hover:shadow-md transition-shadow flex justify-between items-center group shadow-[0_2px_10px_rgba(0,0,0,0.03)]"
                        >
                            <div className="flex-1">
                                <div className="flex items-center gap-3 mb-1">
                                    <h3 className="tex-lg font-bold text-gray-900">
                                        {session.title || "Untitled Workout"}
                                    </h3>
                                    <span className="text-xs font-medium px-2 py-1 bg-gray-100 rounded-full text-gray-600">
                                        ID: {session.id}
                                    </span>
                                </div>

                                <div className="flex items-center gap-6 text-sm text-gray-500 mt-2">
                                    <div className="flex items-center gap-1.5">
                                        <CalendarIcon className="w-4 h-4" />
                                        {format(new Date(session.start_time), "MMM d, yyyy")}
                                    </div>
                                    <div className="flex items-center gap-1.5">
                                        <ClockIcon className="w-4 h-4" />
                                        {calculateDuration(session.start_time, session.end_time)}
                                    </div>
                                </div>
                                {session.notes && (
                                    <p className="text-sm text-gray-400 mt-2 line-clamp-1 italic">
                                        "{session.notes}"
                                    </p>
                                )}
                            </div>

                            <div className="flex items-center gap-2 opacity-100 sm:opacity-0 sm:group-hover:opacity-100 transition-opacity">
                                <button
                                    onClick={() => setEditingSessionId(session.id)}
                                    className="p-2 text-gray-400 hover:text-blue-600 hover:bg-blue-50 rounded-full transition-colors"
                                    title="Edit Workout"
                                >
                                    <PencilSquareIcon className="w-5 h-5" />
                                </button>
                                <button
                                    onClick={() => handleDeleteClick(session.id)}
                                    className="p-2 text-gray-400 hover:text-red-600 hover:bg-red-50 rounded-full transition-colors"
                                    title="Delete Workout"
                                >
                                    <TrashIcon className="w-5 h-5" />
                                </button>
                            </div>
                        </div>
                    ))}
                </div>
            )}
            {editingSessionId && (
                <EditSessionModal
                    sessionId={editingSessionId}
                    onClose={() => setEditingSessionId(null)}
                    onUpdate={() => {
                        fetchHistory();
                        setPopupMessage("Workout updated!");
                    }}
                />
            )}

            <Modal isOpen={!!deletingSessionId} onClose={() => setDeletingSessionId(null)} title="Confirm Delete">
                <div className="space-y-4">
                    <p className="text-gray-600">Are you sure you want to delete this workout? This action cannot be undone.</p>
                    <div className="flex gap-3">
                        <button
                            onClick={() => setDeletingSessionId(null)}
                            className="flex-1 py-3 bg-gray-100 text-gray-700 rounded-xl font-bold hover:bg-gray-200 transition-colors"
                        >
                            Cancel
                        </button>
                        <button
                            onClick={confirmDelete}
                            className="flex-1 py-3 bg-red-600 text-white rounded-xl font-bold hover:bg-red-700 transition-colors shadow-lg shadow-red-600/20"
                        >
                            Delete Workout
                        </button>
                    </div>
                </div>
            </Modal>

            <Popup message={popupMessage} duration={1000} onClose={() => setPopupMessage("")} />
        </div>
    );
}
