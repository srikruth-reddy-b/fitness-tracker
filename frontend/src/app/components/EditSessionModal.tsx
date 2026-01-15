"use client";

import { useEffect, useState } from "react";
import Modal from "./Modal";
import { TrashIcon } from "@heroicons/react/24/outline";
import { useAuthFetch } from "../hooks/useAuthFetch";

interface EditSessionModalProps {
    sessionId: number | null;
    onClose: () => void;
    onUpdate: () => void;
}

interface SetData {
    id: number;
    variation_id: number;
    weight: number;
    reps: number;
    // We might need variation name to display? 
    // The details response includes 'sets' which are WorkoutSet structs.
    // WorkoutSet struct doesn't have the Name.
    // Issue: The backend 'get_session_details' returns raw table structs.
    // The Sets table has variation_id, but not name.
    // The Frontend needs to display what exercise it is.
    // Current backend `get_session_details` returns: (Session, Vec<Set>, Vec<Cardio>).
    // It does NOT return joined Variation names.
    // This is a UI probem. I can only display "Set #ID" or "Variation #ID" unless I fetch metadata or update backend.
    // Given previous steps, I should probably check if I can get names. 
    // For now, I'll stick to displaying "Variation ID: X" or see if I can fetch all variations to map them?
    // Actually, `records/page.tsx` didn't display sets, just summary.
    // `log/page.tsx` has `INITIAL_VARIATIONS`.
    // Maybe I can fetch all variations and map? Or update backend to return names.
    // Updating backend is cleaner for "Session Details".
    // But for SPEED, I will check if `records/page.tsx` fetches names? No, it just shows title.
    // The user just asked for "edit feature".
    // If I show "Variation ID: 5", user will be confused.
    // I SHOULD update the backend to ensure names are returned, OR fetch variations map on mount.
    // The backend `get_session_details` is in `workouts.rs`.
    // Let's implement the Modal first, and if names are missing, I'll update the backend.
    // Wait, the user previously updated `get_sets_for_musclegroups` to join.
    // I should probably update `get_session_details` to return a specialized struct with names.
    // BUT the user didn't ask me to refactor backend for that yet.
    // I'll assume for now I will display IDs, and if it's ugly, I'll fix it immediately after.
}

