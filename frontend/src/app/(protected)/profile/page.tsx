"use client";

import { useState, useEffect } from "react";
import { useRouter } from "next/navigation";
import Popup from "../../components/Popup";
import { UserCircleIcon, PencilIcon, CheckIcon, XMarkIcon, KeyIcon } from "@heroicons/react/24/outline";
import { useAuthFetch } from "../../hooks/useAuthFetch";

export default function ProfilePage() {
    const router = useRouter(); // Keep for password reset nav
    const authFetch = useAuthFetch();
    const [isEditing, setIsEditing] = useState(false);
    const [popupMessage, setPopupMessage] = useState("");
    const [isLoading, setIsLoading] = useState(true);

    const [formData, setFormData] = useState({
        username: "username",
        fullname: "name",
        email: "email",
        joined: "",
        weight: "0",
        height: "0",
        dob: "2000-01-01",
    });

    const [originalData, setOriginalData] = useState(formData);

    useEffect(() => {
        const fetchUserInfo = async () => {
            try {
                const userinfo_result = await authFetch(`${process.env.API_URL}api/userinfo`, {
                    method: "GET",
                    headers: { "Content-Type": "application/json" },
                    credentials: "include",
                });
                if (!userinfo_result.ok) throw new Error("Failed to fetch user info");
                const userinfo = await userinfo_result.json();

                let joinedDate = "Dec 2024";
                if (userinfo.created_at) {
                    const date = new Date(userinfo.created_at);
                    joinedDate = date.toLocaleDateString("en-US", { month: "short", year: "numeric" });
                }

                const formattedData = {
                    username: userinfo.username || "username",
                    fullname: userinfo.fullname || "",
                    email: userinfo.email || "",
                    joined: joinedDate,
                    weight: userinfo.weight?.toString() || "0",
                    height: userinfo.height?.toString() || "0",
                    dob: userinfo.dob || "2000-01-01",
                };

                setFormData(formattedData);
                setOriginalData(formattedData);
            } catch (err) {
                console.error("Error fetching user info:", err);
                setPopupMessage("Error fetching user info. Please try again later.");
            } finally {
                setIsLoading(false);
            }
        };

        fetchUserInfo();
    }, []);

    const handleSave = async () => {

        try {
            const update_result = await authFetch(`${process.env.API_URL}api/updateuser`, {
                method: "PUT",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({
                    fullname: formData.fullname,
                    email: formData.email,
                    weight: parseFloat(formData.weight),
                    height: parseFloat(formData.height),
                    dob: formData.dob,
                }),
                credentials: "include"
            });

            if (!update_result.ok) {
                const text = await update_result.text();
                console.error("Update failed status:", update_result.status);
                console.error("Update failed body:", text);

                let errorMessage = "Server Error";
                try {
                    const errData = JSON.parse(text);
                    errorMessage = errData.message || errorMessage;
                } catch {
                    errorMessage = text || `Error ${update_result.status}`;
                }
                setPopupMessage(`Failed: ${errorMessage}`);
                return;
            }
        } catch (error) {
            console.error("Network/Fetch error:", error);
            setPopupMessage("Network error. Is backend running?");
            return;
        }
        console.log("Saving profile:", formData);
        setOriginalData(formData);
        setIsEditing(false);
        setPopupMessage("Profile updated successfully!");
    };

    const handleCancel = () => {
        setFormData(originalData);
        setIsEditing(false);
    };

    const handleChange = (field: string, value: string) => {
        setFormData(prev => ({ ...prev, [field]: value }));
    };

    if (isLoading) {
        return <div className="p-8 text-center text-gray-500 font-bold">Loading profile...</div>;
    }

    return (
        <div className="max-w-2xl mx-auto space-y-8 pb-32">
            <div className="flex items-center justify-between">
                <div>
                    <h1 className="text-3xl font-bold text-black">Profile</h1>
                    <p className="text-gray-600 mt-2 font-medium">Manage your personal information.</p>
                </div>
                {!isEditing && (
                    <button
                        onClick={() => setIsEditing(true)}
                        className="flex items-center gap-2 px-5 py-2.5 bg-gray-900 text-white rounded-xl font-bold hover:bg-black transition-all shadow-sm active:scale-95"
                    >
                        <PencilIcon className="w-4 h-4" />
                        Edit Profile
                    </button>
                )}
            </div>

            <div className="bg-white p-8 rounded-2xl shadow-[4px_0_20px_rgba(0,0,0,0.05)] border border-gray-100 relative">
                <div className="space-y-8">

                    <div className="flex items-center gap-6 pb-8 border-b border-gray-100">
                        <div className="w-20 h-20 bg-gray-100 rounded-full flex items-center justify-center text-gray-400">
                            <UserCircleIcon className="w-12 h-12" />
                        </div>
                        <div>
                            <h2 className="text-xl font-bold text-gray-900">@{formData.username}</h2>
                            <p className="text-gray-500 font-medium">Member since {formData.joined}</p>
                        </div>
                    </div>

                    <div className="grid grid-cols-1 gap-6">
                        <div className="flex flex-col gap-2">
                            <label className="text-sm font-bold text-gray-900">Full name</label>
                            <input
                                type="text"
                                value={formData.fullname}
                                onChange={(e) => handleChange("fullname", e.target.value)}
                                disabled={!isEditing}
                                className={`p-3 rounded-xl font-semibold transition-all outline-none ${isEditing
                                    ? "bg-gray-50 border border-gray-200 text-gray-900 focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500"
                                    : "bg-transparent border-transparent text-gray-900 pl-0"
                                    }`}
                            />
                        </div>

                        <div className="flex flex-col gap-2">
                            <label className="text-sm font-bold text-gray-900">Email</label>
                            <input
                                type="email"
                                value={formData.email}
                                onChange={(e) => handleChange("email", e.target.value)}
                                disabled={!isEditing}
                                className={`p-3 rounded-xl font-semibold transition-all outline-none ${isEditing
                                    ? "bg-gray-50 border border-gray-200 text-gray-900 focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500"
                                    : "bg-transparent border-transparent text-gray-900 pl-0"
                                    }`}
                            />
                        </div>

                        <div className="flex flex-col gap-2">
                            <label className="text-sm font-bold text-gray-900">Password</label>
                            <button
                                onClick={() => router.push('/reset-pwd')}
                                className="flex w-50 items-center gap-2 p-3 text-left rounded-xl font-semibold text-gray-700 bg-gray-50 hover:bg-gray-100 transition-all border border-gray-200"
                            >
                                <KeyIcon className="w-5 h-5 text-gray-500" />
                                Change Password
                            </button>
                        </div>

                        <div className="grid grid-cols-2 gap-6">
                            <div className="flex flex-col gap-2">
                                <label className="text-sm font-bold text-gray-900">Weight (kg)</label>
                                <input
                                    type="number"
                                    value={formData.weight}
                                    onChange={(e) => handleChange("weight", e.target.value)}
                                    disabled={!isEditing}
                                    className={`p-3 rounded-xl font-semibold transition-all outline-none ${isEditing
                                        ? "bg-gray-50 border border-gray-200 text-gray-900 focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500"
                                        : "bg-transparent border-transparent text-gray-900 pl-0"
                                        }`}
                                />
                            </div>

                            <div className="flex flex-col gap-2">
                                <label className="text-sm font-bold text-gray-900">Height (cm)</label>
                                <input
                                    type="number"
                                    value={formData.height}
                                    onChange={(e) => handleChange("height", e.target.value)}
                                    disabled={!isEditing}
                                    className={`p-3 rounded-xl font-semibold transition-all outline-none ${isEditing
                                        ? "bg-gray-50 border border-gray-200 text-gray-900 focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500"
                                        : "bg-transparent border-transparent text-gray-900 pl-0"
                                        }`}
                                />
                            </div>
                        </div>

                        <div className="flex flex-col gap-2">
                            <label className="text-sm font-bold text-gray-900">Date of Birth</label>
                            <input
                                type="date"
                                value={formData.dob}
                                onChange={(e) => handleChange("dob", e.target.value)}
                                disabled={!isEditing}
                                className={`p-3 rounded-xl font-semibold transition-all outline-none w-full ${isEditing
                                    ? "bg-gray-50 border border-gray-200 text-gray-900 focus:ring-2 focus:ring-blue-500/20 focus:border-blue-500"
                                    : "bg-transparent border-transparent text-gray-900 pl-0"
                                    }`}
                            />
                        </div>
                    </div>

                    {isEditing && (
                        <div className="flex gap-3 pt-6 border-t border-gray-100 animate-fadeIn">
                            <button
                                onClick={handleCancel}
                                className="flex-1 py-3 bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-xl font-bold transition-all active:scale-[0.98] flex items-center justify-center gap-2"
                            >
                                <XMarkIcon className="w-5 h-5 stroke-2" />
                                Cancel
                            </button>
                            <button
                                onClick={handleSave}
                                className="flex-1 py-3 bg-black hover:bg-gray-800 text-white rounded-xl font-bold shadow-lg shadow-black/10 transition-all active:scale-[0.98] flex items-center justify-center gap-2"
                            >
                                <CheckIcon className="w-5 h-5 stroke-2" />
                                Save Changes
                            </button>
                        </div>
                    )}

                </div>
            </div>

            <Popup
                message={popupMessage}
                duration={1000}
                onClose={() => setPopupMessage("")}
            />
        </div>
    );
}