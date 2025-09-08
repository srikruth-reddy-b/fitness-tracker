
"use client";
import { EyeIcon, EyeSlashIcon,ArrowLeftIcon} from "@heroicons/react/24/outline";
import React, { useState } from "react";
import { AuthCard } from '../components/AuthCard'
import { AuthInput } from '../components/AuthInput'
import { AuthButton } from '../components/AuthButton'
import Link from "next/link";

const RegisterPage = () => {
    const [showPassword, setShowPassword] = useState(false);
    const [fullname, setFullname] = useState("");
    const [username, setUsername] = useState("");
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");
    const [confirmpassword, setConfirmPassword] = useState("");

    const handleRegister = (e:React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        //API CALL
        console.log(username,password,confirmpassword,email,fullname)
    }
    return (
        <AuthCard>
        <h1 className="mb-6 text-center text-2xl font-bold text-gray-900">Sign up for fitness</h1>
        <form onSubmit={handleRegister} className="space-y-4">
            <AuthInput type="text" placeholder="Enter Full Name" 
            value={fullname}
            onChange={(e:React.ChangeEvent<HTMLInputElement>) => setFullname(e.target.value)}required/>

            <AuthInput type="email" placeholder="Enter email"
            value={email}
            onChange={(e:React.ChangeEvent<HTMLInputElement>) => setEmail(e.target.value)} required />

            <AuthInput type ="text" placeholder='Enter username'
            value={username}
            onChange={(e:React.ChangeEvent<HTMLInputElement>) => setUsername(e.target.value)} required/>

            <AuthInput type="password" placeholder="Enter password" 
            value={password}
            onChange={(e:React.ChangeEvent<HTMLInputElement>) => setPassword(e.target.value)} required/>

            <AuthInput type={showPassword? "text" : "password"} className = "relative" placeholder = "Confirm password"
            value={confirmpassword}
            onChange={(e:React.ChangeEvent<HTMLInputElement>) => setConfirmPassword(e.target.value)} required/>

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
            <AuthButton type="submit">Register</AuthButton>
            <p className='mt-3 text-center text-sm text-gray-700'>
                Already have an account? {" "}
                <Link href ="/login" className=' text-medium text-indigo-500 hover:underline'>
                Sign in
                </Link>
            </p>
        </form>
        </AuthCard>
  )
}

export default RegisterPage