export default function EditSessionModal({ sessionId, onClose, onUpdate }: EditSessionModalProps) {
    const authFetch = useAuthFetch();
    const [loading, setLoading] = useState(false);
    const [title, setTitle] = useState("");
    const [notes, setNotes] = useState("");
    const [date, setDate] = useState("");
    const [startTime, setStartTime] = useState("");
    const [endTime, setEndTime] = useState("");

    // Data State
    const [sets, setSets] = useState<any[]>([]);
    const [cardioLogs, setCardioLogs] = useState<any[]>([]);

    // Deletion State
    const [setsToDelete, setSetsToDelete] = useState<number[]>([]);
    const [cardioToDelete, setCardioToDelete] = useState<number[]>([]);

    // Addition State (Temp Inputs)
    const [newSetVarId, setNewSetVarId] = useState<number | "">("");
    const [newSetWeight, setNewSetWeight] = useState("");
    const [newSetReps, setNewSetReps] = useState("");

    const [newCardioId, setNewCardioId] = useState<number | "">("");
    const [newCardioDuration, setNewCardioDuration] = useState("");

    const [error, setError] = useState("");
    const [variationMap, setVariationMap] = useState<Record<number, { name: string, muscle_group_id: number }>>({});
    const [cardioMap, setCardioMap] = useState<Record<number, string>>({});

    // Metadata Loading
    useEffect(() => {
        const fetchMetadata = async () => {
            try {
                const [vRes, cRes] = await Promise.all([
                    authFetch(`${process.env.API_URL}api/workouts/variations`, { credentials: "include" }),
                    authFetch(`${process.env.API_URL}api/workouts/cardio_exercises`, { credentials: "include" })
                ]);
                if (vRes.ok) {
                    const data = await vRes.json();
                    const map: Record<number, { name: string, muscle_group_id: number }> = {};
                    data.forEach((v: any) => {
                        map[v.id] = { name: v.name, muscle_group_id: v.muscle_group_id };
                    });
                    setVariationMap(map);
                }
                if (cRes.ok) {
                    const data = await cRes.json();
                    const map: Record<number, string> = {};
                    data.forEach((c: any) => map[c.id] = c.name);
                    setCardioMap(map);
                }
            } catch (e) {
                console.error("Failed to load metadata", e);
            }
        };
        fetchMetadata();
    }, []);

    // Load Details
    useEffect(() => {
        if (sessionId) {
            setSetsToDelete([]);
            setCardioToDelete([]);
            setNewSetVarId("");
            setNewSetWeight("");
            setNewSetReps("");
            setNewCardioId("");
            setNewCardioDuration("");
            fetchDetails(sessionId);
        }
    }, [sessionId]);

    const fetchDetails = async (id: number) => {
        setLoading(true);
        setError("");
        try {
            const res = await authFetch(`${process.env.API_URL}api/workouts/session/${id}`, {
                credentials: "include"
            });
            if (!res.ok) throw new Error("Failed to load details");
            const data = await res.json();

            setTitle(data.session.title || "");
            setNotes(data.session.notes || "");
            setDate(data.session.date);

            const sTime = data.session.start_time ? new Date(data.session.start_time).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', hour12: false }) : "";
            const eTime = data.session.end_time ? new Date(data.session.end_time).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', hour12: false }) : "";

            setStartTime(data.session.start_time && data.session.start_time.includes('T') ? data.session.start_time.split('T')[1].substring(0, 5) : sTime);
            setEndTime(data.session.end_time && data.session.end_time.includes('T') ? data.session.end_time.split('T')[1].substring(0, 5) : eTime);

            setSets(data.sets);
            setCardioLogs(data.cardio_logs);
        } catch (err) {
            console.error(err);
            setError("Error loading details");
        } finally {
            setLoading(false);
        }
    };

    // --- Actions ---

    const handleDeleteSet = (id: number) => {
        if (id > 0) {
            setSetsToDelete(prev => [...prev, id]);
        }
        setSets(prev => prev.filter(s => s.id !== id));
    };

    const handleAddSet = () => {
        if (!newSetVarId || !newSetWeight || !newSetReps) return;
        const tempId = -1 * (Math.floor(Math.random() * 10000) + 1); // Random negative ID
        const newSet = {
            id: tempId,
            variation_id: Number(newSetVarId),
            weight: parseFloat(newSetWeight),
            reps: parseInt(newSetReps),
            user_id: 0, // Ignored logic
            workout_session_id: sessionId
        };
        setSets(prev => [...prev, newSet]);
        // Reset inputs
        setNewSetVarId("");
        setNewSetWeight("");
        setNewSetReps("");
    };

    const handleDeleteCardio = (id: number) => {
        if (id > 0) {
            setCardioToDelete(prev => [...prev, id]);
        }
        setCardioLogs(prev => prev.filter(c => c.id !== id));
    };

    const handleAddCardio = () => {
        if (!newCardioId || !newCardioDuration) return;
        const tempId = -1 * (Math.floor(Math.random() * 10000) + 1);
        const newLog = {
            id: tempId,
            cardio_exercise_id: Number(newCardioId),
            duration_minutes: parseInt(newCardioDuration),
            user_id: 0,
            workout_session_id: sessionId
        };
        setCardioLogs(prev => [...prev, newLog]);
        setNewCardioId("");
        setNewCardioDuration("");
    };


    const handleSave = async () => {
        if (!sessionId) return;
        setLoading(true);
        try {
            const apiUrl = process.env.API_URL || '';

            const request = async (endpoint: string, method: string, body?: any) => {
                const res = await authFetch(`${apiUrl}${endpoint}`, {
                    method,
                    headers: { "Content-Type": "application/json" },
                    body: body ? JSON.stringify(body) : undefined,
                    credentials: "include"
                });
                if (!res.ok) {
                    const text = await res.text();
                    throw new Error(`${method} ${endpoint} failed: ${res.status} - ${text}`);
                }
                return res;
            };

            const promises = [];

            // 1. Update Session
            const sFull = startTime ? `${date}T${startTime}:00` : null;
            const eFull = endTime ? `${date}T${endTime}:00` : null;

            promises.push(request(`api/workouts/session/${sessionId}`, "PUT", {
                title: title,
                notes: notes,
                date: date,
                start_time: sFull,
                end_time: eFull,
            }));

            // 2. Process Sets
            sets.forEach(set => {
                if (set.id > 0) {
                    // Update existing
                    promises.push(request(`api/workouts/set/${set.id}`, "PUT", {
                        weight: parseFloat(set.weight),
                        reps: parseInt(set.reps)
                    }));
                } else {
                    // Create new (POST)
                    const mgId = variationMap[set.variation_id]?.muscle_group_id || 1;

                    promises.push(request(`api/workouts/addset`, "POST", {
                        user_id: 0,
                        workout_session_id: sessionId,
                        muscle_group_id: mgId,
                        variation_id: set.variation_id,
                        weight: parseFloat(set.weight),
                        reps: parseInt(set.reps),
                        performed_on: date
                    }));
                }
            });

            // 3. Process Deleted Sets
            setsToDelete.forEach(id => {
                promises.push(request(`api/workouts/set/${id}`, "DELETE"));
            });

            // 4. Process Cardio
            cardioLogs.forEach(log => {
                if (log.id > 0) {
                    // Update
                    promises.push(request(`api/workouts/cardio/${log.id}`, "PUT", {
                        duration: parseInt(log.duration_minutes)
                    }));
                } else {
                    // Create
                    promises.push(request(`api/workouts/addcardio`, "POST", {
                        user_id: 0,
                        workout_session_id: sessionId,
                        cardio_exercise_id: parseInt(log.cardio_exercise_id),
                        duration: parseInt(log.duration_minutes)
                    }));
                }
            });

            // 5. Process Deleted Cardio
            cardioToDelete.forEach(id => {
                promises.push(request(`api/workouts/cardio/${id}`, "DELETE"));
            });

            await Promise.all(promises);
            onUpdate();
            onClose();
        } catch (err: any) {
            console.error(err);
            if (err.message === "UNAUTHORIZED") return;
            setError(err.message || "Failed to save changes");
        } finally {
            setLoading(false);
        }
    };

    return (
        <Modal isOpen={!!sessionId} onClose={onClose} title="Edit Workout" maxWidth="max-w-2xl">
            {loading && <div className="text-center py-4">Saving...</div>}
            {error && <div className="text-red-500 text-center mb-4">{error}</div>}

            {!loading && !error && (
                <div className="space-y-4 max-h-[70vh] overflow-y-auto pr-2 custom-scrollbar">
                    {/* Session Fields (unchanged mostly) */}
                    <div>
                        <label className="block text-sm font-bold text-gray-900 mb-1">Title</label>
                        <input value={title} onChange={e => setTitle(e.target.value)} className="w-full p-2 text-gray-900 border rounded-lg" />
                    </div>
                    <div>
                        <label className="block text-sm font-bold text-gray-900 mb-1">Date</label>
                        <input type="date" value={date} onChange={e => setDate(e.target.value)} className="w-full p-2 text-gray-900 border rounded-lg" />
                    </div>
                    <div className="flex gap-4">
                        <div className="flex-1">
                            <label className="block text-sm font-bold text-gray-900 mb-1">Start Time</label>
                            <input type="time" value={startTime} onChange={e => setStartTime(e.target.value)} className="w-full p-2 text-gray-900 border rounded-lg" />
                        </div>
                        <div className="flex-1">
                            <label className="block text-sm font-bold text-gray-900 mb-1">End Time</label>
                            <input type="time" value={endTime} onChange={e => setEndTime(e.target.value)} className="w-full p-2 text-gray-900 border rounded-lg" />
                        </div>
                    </div>
                    <div>
                        <label className="block text-sm font-bold text-gray-900 mb-1">Notes</label>
                        <textarea value={notes} onChange={e => setNotes(e.target.value)} className="w-full p-2 text-gray-900 border rounded-lg" rows={2} />
                    </div>

                    <hr />

                    {/* Sets Management */}
                    <div>
                        <h4 className="font-bold text-gray-900 mb-2">Sets</h4>
                        <div className="space-y-2 mb-3">
                            {sets.map((set, idx) => (
                                <div key={set.id} className="flex gap-2 items-center bg-gray-50 p-2 rounded-lg">
                                    <div className="w-6 text-xs font-bold text-blue-600">#{idx + 1}</div>
                                    <div className="flex-1 truncate font-medium text-gray-700 text-sm">
                                        {variationMap[set.variation_id]?.name || `Var #${set.variation_id}`}
                                    </div>
                                    <input type="number" value={set.weight}
                                        onChange={e => {
                                            const newSets = [...sets];
                                            newSets[idx].weight = e.target.value;
                                            setSets(newSets);
                                        }}
                                        className="w-16 p-1 text-gray-900 border rounded text-sm text-center"
                                    />
                                    <span className="text-xs font-bold text-gray-500">kg</span>
                                    <input type="number" value={set.reps}
                                        onChange={e => {
                                            const newSets = [...sets];
                                            newSets[idx].reps = e.target.value;
                                            setSets(newSets);
                                        }}
                                        className="w-12 p-1 text-gray-900 border rounded text-sm text-center"
                                    />
                                    <span className="text-xs font-bold text-gray-500">reps</span>

                                    <button onClick={() => handleDeleteSet(set.id)} className="p-1 text-red-500 hover:bg-red-50 rounded">
                                        <TrashIcon className="w-4 h-4" />
                                    </button>
                                </div>
                            ))}
                        </div>

                        {/* Add Set Row */}
                        <div className="flex gap-2 items-center bg-blue-50/50 p-2 rounded-lg border border-blue-100 border-dashed">
                            <div className="w-6 text-xs font-bold text-blue-300">+</div>
                            <select
                                value={newSetVarId}
                                onChange={e => setNewSetVarId(Number(e.target.value))}
                                className="flex-1 p-1 text-sm border rounded text-gray-900"
                            >
                                <option value="">Select Exercise...</option>
                                {Object.entries(variationMap).map(([id, val]) => (
                                    <option key={id} value={id}>{val.name}</option>
                                ))}
                            </select>
                            <input type="number" placeholder="kg" value={newSetWeight} onChange={e => setNewSetWeight(e.target.value)} className="w-16 p-1 text-sm border rounded text-gray-900" />
                            <input type="number" placeholder="reps" value={newSetReps} onChange={e => setNewSetReps(e.target.value)} className="w-12 p-1 text-sm border rounded text-gray-900" />
                            <button
                                onClick={handleAddSet}
                                disabled={!newSetVarId || !newSetWeight || !newSetReps}
                                className="px-3 py-1 bg-blue-600 text-white text-xs font-bold rounded disabled:opacity-50"
                            >
                                Add
                            </button>
                        </div>
                    </div>

                    <hr />

                    {/* Cardio Management */}
                    <div>
                        <h4 className="font-bold text-gray-900 mb-2">Cardio</h4>
                        <div className="space-y-2 mb-3">
                            {cardioLogs.map((log, idx) => (
                                <div key={log.id} className="flex gap-2 items-center bg-gray-50 p-2 rounded-lg">
                                    <div className="w-6 text-xs font-bold text-blue-600">#{idx + 1}</div>
                                    <div className="flex-1 truncate font-medium text-gray-700 text-sm">
                                        {cardioMap[log.cardio_exercise_id] || `Cardio #${log.cardio_exercise_id}`}
                                    </div>
                                    <input type="number" value={log.duration_minutes}
                                        onChange={e => {
                                            const newLogs = [...cardioLogs];
                                            newLogs[idx].duration_minutes = e.target.value;
                                            setCardioLogs(newLogs);
                                        }}
                                        className="w-20 p-1 border rounded text-sm text-center"
                                    />
                                    <span className="text-xs font-bold text-gray-500">mins</span>
                                    <button onClick={() => handleDeleteCardio(log.id)} className="p-1 text-red-500 hover:bg-red-50 rounded">
                                        <TrashIcon className="w-4 h-4" />
                                    </button>
                                </div>
                            ))}
                        </div>

                        {/* Add Cardio Row */}
                        <div className="flex gap-2 items-center bg-blue-50/50 p-2 rounded-lg border border-blue-100 border-dashed">
                            <div className="w-6 text-xs font-bold text-blue-300">+</div>
                            <select
                                value={newCardioId}
                                onChange={e => setNewCardioId(Number(e.target.value))}
                                className="flex-1 p-1 text-sm border rounded text-gray-900"
                            >
                                <option value="">Select Activity...</option>
                                {Object.entries(cardioMap).map(([id, name]) => (
                                    <option key={id} value={id}>{name}</option>
                                ))}
                            </select>
                            <input type="number" placeholder="mins" value={newCardioDuration} onChange={e => setNewCardioDuration(e.target.value)} className="w-20 p-1 text-sm border rounded text-gray-900" />
                            <button
                                onClick={handleAddCardio}
                                disabled={!newCardioId || !newCardioDuration}
                                className="px-3 py-1 bg-blue-600 text-white text-xs font-bold rounded disabled:opacity-50"
                            >
                                Add
                            </button>
                        </div>
                    </div>

                    <div className="pt-4 flex gap-3">
                        <button onClick={onClose} className="flex-1 py-2 text-gray-600 bg-gray-100 rounded-lg font-bold">Cancel</button>
                        <button onClick={handleSave} className="flex-1 py-2 text-white bg-blue-600 rounded-lg font-bold">Save Changes</button>
                    </div>
                </div>
            )}
        </Modal>
    );
}
