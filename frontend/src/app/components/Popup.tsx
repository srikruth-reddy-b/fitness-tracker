"use client";
import React, { useEffect, useState } from "react";

interface PopupProps {
  message: string;
  duration?: number; 
  onClose: () => void;
}

const Popup: React.FC<PopupProps> = ({ message, duration = 2000, onClose }) => {
  const [progress, setProgress] = useState(100);

  useEffect(() => {
  if (!message) return;

  setProgress(100);

  const interval = 20; 
  const step = 100 / (duration / interval);

  const timer = setInterval(() => {
    setProgress((prev) => {
      if (prev <= 0) {
        clearInterval(timer);
        setTimeout(() => {
          onClose();
        }, 0);

        return 0;
      }
      return prev - step;
    });
  }, interval);

  return () => clearInterval(timer);
}, [message, duration, onClose]);

  if (!message) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      <div className="bg-white p-6 rounded-xl shadow-lg text-center animate-fadeIn relative w-[300px] h-[125px] flex flex-col justify-center">
        <p className="text-gray-800">{message}</p>
        
        {/* Progress bar */}
        <div className="absolute bottom-0 left-0 w-full h-1 bg-gray-200 rounded-b-xl overflow-hidden">
          <div
            className="h-full bg-blue-500 transition-all duration-100 ease-linear"
            style={{ width: `${progress}%` }}
          />
        </div>
      </div>
    </div>
  );
};

export default Popup;
