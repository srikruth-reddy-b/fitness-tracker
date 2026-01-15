"use client";
import { EyeIcon, EyeSlashIcon } from "@heroicons/react/24/outline";
import React, { useState } from "react";
import { AuthInput } from '../../components/AuthInput'
import { AuthButton } from '../../components/AuthButton'

import Link from "next/link";
import { useRouter } from "next/navigation";
import Popup from "../../components/Popup";


const RegisterPage = () => {
    const [showPassword, setShowPassword] = useState(false);
    const [fullname, setFullname] = useState("");
    const [username, setUsername] = useState("");
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");
    const [confirmpassword, setConfirmPassword] = useState("");
    const [weight, setWeight] = useState("");
    const [height, setHeight] = useState("");
    const [dob, setDob] = useState("");
    const [popupMessage, setPopupMessage] = useState("");
    const [showPopup, setShowPopup] = useState(false);
    const router = useRouter();

    const handleRegister = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        try {
            const response = await fetch(process.env.API_URL + 'api/register', {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    fullname,
                    username,
                    email,
                    password,
                    confirmpassword,
                    weight: parseFloat(weight.toString()),
                    height: parseFloat(height.toString()),
                    dob
                }),
            });

            if (!response.ok) {
                throw new Error("Registration failed");
            }

            const data = await response.json();
            console.log("Server message: ", data.message);
            if (data.success) {
                router.push("/login")
            }
            else {
                setPopupMessage(data.message);
                setShowPopup(true);
            }

        } catch (error) {
            console.error("Error:", error);
        }
    };

    return (
        <div className="flex min-h-screen bg-white">
            {/* Left Panel: Hero/Motivational */}
            <div className="hidden lg:flex w-1/2 bg-gray-900 relative overflow-hidden flex-col justify-between p-12 text-white">
                {/* Background Gradient */}
                <div className="absolute inset-0 bg-gradient-to-br from-purple-900/40 to-black z-0 pointer-events-none" />
                <div className="absolute inset-0 opacity-20 z-0 bg-[radial-gradient(ellipse_at_top_right,_var(--tw-gradient-stops))] from-purple-500 via-gray-900 to-black"></div>

                <div className="relative z-10">
                    <div className="flex items-center gap-2">
                        <img src="/auth-logo.png" alt="Logo" className="h-8 w-8 rounded-lg" />
                        <span className="text-2xl font-bold tracking-tight">FitTrack</span>
                    </div>
                </div>

                <div className="relative z-10 flex-1 flex flex-col justify-center">
                    <h1 className="text-6xl md:text-8xl font-black tracking-tighter leading-none mb-4 bg-clip-text text-transparent bg-gradient-to-b from-white to-white/50">
                        JOIN<br />
                        THE<br />
                        MOVEMENT
                    </h1>
                    <p className="text-lg text-gray-400 font-medium max-w-sm">
                        Start your journey today. Your future self will thank you.
                    </p>
                </div>
            </div>

            {/* Right Panel: Form */}
            <div className="w-full lg:w-1/2 flex items-center justify-center p-8 bg-white text-gray-900">
                <div className="w-full max-w-md space-y-8">
                    <Popup
                        message={popupMessage}
                        onClose={() => setPopupMessage("")}
                    />

                    <div className="text-center lg:text-left">
                        <h1 className="text-3xl font-bold tracking-tight text-gray-900">
                            Create an account
                        </h1>
                        <p className="mt-2 text-gray-500">
                            Enter your details to get started
                        </p>
                    </div>

                    <form onSubmit={handleRegister} className="space-y-4">
                        <div className="grid grid-cols-1 gap-4">
                            <AuthInput type="text" placeholder="Full Name"
                                value={fullname}
                                onChange={(e: React.ChangeEvent<HTMLInputElement>) => setFullname(e.target.value)} required />

                            <AuthInput type="email" placeholder="Email address"
                                value={email}
                                onChange={(e: React.ChangeEvent<HTMLInputElement>) => setEmail(e.target.value)} required />

                            <AuthInput type="text" placeholder='Username'
                                value={username}
                                onChange={(e: React.ChangeEvent<HTMLInputElement>) => setUsername(e.target.value)} required />

                            <div className="relative">
                                <AuthInput type={showPassword ? "text" : "password"} placeholder="Password"
                                    value={password}
                                    onChange={(e: React.ChangeEvent<HTMLInputElement>) => setPassword(e.target.value)} required
                                    className="pr-10"
                                />
                                <button
                                    type="button"
                                    onClick={() => setShowPassword(!showPassword)}
                                    className="absolute right-3 top-3 text-gray-400 hover:text-gray-600"
                                >
                                    {showPassword ? (
                                        <EyeSlashIcon className="h-5 w-5" />
                                    ) : (
                                        <EyeIcon className="h-5 w-5" />
                                    )}
                                </button>
                            </div>

                            <AuthInput type="password" placeholder="Confirm password"
                                value={confirmpassword}
                                onChange={(e: React.ChangeEvent<HTMLInputElement>) => setConfirmPassword(e.target.value)} required />

                            <div className="grid grid-cols-2 gap-4">
                                <AuthInput type="number" placeholder="Weight (kg)"
                                    value={weight}
                                    onChange={(e: React.ChangeEvent<HTMLInputElement>) => setWeight(e.target.value)}
                                />

                                <AuthInput type="number" placeholder="Height (cm)"
                                    value={height}
                                    onChange={(e: React.ChangeEvent<HTMLInputElement>) => setHeight(e.target.value)}
                                />
                            </div>

                            <div className="space-y-1">
                                <label className="text-xs font-bold text-gray-500 uppercase">Date of Birth</label>
                                <AuthInput type="date"
                                    value={dob}
                                    onChange={(e: React.ChangeEvent<HTMLInputElement>) => setDob(e.target.value)}
                                />
                            </div>
                        </div>

                        <AuthButton type="submit" className="w-full py-3 bg-black hover:bg-gray-800 text-white rounded-xl font-bold transition-all mt-6">
                            Register
                        </AuthButton>

                        <p className='mt-4 text-center text-sm text-gray-500'>
                            Already have an account? {" "}
                            <Link href="/login" className='font-bold text-purple-600 hover:underline'>
                                Sign in
                            </Link>
                        </p>
                    </form>
                </div>
            </div>
        </div>
    )
}

export default RegisterPage
