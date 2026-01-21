"use client";

import { useState, useEffect } from "react";
import { useRouter } from "next/navigation";
import Modal from "../../components/Modal";
import Popup from "../../components/Popup";
import { PlusIcon, ChevronDownIcon, ArrowPathIcon, TrashIcon } from "@heroicons/react/24/outline";

type LogType = 'strength' | 'cardio';

interface MuscleGroup {
    id: number;
    name: string;
}

interface Variation {
    id: number;
    muscle_group_id: number;
    name: string;
}

interface CardioExercise {
    id: number;
    name: string;
}

interface BaseLogEntry {
    id: number;
    type: LogType;
    timestamp: string;
    date: string;
}

interface StrengthLogEntry extends BaseLogEntry {
    type: 'strength';
    muscleGroupId: number;
    muscleGroupName: string;
    variationId: number;
    variationName: string;
    weight: string;
    reps: string;
}

interface CardioLogEntry extends BaseLogEntry {
    type: 'cardio';
    cardioExerciseId: number;
    cardioExerciseName: string;
    duration: string;
}

type LogEntry = StrengthLogEntry | CardioLogEntry;

export default function LogsPage() {
    const router = useRouter();

    const [title, setTitle] = useState("");
    const [notes, setNotes] = useState("");
    const [date, setDate] = useState(new Date().toISOString().split("T")[0]);
    const [startTime, setStartTime] = useState("06:00");
    const [endTime, setEndTime] = useState("07:30");

    const [logType, setLogType] = useState<LogType>('strength');

    const [selectedMuscleGroupId, setSelectedMuscleGroupId] = useState<number | "">("");
    const [selectedVariationId, setSelectedVariationId] = useState<number | "">("");
    const [selectedCardioId, setSelectedCardioId] = useState<number | "">("");

    const [weight, setWeight] = useState("");
    const [reps, setReps] = useState("");
    const [duration, setDuration] = useState("");

    const [muscleGroups, setMuscleGroups] = useState<MuscleGroup[]>([]);
    const [variations, setVariations] = useState<Variation[]>([]);
    const [cardioExercises, setCardioExercises] = useState<CardioExercise[]>([]);
    const [dailyLogs, setDailyLogs] = useState<LogEntry[]>([]);

    const [isModalOpen, setIsModalOpen] = useState(false);
    const [modalType, setModalType] = useState<"muscle" | "variation" | "cardio" | null>(null);
    const [newItemName, setNewItemName] = useState("");
    const [popupMessage, setPopupMessage] = useState("");
    const [isSaving, setIsSaving] = useState(false);
    const [isClearModalOpen, setIsClearModalOpen] = useState(false);

    const [isNavWarningOpen, setIsNavWarningOpen] = useState(false);
    const [pendingUrl, setPendingUrl] = useState<string | null>(null);

    // Warn about unsaved changes on browser close/refresh AND internal navigation
    useEffect(() => {
        const handleBeforeUnload = (e: BeforeUnloadEvent) => {
            if (dailyLogs.length > 0) {
                e.preventDefault();
                e.returnValue = '';
            }
        };

        const handleAnchorClick = (e: MouseEvent) => {
            if (dailyLogs.length > 0) {
                const target = (e.target as HTMLElement).closest('a');
                if (target) {
                    if (target.href && target.href.startsWith(window.location.origin) && !target.getAttribute('download')) {
                        e.preventDefault();
                        e.stopImmediatePropagation();
                        setPendingUrl(target.href);
                        setIsNavWarningOpen(true);
                    }
                }
            }
        };

        window.addEventListener('beforeunload', handleBeforeUnload);
        window.addEventListener('click', handleAnchorClick, true);

        return () => {
            window.removeEventListener('beforeunload', handleBeforeUnload);
            window.removeEventListener('click', handleAnchorClick, true);
        };
    }, [dailyLogs]);

    const confirmNavigation = () => {
        if (pendingUrl) {
            window.location.href = pendingUrl;
        }
        setIsNavWarningOpen(false);
    };

    const availableVariations = selectedMuscleGroupId
        ? variations.filter(v => v.muscle_group_id === Number(selectedMuscleGroupId))
        : [];

    useEffect(() => {
        const fetchData = async () => {
            try {
                const [mgRes, varRes, cRes] = await Promise.all([
                    fetch(`${process.env.API_URL}api/workouts/muscle_groups`, { credentials: "include" }),
                    fetch(`${process.env.API_URL}api/workouts/variations`, { credentials: "include" }),
                    fetch(`${process.env.API_URL}api/workouts/cardio_exercises`, { credentials: "include" })
                ]);

                if (mgRes.ok) setMuscleGroups(await mgRes.json());
                if (varRes.ok) setVariations(await varRes.json());
                if (cRes.ok) setCardioExercises(await cRes.json());
            } catch (e) {
                console.error("Failed to load metadata", e);
            }
        };
        fetchData();
    }, []);

    const handleCreateNew = (type: "muscle" | "variation" | "cardio") => {
        setModalType(type);
        setNewItemName("");
        setIsModalOpen(true);
    };

    const submitNewItem = async () => {
        if (!newItemName.trim()) return;

        try {
            const apiUrl = process.env.API_URL || '';
            let newId: number | null = null;
            let success = false;

            if (modalType === "muscle") {
                const res = await fetch(`${apiUrl}api/workouts/muscle_groups`, {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify({ name: newItemName }),
                    credentials: "include"
                });
                const data = await res.json();
                if (data.success && data.id) {
                    newId = data.id;
                    setMuscleGroups([...muscleGroups, { id: newId!, name: newItemName }]);
                    setSelectedMuscleGroupId(newId!);
                    setSelectedVariationId("");
                    success = true;
                }
            } else if (modalType === "variation" && selectedMuscleGroupId) {
                const res = await fetch(`${apiUrl}api/workouts/variations`, {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify({ name: newItemName, muscle_group_id: Number(selectedMuscleGroupId) }),
                    credentials: "include"
                });
                const data = await res.json();
                if (data.success && data.id) {
                    newId = data.id;
                    setVariations([...variations, { id: newId!, muscle_group_id: Number(selectedMuscleGroupId), name: newItemName }]);
                    setSelectedVariationId(newId!);
                    success = true;
                }
            } else if (modalType === "cardio") {
                const res = await fetch(`${apiUrl}api/workouts/cardio_exercises`, {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify({ name: newItemName }),
                    credentials: "include"
                });
                const data = await res.json();
                if (data.success && data.id) {
                    newId = data.id;
                    setCardioExercises([...cardioExercises, { id: newId!, name: newItemName }]);
                    setSelectedCardioId(newId!);
                    success = true;
                }
            }

            if (success) {
                setPopupMessage("Created Successfully!");
                setIsModalOpen(false);
            } else {
                setPopupMessage("Failed to create.");
            }

        } catch (e) {
            console.error(e);
            setPopupMessage("Error creating item.");
        }
    };

    const handleAddEntry = (e: React.FormEvent) => {
        e.preventDefault();

        let newLog: LogEntry;

        if (logType === 'strength') {
            if (!selectedMuscleGroupId || !selectedVariationId) return;
            const mg = muscleGroups.find(m => m.id === Number(selectedMuscleGroupId));
            const v = variations.find(v => v.id === Number(selectedVariationId));

            newLog = {
                id: Date.now(),
                type: 'strength',
                muscleGroupId: Number(selectedMuscleGroupId),
                muscleGroupName: mg?.name || "Unknown",
                variationId: Number(selectedVariationId),
                variationName: v?.name || "Unknown",
                weight,
                reps,
                timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
                date: date
            };
            setPopupMessage(`Added: ${v?.name} - ${weight}kg x ${reps} `);
        } else {
            if (!selectedCardioId) return;
            const c = cardioExercises.find(c => c.id === Number(selectedCardioId));

            newLog = {
                id: Date.now(),
                type: 'cardio',
                cardioExerciseId: Number(selectedCardioId),
                cardioExerciseName: c?.name || "Unknown",
                duration,
                timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
                date: date
            };
            setPopupMessage(`Added: ${c?.name} - ${duration} mins`);
        }

        // Local State Update ONLY
        setDailyLogs((prev) => [newLog, ...prev]);
    };

    const handleDeleteEntry = (id: number) => {
        setDailyLogs((prev) => prev.filter(log => log.id !== id));
    };

    const handleClearForm = () => {
        if (logType === 'strength') {
            setSelectedMuscleGroupId("");
            setSelectedVariationId("");
            setWeight("");
            setReps("");
        } else {
            setSelectedCardioId("");
            setDuration("");
        }
    };

    const handleSaveWorkout = async () => {
        if (dailyLogs.length === 0) {
            setPopupMessage("Add at least one set!");
            return;
        }
        if (startTime >= endTime) {
            setPopupMessage("Start time must be before end time!");
            return;
        }
        setIsSaving(true);
        try {
            const sessionStart = `${date}T${startTime}:00`;
            const sessionEnd = `${date}T${endTime}:00`;
            const apiUrl = process.env.API_URL || '';

            const sessionRes = await fetch(`${apiUrl}api/workouts/addsession`, {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({
                    user_id: 0,
                    title: title,
                    notes: notes,
                    date: date,
                    start_time: sessionStart,
                    end_time: sessionEnd,
                }),
                credentials: "include"
            });
            if (!sessionRes.ok) throw new Error("Failed to create session");

            const sessionData = await sessionRes.json();
            const sessionId = sessionData.id;

            const apiCalls = dailyLogs.map(log => {
                if (log.type === 'strength') {
                    return fetch(`${apiUrl}api/workouts/addset`, {
                        method: "POST",
                        headers: { "Content-Type": "application/json" },
                        body: JSON.stringify({
                            user_id: 0,
                            workout_session_id: sessionId,
                            muscle_group_id: log.muscleGroupId,
                            variation_id: log.variationId,
                            weight: parseFloat(log.weight),
                            reps: parseInt(log.reps),
                            performed_on: log.date,
                        }),
                        credentials: "include"
                    });
                } else {
                    return fetch(`${apiUrl}api/workouts/addcardio`, {
                        method: "POST",
                        headers: { "Content-Type": "application/json" },
                        body: JSON.stringify({
                            user_id: 0,
                            workout_session_id: sessionId,
                            cardio_exercise_id: log.cardioExerciseId,
                            duration: parseInt(log.duration),
                        }),
                        credentials: "include"
                    });
                }
            });

            await Promise.all(apiCalls);

            setPopupMessage("Workout Saved!");
            setDailyLogs([]);
            router.push("/records");

        } catch (e) {
            console.error(e);
            setPopupMessage("Failed to save workout.");
        } finally {
            setIsSaving(false);
        }
    };

    const confirmClearSession = () => {
        setDailyLogs([]);
        setPopupMessage("Cleared.");
        setIsClearModalOpen(false);
    };

    const currentDayLogs = dailyLogs.filter(log => log.date === date);

    const groupedLogs = currentDayLogs.reduce((acc, log) => {
        let groupKey = log.type === 'strength'
            ? `${log.muscleGroupName} - ${log.variationName}`
            : `${log.cardioExerciseName}`;

        if (!acc[groupKey]) acc[groupKey] = [];
        acc[groupKey].push(log);
        return acc;
    }, {} as Record<string, LogEntry[]>);

    return (
        <div className="max-w-2xl mx-auto space-y-8 pb-32">
            <div>
                <h1 className="text-3xl font-bold text-black">Log Workout</h1>
                <p className="text-gray-600 mt-2 font-medium">Record your sets. Save when finished.</p>
            </div>

            <div className="bg-white p-8 rounded-2xl shadow-[4px_0_20px_rgba(0,0,0,0.05)] border border-gray-100 relative">
                <form onSubmit={handleAddEntry} className="space-y-6">
                    <div>
                        <label className="text-sm font-bold text-gray-900 block mb-2">Workout Title</label>
                        <input
                            type="text"
                            value={title}
                            onChange={(e) => setTitle(e.target.value)}
                            placeholder="e.g. Chest Day & Cardio"
                            className="p-3 bg-gray-50 border border-gray-200 rounded-xl text-gray-900 font-semibold focus:outline-none focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500 transition-all w-full"
                        />
                    </div>
                    <div>
                        <label className="text-sm font-bold text-gray-900 block mb-2">Notes</label>
                        <textarea
                            value={notes}
                            onChange={(e) => setNotes(e.target.value)}
                            placeholder=""
                            className="p-3 bg-gray-50 border border-gray-200 rounded-xl text-gray-900 font-semibold focus:outline-none focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500 transition-all w-full"
                        />
                    </div>
                    <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                        <div className="flex flex-col gap-2">
                            <label className="text-sm font-bold text-gray-900">Date</label>
                            <input
                                type="date"
                                value={date}
                                onChange={(e) => setDate(e.target.value)}
                                className="p-3 bg-gray-50 border border-gray-200 rounded-xl text-gray-900 font-semibold focus:outline-none focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500 transition-all w-full"
                                required
                            />
                        </div>
                        <div className="flex flex-col gap-2">
                            <label className="text-sm font-bold text-gray-900">Start Time</label>
                            <input
                                type="time"
                                value={startTime}
                                onChange={(e) => setStartTime(e.target.value)}
                                className="p-3 bg-gray-50 border border-gray-200 rounded-xl text-gray-900 font-semibold focus:outline-none focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500 transition-all w-full"
                            />
                        </div>
                        <div className="flex flex-col gap-2">
                            <label className="text-sm font-bold text-gray-900">End Time</label>
                            <input
                                type="time"
                                value={endTime}
                                onChange={(e) => setEndTime(e.target.value)}
                                className="p-3 bg-gray-50 border border-gray-200 rounded-xl text-gray-900 font-semibold focus:outline-none focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500 transition-all w-full"
                            />
                        </div>
                    </div>

                    <hr className="border-gray-100" />

                    <div className="flex gap-1 p-1 bg-gray-100 rounded-xl">
                        <button type="button" onClick={() => setLogType('strength')} className={`flex-1 py-2 rounded-lg font-bold transition-all ${logType === 'strength' ? 'bg-white text-black shadow-sm' : 'text-gray-500 hover:text-gray-700'}`}>Strength</button>
                        <button type="button" onClick={() => setLogType('cardio')} className={`flex-1 py-2 rounded-lg font-bold transition-all ${logType === 'cardio' ? 'bg-white text-black shadow-sm' : 'text-gray-500 hover:text-gray-700'}`}>Cardio</button>
                    </div>

                    {logType === 'strength' ? (
                        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                            <div className="flex flex-col gap-2">
                                <label className="text-sm font-bold text-gray-900">Muscle Group</label>
                                <div className="relative">
                                    <select
                                        value={selectedMuscleGroupId}
                                        onChange={(e) => {
                                            const val = e.target.value;
                                            if (val === "CREATE_NEW") handleCreateNew("muscle");
                                            else {
                                                setSelectedMuscleGroupId(Number(val));
                                                setSelectedVariationId("");
                                            }
                                        }}
                                        className="p-3 bg-gray-50 border border-gray-200 rounded-xl text-gray-900 font-semibold w-full pr-10 appearance-none"
                                        required
                                    >
                                        <option value="" disabled>Select Muscle Group</option>
                                        {muscleGroups.map(g => <option key={g.id} value={g.id}>{g.name}</option>)}
                                        <option value="CREATE_NEW" className="font-bold text-blue-600 bg-blue-50">+ Create New Muscle Group</option>
                                    </select>
                                    <ChevronDownIcon className="w-5 h-5 text-gray-500 absolute right-3 top-3.5 pointer-events-none" />
                                </div>
                            </div>
                            <div className="flex flex-col gap-2">
                                <label className="text-sm font-bold text-gray-900">Variation</label>
                                <div className="relative">
                                    <select
                                        value={selectedVariationId}
                                        onChange={(e) => {
                                            const val = e.target.value;
                                            if (val === "CREATE_NEW") handleCreateNew("variation");
                                            else setSelectedVariationId(Number(val));
                                        }}
                                        className="p-3 bg-gray-50 border border-gray-200 rounded-xl text-gray-900 font-semibold w-full pr-10 appearance-none disabled:opacity-50"
                                        disabled={!selectedMuscleGroupId}
                                        required
                                    >
                                        <option value="" disabled>Select Exercise</option>
                                        {availableVariations.map(v => <option key={v.id} value={v.id}>{v.name}</option>)}
                                        <option value="CREATE_NEW" className="font-bold text-blue-600 bg-blue-50">+ Create New Exercise</option>
                                    </select>
                                    <ChevronDownIcon className="w-5 h-5 text-gray-500 absolute right-3 top-3.5 pointer-events-none" />
                                </div>
                            </div>
                            <div className="flex flex-col gap-2">
                                <label className="text-sm font-bold text-gray-900">Weight (kg)</label>
                                <input type="number" value={weight} onChange={e => setWeight(e.target.value)} className="p-3 bg-gray-50 border border-gray-200 rounded-xl text-gray-900 font-semibold" required min="0" step="0.5" />
                            </div>
                            <div className="flex flex-col gap-2">
                                <label className="text-sm font-bold text-gray-900">Reps</label>
                                <input type="number" value={reps} onChange={e => setReps(e.target.value)} className="p-3 bg-gray-50 border border-gray-200 rounded-xl text-gray-900 font-semibold" required min="1" />
                            </div>
                        </div>
                    ) : (
                        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                            <div className="flex flex-col gap-2">
                                <label className="text-sm font-bold text-gray-900">Exercise</label>
                                <div className="relative">
                                    <select
                                        value={selectedCardioId}
                                        onChange={(e) => {
                                            const val = e.target.value;
                                            if (val === "CREATE_NEW") handleCreateNew("cardio");
                                            else setSelectedCardioId(Number(val));
                                        }}
                                        className="p-3 bg-gray-50 border border-gray-200 rounded-xl text-gray-900 font-semibold w-full pr-10 appearance-none"
                                        required
                                    >
                                        <option value="" disabled>Select Cardio</option>
                                        {cardioExercises.map(e => <option key={e.id} value={e.id}>{e.name}</option>)}
                                        <option value="CREATE_NEW" className="font-bold text-blue-600 bg-blue-50">+ Create New Cardio</option>
                                    </select>
                                    <ChevronDownIcon className="w-5 h-5 text-gray-500 absolute right-3 top-3.5 pointer-events-none" />
                                </div>
                            </div>
                            <div className="flex flex-col gap-2">
                                <label className="text-sm font-bold text-gray-900">Duration (mins)</label>
                                <input type="number" value={duration} onChange={e => setDuration(e.target.value)} className="p-3 bg-gray-50 border border-gray-200 text-gray-900 rounded-xl font-semibold" required min="1" />
                            </div>
                        </div>
                    )}

                    <div className="flex gap-3 pt-2">
                        <button type="button" onClick={handleClearForm} className="flex-1 py-4 bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-xl font-bold transition-all flex items-center justify-center gap-2">
                            <ArrowPathIcon className="w-5 h-5 font-bold" />
                            Clear
                        </button>
                        <button type="submit" className="flex-[2] py-4 bg-blue-600 hover:bg-blue-700 text-white rounded-xl font-bold shadow-lg shadow-blue-600/20 transition-all flex items-center justify-center gap-2">
                            <PlusIcon className="w-5 h-5 font-bold" />
                            Add to Session
                        </button>
                    </div>
                </form>
            </div>

            {Object.keys(groupedLogs).length > 0 && (
                <div className="border-t border-gray-200 pt-8">
                    <div className="flex items-center justify-between mb-6">
                        <h2 className="text-xl font-bold text-gray-900">Current Session</h2>
                        <div className="text-sm font-medium text-gray-500 bg-gray-100 px-3 py-1 rounded-full">{currentDayLogs.length} Sets</div>
                    </div>

                    <div className="space-y-4">
                        {Object.entries(groupedLogs).map(([groupName, logs]) => (
                            <div key={groupName} className="bg-white rounded-2xl shadow-sm border border-gray-100 overflow-hidden">
                                <div className="bg-gray-50 px-6 py-3 border-b border-gray-100 font-bold text-gray-900">{groupName}</div>
                                <div className="divide-y divide-gray-50">
                                    {logs.map((log, idx) => (
                                        <div key={log.id} className="px-6 py-3 flex justify-between items-center hover:bg-gray-50 transition-colors">
                                            <div className="flex items-center gap-4">
                                                <span className="w-6 h-6 rounded-full bg-blue-50 text-blue-600 text-xs font-bold flex items-center justify-center">{idx + 1}</span>
                                                {log.type === 'strength' ? (
                                                    <span className="font-semibold text-gray-900">{log.weight}kg x {log.reps}</span>
                                                ) : (
                                                    <span className="font-semibold text-gray-900">{log.duration} mins</span>
                                                )}
                                            </div>
                                            <button onClick={() => handleDeleteEntry(log.id)} className="text-gray-300 hover:text-red-500 p-2"><TrashIcon className="w-5 h-5" /></button>
                                        </div>
                                    ))}
                                </div>
                            </div>
                        ))}
                    </div>
                </div>
            )}

            <div className="fixed bottom-0 left-0 right-0 p-4 bg-white/80 backdrop-blur-xl border-t border-gray-200 z-50">
                <div className="max-w-2xl mx-auto flex gap-4">
                    <button
                        onClick={() => setIsClearModalOpen(true)}
                        className="px-6 py-3 rounded-xl font-bold text-red-600 border border-red-100 bg-red-50 hover:bg-red-100 transition-all text-sm"
                    >
                        Clear Session
                    </button>
                    <button
                        onClick={handleSaveWorkout}
                        disabled={isSaving}
                        className="flex-1 py-3 bg-black hover:bg-gray-800 text-white rounded-xl font-bold shadow-lg transition-all active:scale-[0.98] text-sm disabled:opacity-50"
                    >
                        {isSaving ? "Saving..." : "Save Workout"}
                    </button>
                </div>
            </div>

            <Modal isOpen={isModalOpen} onClose={() => setIsModalOpen(false)} title="Create New">
                <input value={newItemName} onChange={e => setNewItemName(e.target.value)} className="w-full p-3 border rounded-xl mb-4" placeholder="Name" autoFocus />
                <button onClick={submitNewItem} className="w-full py-3 bg-blue-600 text-white rounded-xl font-bold">Create</button>
            </Modal>

            {/* Clear Session Confirmation Modal */}
            <Modal isOpen={isClearModalOpen} onClose={() => setIsClearModalOpen(false)} title="Clear Session?">
                <div className="space-y-4">
                    <p className="text-gray-600">Are you sure you want to clear all unsaved logs? This action cannot be undone.</p>
                    <div className="flex gap-3">
                        <button
                            onClick={() => setIsClearModalOpen(false)}
                            className="flex-1 py-3 bg-gray-100 text-gray-700 rounded-xl font-bold hover:bg-gray-200 transition-colors"
                        >
                            Cancel
                        </button>
                        <button
                            onClick={confirmClearSession}
                            className="flex-1 py-3 bg-red-600 text-white rounded-xl font-bold hover:bg-red-700 transition-colors shadow-lg shadow-red-600/20"
                        >
                            Clear All
                        </button>
                    </div>
                </div>
            </Modal>

            {/* Navigation Warning Modal */}
            <Modal isOpen={isNavWarningOpen} onClose={() => setIsNavWarningOpen(false)} title="Unsaved Changes">
                <div className="space-y-4">
                    <p className="text-gray-600">You have unsaved changes. Leaving this page will discard them.</p>
                    <div className="flex gap-3">
                        <button
                            onClick={() => setIsNavWarningOpen(false)}
                            className="flex-1 py-3 bg-gray-100 text-gray-700 rounded-xl font-bold hover:bg-gray-200 transition-colors"
                        >
                            Stay
                        </button>
                        <button
                            onClick={confirmNavigation}
                            className="flex-1 py-3 bg-red-600 text-white rounded-xl font-bold hover:bg-red-700 transition-colors shadow-lg shadow-red-600/20"
                        >
                            Leave
                        </button>
                    </div>
                </div>
            </Modal>

            <Popup message={popupMessage} duration={1000} onClose={() => setPopupMessage("")} />
        </div>
    );
}