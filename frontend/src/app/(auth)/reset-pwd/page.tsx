"use client";
import React, { useState } from "react";
import { AuthInput } from '../../components/AuthInput'
import { AuthButton } from '../../components/AuthButton'
import { ArrowLeftIcon, EyeIcon, EyeSlashIcon } from "@heroicons/react/24/outline";
import { useRouter } from "next/navigation";

const ResetPage = () => {
  const [showPassword, setShowPassword] = useState(false);
  const [username, setUserName] = useState("");
  const [password, setPassword] = useState("");
  const [confirmpassword, setConfirmPassword] = useState("");
  const [error, setError] = useState("");

  const router = useRouter();

  const handleReset = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setError("");

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
        throw new Error("Reset failed");
      }

      const data = await response.json();
      if (data.success) {
        console.log("", data.message);
        router.back();
      } else {
        setError(data.message || "Failed to reset password");
      }

    }
    catch (error) {
      console.error("Error:", error);
      setError("Something went wrong. Please try again.");
    }
  };

  return (
    <div className="flex min-h-screen bg-white">
      {/* Left Panel: Hero/Motivational */}
      <div className="hidden lg:flex w-1/2 bg-gray-900 relative overflow-hidden flex-col justify-between p-12 text-white">
        {/* Background Gradient */}
        <div className="absolute inset-0 bg-gradient-to-br from-blue-900/40 to-black z-0 pointer-events-none" />
        <div className="absolute inset-0 opacity-20 z-0 bg-[radial-gradient(ellipse_at_top_right,_var(--tw-gradient-stops))] from-indigo-500 via-gray-900 to-black"></div>

        <div className="relative z-10">
          <div className="flex items-center gap-2">
            <div className="h-8 w-8 bg-blue-600 rounded-lg"></div>
            <span className="text-2xl font-bold tracking-tight">FitTrack</span>
          </div>
        </div>

        <div className="relative z-10 flex-1 flex flex-col justify-center">
          <h1 className="text-6xl md:text-8xl font-black tracking-tighter leading-none mb-4 bg-clip-text text-transparent bg-gradient-to-b from-white to-white/50">
            RECOVER<br />
            ACCESS
          </h1>
          <p className="text-lg text-gray-400 font-medium max-w-sm">
            Don't worry, even the strongest reset sometimes.
          </p>
        </div>
      </div>

      {/* Right Panel: Form */}
      <div className="w-full lg:w-1/2 flex items-center justify-center p-8 bg-white text-gray-900">
        <div className="w-full max-w-md space-y-8">

          <div>
            <button
              onClick={() => router.back()}
              className="flex items-center gap-2 text-sm font-medium text-gray-500 hover:text-gray-900 mb-8 transition-colors"
            >
              <ArrowLeftIcon className="h-4 w-4" />
              Back
            </button>

            <h1 className="text-3xl font-bold tracking-tight text-gray-900">
              Reset Password
            </h1>
            <p className="mt-2 text-gray-500">
              Enter your credentials to set a new password
            </p>
          </div>

          {error && (
            <div className="p-3 text-sm text-red-500 bg-red-50 rounded-lg border border-red-100 font-medium">
              {error}
            </div>
          )}

          <form onSubmit={handleReset} className="space-y-6">
            <div className="space-y-4">
              <AuthInput type='text'
                value={username} onChange={(e: React.ChangeEvent<HTMLInputElement>) => setUserName(e.target.value)}
                placeholder='Enter username' required />

              <AuthInput type='password'
                value={password} onChange={(e: React.ChangeEvent<HTMLInputElement>) => setPassword(e.target.value)}
                placeholder='Enter New password' required />

              <div className="relative">
                <AuthInput type={showPassword ? "text" : "password"} className="relative pr-10"
                  value={confirmpassword} onChange={(e: React.ChangeEvent<HTMLInputElement>) => setConfirmPassword(e.target.value)}
                  placeholder='Confirm password' required />

                <button type="button"
                  className="absolute right-3 top-3 text-gray-400 hover:text-gray-600"
                  onClick={() => setShowPassword(!showPassword)}>
                  {showPassword ? (
                    <EyeSlashIcon className="h-5 w-5" />) : (<EyeIcon className="h-5 w-5" />)
                  }
                </button>
              </div>
            </div>

            <AuthButton type='submit' className="w-full py-3 bg-black hover:bg-gray-800 text-white rounded-xl font-bold transition-all">
              Reset Password
            </AuthButton>
          </form>
        </div>
      </div>
    </div>
  )
}

export default ResetPage
