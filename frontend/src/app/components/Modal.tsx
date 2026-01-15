"use client";
import React, { useEffect, useState } from "react";

interface ModalProps {
    isOpen: boolean;
    onClose: () => void;
    title?: string;
    children: React.ReactNode;
    maxWidth?: string;
}

const Modal: React.FC<ModalProps> = ({ isOpen, onClose, title, children, maxWidth = "max-w-md" }) => {
    const [show, setShow] = useState(isOpen);

    useEffect(() => {
        setShow(isOpen);
    }, [isOpen]);

    if (!show) return null;

    return (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm transition-opacity opacity-100">
            <div className={`bg-white rounded-2xl shadow-xl w-full ${maxWidth} mx-4 transform transition-all scale-100 p-6`}>
                <div className="flex justify-between items-center mb-4">
                    {title && <h3 className="text-xl font-bold text-gray-800">{title}</h3>}
                    <button
                        onClick={onClose}
                        className="text-gray-900 hover:text-gray-600 transition-colors"
                    >
                        <svg
                            className="w-6 h-6"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                        >
                            <path
                                strokeLinecap="round"
                                strokeLinejoin="round"
                                strokeWidth={2}
                                d="M6 18L18 6M6 6l12 12"
                            />
                        </svg>
                    </button>
                </div>
                <div className="text-gray-900">{children}</div>
            </div>
        </div>
    );
};

export default Modal;
