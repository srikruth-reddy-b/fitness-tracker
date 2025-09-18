import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  devIndicators : false,
  reactStrictMode: true, 
  env: {
    API_URL: 'http://127.0.0.1:3001/',
  },
};

export default nextConfig;
