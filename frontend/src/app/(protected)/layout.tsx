"use client";

import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import Sidebar from "../components/Sidebar";

export default function ProtectedLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const router = useRouter();
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const verify = async () => {
      try {
        const res = await fetch(
          `${process.env.API_URL}api/verify-token`,
          { credentials: "include" }
        );

        const data = await res.json();

        if (!res.ok || !data.success) {
          router.replace("/login");
          return;
        }

        setLoading(false);
      } catch {
        router.replace("/login");
      }
    };

    verify();
  }, [router]);

  if (loading) return <p>Loading...</p>;

  return (
    <div className="flex min-h-screen bg-gray-100">
        <Sidebar />
        <main className="flex-1 p-8">{children}</main>
    </div>
    );


}
