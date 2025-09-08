import React from "react";

export const AuthCard = ({ children }: { children: React.ReactNode }) => (
  <div className="flex min-h-screen items-center justify-center">
    <div className="w-full max-w-sm rounded-2xl border-2 border-black p-8 shadow-2xl backdrop-blur-sm">
        {children}
    </div>
  </div>  
);
