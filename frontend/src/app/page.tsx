"use client"
import { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import Home from "./protected/page";

export default function Page() {
  const router = useRouter();
  const [loading, setLoading] = useState(true);
  const [username, setUsername] = useState<string | null>(null);

  useEffect(() => {
    const verify = async () => {
      try {
        const res = await fetch(`${process.env.API_URL}api/verify-token`, {
            method: "GET",
            credentials: "include",
        });

        const data = await res.json();
        console.log(data);
        if (!res.ok || !data.success) {
          router.push("/login"); 
          return;
        }

        setUsername(data.username);
        setLoading(false);
      } catch (err) {
        console.error(err);
        router.push("/login");
      }
    };

    verify();
  }, [router]);

  if (loading) return <p>Loading...</p>;

  return (
    <div>
    <Home/>
    </div>
  );
}
