"use client";
import { EyeIcon, EyeSlashIcon, ArrowLeftIcon } from "@heroicons/react/24/outline";
import Link from "next/link";
import React, { useState } from "react";
import { AuthInput } from '../../components/AuthInput'
import { AuthButton } from '../../components/AuthButton'
import Popup from "../../components/Popup";
import { useRouter } from "next/navigation";

export default function LoginPage() {
  const [step, setStep] = useState(1);
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const [popupMessage, setPopupMessage] = useState("");
  const [showPopup, setShowPopup] = useState(false);

  const router = useRouter();
  const handleContinue = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (username.trim()) {
      setStep(2);
    } else {
      alert("Please enter username");
    }
  };

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (!password.trim()) {
      alert("Please enter password");
      return;
    }

    try {
      const response = await fetch(`${process.env.API_URL}api/login`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ username, password }),
        credentials: "include",
      });

      const data = await response.json();
      if (!response.ok || !data.success) {
        setPopupMessage(data.message || "Login failed");
        setShowPopup(true);
        setStep(1);
        return;
      }

      router.push("/dashboard");
    } catch (error) {
      console.error("Error:", error);
      setPopupMessage("Something went wrong");
      setShowPopup(true);
    }
  };


  return (
    <div className="flex min-h-screen bg-white">
      {/* Left Panel: Hero/Motivational */}
      <div className="hidden lg:flex w-1/2 bg-gray-900 relative overflow-hidden flex-col justify-between p-12 text-white">
        {/* Background Gradient/Image Placeholder */}
        <div className="absolute inset-0 bg-gradient-to-br from-blue-900/40 to-black z-0 pointer-events-none" />
        {/* You can add a real Next.js Image here if you have one. For now, a CSS pattern works great. */}
        <div className="absolute inset-0 opacity-20 z-0 bg-[radial-gradient(ellipse_at_top_right,_var(--tw-gradient-stops))] from-indigo-500 via-gray-900 to-black"></div>

        <div className="relative z-10">
          <div className="flex items-center gap-2">
            <img src="/auth-logo.png" alt="Logo" className="h-8 w-8 rounded-lg" />
            <span className="text-2xl font-bold tracking-tight">Fitness Tracker</span>
          </div>
        </div>

        <div className="relative z-10 flex-1 flex flex-col justify-center">
          <h1 className="text-6xl md:text-8xl font-black tracking-tighter leading-none mb-4 bg-clip-text text-transparent bg-gradient-to-b from-white to-white/50">
            TIME<br />
            FOR<br />
            FITNESS
          </h1>
          <p className="text-lg text-gray-400 font-medium max-w-sm">
            Track your progress. Crush your goals. The best time to start is now.
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
              {step === 1 ? "Welcome back" : `Hi, ${username}`}
            </h1>
            <p className="mt-2 text-gray-500">
              {step === 1 ? "Enter your username to continue" : "Enter your password to sign in"}
            </p>
          </div>

          {step === 2 && (
            <button
              type="button"
              onClick={() => setStep(1)}
              className="flex items-center text-sm font-medium text-blue-600 hover:text-blue-500 transition-colors"
            >
              <ArrowLeftIcon className="h-4 w-4 mr-1" />
              Back
            </button>
          )}

          {step === 1 ? (
            <form onSubmit={handleContinue} className="space-y-6">
              <div>
                <label className="block text-sm font-bold text-gray-900 mb-2">
                  Username
                </label>
                <AuthInput
                  type="text"
                  value={username}
                  onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
                    setUsername(e.target.value)
                  }
                  placeholder="Username"
                  required
                  className="!bg-gray-50 !border-gray-200 focus:!border-blue-600 focus:!ring-blue-600/20"
                  autoFocus
                />
              </div>
              <AuthButton type="submit" className="w-full py-3 bg-black hover:bg-gray-800 text-white rounded-xl font-bold transition-all">
                Continue
              </AuthButton>

              <p className="text-center text-sm text-gray-500">
                Don’t have an account?{" "}
                <Link href="/register" className="font-bold text-blue-600 hover:underline">
                  Sign up
                </Link>
              </p>
            </form>
          ) : (
            <form onSubmit={handleSubmit} className="space-y-6">
              <div>
                <label className="block text-sm font-bold text-gray-900 mb-2">
                  Password
                </label>
                <div className="relative">
                  <AuthInput
                    type={showPassword ? "text" : "password"}
                    value={password}
                    onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
                      setPassword(e.target.value)
                    }
                    placeholder="••••••••"
                    required
                    className="!bg-gray-50 !border-gray-200 focus:!border-blue-600 focus:!ring-blue-600/20 pr-10"
                    autoFocus
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
                <div className="flex justify-end mt-2">
                  <Link
                    href="/reset-pwd"
                    className="text-sm font-medium text-blue-600 hover:underline"
                  >
                    Forgot password?
                  </Link>
                </div>
              </div>

              <AuthButton type="submit" className="w-full py-3 bg-black hover:bg-gray-800 text-white rounded-xl font-bold transition-all">
                Sign In
              </AuthButton>
            </form>
          )}
        </div>
      </div>
    </div>
  );
}
