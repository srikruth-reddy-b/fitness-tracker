import React from "react";

type Props = React.ButtonHTMLAttributes<HTMLButtonElement>;

export const AuthButton = ({ children, ...props }: Props) => (
  <button
    {...props}
    className="w-full rounded-lg bg-indigo-600 px-4 py-2 font-medium text-white shadow-lg transition hover:bg-indigo-700"
  >
    {children}
  </button>
);
