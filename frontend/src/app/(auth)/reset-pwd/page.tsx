"use client";
import React, { useState } from "react";
import { AuthCard } from '../../components/AuthCard'
import { AuthInput } from '../../components/AuthInput'
import { AuthButton } from '../../components/AuthButton'
import { ArrowLeftIcon, EyeIcon, EyeSlashIcon } from "@heroicons/react/24/outline";
import { useRouter } from "next/navigation";

const ResetPage = () => {
  const [showPassword, setShowPassword] = useState(false);
  const [username, setUserName] = useState("");
  const [password, setPassword] = useState("");
  const [confirmpassword, setConfirmPassword] = useState("");

  const router = useRouter();
  const handleLogin = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    //API CALL
    try {
      const response = await fetch(process.env.API_URL + 'api/forgot-password', {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          username,
          password,
          confirmpassword
        })
      });
      if (!response.ok) {
        throw new Error("Registration failed");
      }

      const data = await response.json();
      if (data.success) {
        console.log("", data.message);
        router.back();
      }
      else {

      }

    }
    catch (error) {
      console.error("Error:", error);
    }
  };

  return (
    <AuthCard>
      <form onSubmit={handleLogin}>
        <button
          type="button"
          onClick={() => router.back()}
          className="mb-4 text-sm font-medium text-indigo-600 hover:underline"
        >
          <ArrowLeftIcon className="h-5 w-6" />
        </button>
        <h1 className='mb-6 text-center text-2xl font-bold text-gray-900'>
          Reset Password
        </h1>
        <AuthInput type='text'
          value={username} onChange={(e: React.ChangeEvent<HTMLInputElement>) => setUserName(e.target.value)}
          placeholder='Enter username' required />

        <AuthInput type='password'
          value={password} onChange={(e: React.ChangeEvent<HTMLInputElement>) => setPassword(e.target.value)}
          placeholder='Enter New password' required />

        <AuthInput type='password' className="relative"
          value={confirmpassword} onChange={(e: React.ChangeEvent<HTMLInputElement>) => setConfirmPassword(e.target.value)}
          placeholder='Confirm password' required />

        <button type="button"
          className="absolute right-10 translate-y-[70%]"
          onClick={() => setShowPassword(!showPassword)}>
          {showPassword ? (
            <EyeSlashIcon className="h-5 w-5 text-black " />) : (<EyeIcon className="h-5 w-5 text-black " />)
          }
        </button>

        <AuthButton type='submit'>Reset Password</AuthButton>
      </form>
    </AuthCard>
  )
}

export default ResetPage
