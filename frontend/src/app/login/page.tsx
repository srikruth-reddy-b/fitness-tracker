import React from 'react'

export default function LoginPage() {
  return (
    <div className="flex min-h-screen items-center justify-center">
      <div className="w-full max-w-md rounded-2xl bg-white/10 p-8 shadow-xl backdrop-blur-md dark:bg-black/30">
        <h2 className="mb-6 text-center text-2xl font-bold text-foreground">
          Welcome Back
        </h2>

        <form className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-foreground">
              Email
            </label>
            <input
              type="email"
              className="mt-1 w-full rounded-lg border border-gray-300 bg-transparent p-2 text-foreground outline-none focus:border-indigo-500 focus:ring-2 focus:ring-indigo-500"
              placeholder="you@example.com"
              required
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-foreground">
              Password
            </label>
            <input
              type="password"
              className="mt-1 w-full rounded-lg border border-gray-300 bg-transparent p-2 text-foreground outline-none focus:border-indigo-500 focus:ring-2 focus:ring-indigo-500"
              placeholder="••••••••"
              required
            />
          </div>

          <button
            type="submit"
            className="w-full rounded-lg bg-indigo-600 px-4 py-2 font-medium text-white shadow-lg transition hover:bg-indigo-700"
          >
            Sign In
          </button>
        </form>

        <p className="mt-6 text-center text-sm text-foreground/70">
          Don’t have an account?{" "}
          <a href="#" className="font-medium text-indigo-500 hover:underline">
            Sign up
          </a>
        </p>
      </div>
    </div>
  );
}
