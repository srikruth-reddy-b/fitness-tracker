import { useRouter } from "next/navigation";
import { useCallback } from "react";

export function useAuthFetch() {
    const router = useRouter();

    const authFetch = useCallback(async (input: RequestInfo | URL, init?: RequestInit) => {
        const res = await fetch(input, init);
        if (res.status === 401) {
            // Redirect to login
            router.push("/login");
            // Return a promise that never resolves to prevent downstream code (catch/finally) from running
            // while the page redirects. This prevents UI error flashes.
            return new Promise<Response>(() => { });
        }
        return res;
    }, [router]);

    return authFetch;
}
