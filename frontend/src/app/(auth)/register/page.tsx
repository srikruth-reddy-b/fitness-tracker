
"use client";
import { EyeIcon, EyeSlashIcon, ArrowLeftIcon } from "@heroicons/react/24/outline";
import React, { useState } from "react";
import { AuthCard } from '../../components/AuthCard'
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
        <AuthCard>
            <Popup
                message={popupMessage}
                onClose={() => setPopupMessage("")}
            />
            <h1 className="mb-6 text-center text-2xl font-bold text-gray-900">Sign up for fitness</h1>
            <form onSubmit={handleRegister} className="space-y-4">
                <AuthInput type="text" placeholder="Enter Full Name"
                    value={fullname}
                    onChange={(e: React.ChangeEvent<HTMLInputElement>) => setFullname(e.target.value)} required />

                <AuthInput type="email" placeholder="Enter email"
                    value={email}
                    onChange={(e: React.ChangeEvent<HTMLInputElement>) => setEmail(e.target.value)} required />

                <AuthInput type="text" placeholder='Enter username'
                    value={username}
                    onChange={(e: React.ChangeEvent<HTMLInputElement>) => setUsername(e.target.value)} required />

                <AuthInput type="password" placeholder="Enter password"
                    value={password}
                    onChange={(e: React.ChangeEvent<HTMLInputElement>) => setPassword(e.target.value)} required />

                <AuthInput type={showPassword ? "text" : "password"} className="relative" placeholder="Confirm password"
                    value={confirmpassword}
                    onChange={(e: React.ChangeEvent<HTMLInputElement>) => setConfirmPassword(e.target.value)} required />

                <button
                    type="button"
                    onClick={() => setShowPassword(!showPassword)}
                    className="absolute right-10 translate-y-[70%]"
                >
                    {showPassword ? (
                        <EyeSlashIcon className="h-5 w-5 text-black " />
                    ) : (
                        <EyeIcon className="h-5 w-5 text-black " />
                    )}
                    {/* {showPassword ? "🙈" : "👁️"} */}
                </button>

                <AuthInput type="number" placeholder="Weight (kg)"
                    value={weight}
                    onChange={(e: React.ChangeEvent<HTMLInputElement>) => setWeight(e.target.value)}
                />

                <AuthInput type="number" placeholder="Height (cm)"
                    value={height}
                    onChange={(e: React.ChangeEvent<HTMLInputElement>) => setHeight(e.target.value)}
                />

                <AuthInput type="date" placeholder="Date of Birth"
                    value={dob}
                    onChange={(e: React.ChangeEvent<HTMLInputElement>) => setDob(e.target.value)}
                />

                <AuthButton type="submit">Register</AuthButton>
                <p className='mt-3 text-center text-sm text-gray-700'>
                    Already have an account? {" "}
                    <Link href="/login" className=' text-medium text-indigo-500 hover:underline'>
                        Sign in
                    </Link>
                </p>
            </form>
        </AuthCard>
    )
}

export default RegisterPage
