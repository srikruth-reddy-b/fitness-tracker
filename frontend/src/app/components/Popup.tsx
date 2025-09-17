"use client";
import React, { useEffect } from "react";
interface PopupProps {
  message: string;
  duration?: number; 
  onClose: () => void;
}

const Popup: React.FC<PopupProps> = ({ message, duration = 2000, onClose }) => {
  useEffect(() => {
    if (message) {
      const timer = setTimeout(() => {
        onClose();
      }, duration);

      return () => clearTimeout(timer); 
    }
  }, [message, duration, onClose]);

  if (!message) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center ">
      <div className="bg-white p-6 rounded-xl shadow-lg text-center animate-fadeIn">
        <p className="text-gray-800">{message}</p>
      </div>
    </div>
  );
};

export default Popup;
