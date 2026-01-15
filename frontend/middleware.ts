// src/middleware.ts
import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";

export async function middleware(req: NextRequest) {
  const url = req.nextUrl.clone();
  const token = req.cookies.get("token")?.value;

  // Exclude login and static files from auth
  if (url.pathname.startsWith("/login") || url.pathname.startsWith("/_next") || url.pathname === "/favicon.ico") {
    return NextResponse.next();
  }

  // No token → redirect to login
  if (!token) {
    url.pathname = "/login";
    return NextResponse.redirect(url);
  }

  // Optional: verify token with backend API
  try {
    const res = await fetch(`${process.env.API_URL}/verify-token`, {
      method: "GET",
      headers: { "Content-Type": "application/json" },
      // include cookies if backend needs it
      credentials: "include",
    });

    const data = await res.json();

    if (!data.valid) {
      url.pathname = "/login";
      return NextResponse.redirect(url);
    }

    // Token valid → continue
    return NextResponse.next();
  } catch (err) {
    console.error("Token verification error:", err);
    url.pathname = "/login";
    return NextResponse.redirect(url);
  }
}

export const config = {
  matcher: ["/((?!login|_next/static|favicon.ico).*)"], // protect all routes except login and static files
};
