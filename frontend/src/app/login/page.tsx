"use client";
import { EyeIcon, EyeSlashIcon,ArrowLeftIcon} from "@heroicons/react/24/outline";
import Link from "next/link";
import React, { useState } from "react";
import { AuthCard } from '../components/AuthCard'
import { AuthInput } from '../components/AuthInput'
import { AuthButton } from '../components/AuthButton'
import Popup from "../components/Popup";

export default function LoginPage() {
  const [step, setStep] = useState(1);
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);
  const [popupMessage, setPopupMessage] = useState("");
  const [showPopup, setShowPopup] = useState(false);
  

  const handleContinue = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (username.trim()) {
      /* API CALL*/
      setStep(2);
    } else {
      alert("Please enter username");
    }
  };

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (password.trim()) {
      console.log("Logging in with:", { username, password });
      try {
            const response = await fetch("http://127.0.0.1:3001/api/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                username,
                password,
            }),
            });

            if (!response.ok) {
                throw new Error("Registration failed");
            }

            const data = await response.json();
            if (data.success){
                console.log("✅ Success:", data.message);
            }
            else{
                console.warn("⚠️ Failed:", data.message); 
                setPopupMessage(data.message);
                setShowPopup(true);
                
                setStep(1);
            }

        } catch (error) {
            console.error("Error:", error);
        }
    } else {
      alert("Please enter password");
    }
  };

  return (
    
      <AuthCard>
        <Popup
            message={popupMessage} 
            onClose={() => setPopupMessage("")}
        />
        {step === 2 && (
          <button
            type="button"
            onClick={() => setStep(1)}
            className="mb-4 text-sm font-medium text-indigo-600 hover:underline"
          >
          <ArrowLeftIcon className="h-5 w-6" />
          </button>
        )}

        <h1 className="mb-6 text-center text-2xl font-bold text-gray-900">
          Log in to fitness
        </h1>

        {step === 1 ? (
          <form onSubmit={handleContinue} className="space-y-4">
            <div>
              <label className="block text-m font-medium text-gray-800">
                Username
              </label>
              <AuthInput
                type="text"
                value={username}
                onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
                  setUsername(e.target.value)
                }
                placeholder="Enter username"
                required
              />
            </div>
            <AuthButton type="submit">Continue</AuthButton>

            <p className="mt-3 text-center text-sm text-gray-700">
              Don’t have an account?{" "}
              
              <Link
                href="/register"
                className="font-medium text-indigo-500 hover:underline"
              >
                Sign up
              </Link>
            </p>
          </form>
        ) : (
          <form onSubmit={handleSubmit} className="space-y-4">
            <div className="relative">
              <label className="block text-m font-medium text-gray-800">
                Password
              </label>
              <AuthInput
                type={showPassword ? "text" : "password"}
                value={password}
                onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
                  setPassword(e.target.value)
                }
                placeholder="Enter password"
                required
              />
              <button
                type="button"
                onClick={() => setShowPassword(!showPassword)}
                className="absolute right-3 top-1/2 -translate-y-[10%]"
              >
                {showPassword ? (
                  <EyeSlashIcon className="h-5 w-5 text-black " />
                ) : (
                  <EyeIcon className="h-5 w-5 text-black " />
                )}
                {/* {showPassword ? "🙈" : "👁️"} */}
              </button>
            </div>

            <p className="text-right text-sm">
              <Link
                href="/reset-pwd"
                className="font-medium text-indigo-500 hover:underline"
              >
                Forgot password?
              </Link>
            </p>

            <AuthButton type="submit">Sign In</AuthButton>
          </form>
        )}
      </AuthCard>

  );
}
