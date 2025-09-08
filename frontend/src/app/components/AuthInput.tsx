import React from "react";

type Props = React.InputHTMLAttributes<HTMLInputElement>;

export const AuthInput = (props: Props) => (
  <input
    {...props}
    className={`mt-1 w-full rounded-lg border border-gray-500 bg-transparent mb-2 p-2 outline-none text-black
               focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 hover:border-gray-700 placeholder:text-sm placeholder:text-gray-700 ${props.className}`}
  />
);
