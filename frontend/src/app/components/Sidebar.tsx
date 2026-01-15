"use client";

import Link from "next/link";
import { useRouter, usePathname } from "next/navigation";
import {
  Squares2X2Icon,
  ClipboardDocumentListIcon,
  UserCircleIcon,
  ClockIcon,
  ChartBarIcon,
  ArrowRightOnRectangleIcon
} from "@heroicons/react/24/outline";

const Sidebar = () => {
  const pathname = usePathname();
  const router = useRouter();

  const handleLogout = () => {
    // In a real app, calls /api/logout to clear HttpOnly cookie
    router.push("/login");
  };

  const navItems = [
    { name: "Dashboard", path: "/dashboard", icon: Squares2X2Icon },
    { name: "Log Workout", path: "/log", icon: ClipboardDocumentListIcon },
    { name: "Records", path: "/records", icon: ClockIcon },
    { name: "Profile", path: "/profile", icon: UserCircleIcon },
  ];

  return (
    <aside
      className="
        w-72 
        bg-white
        m-4 ml-0 rounded-l-none
        p-6
        rounded-2xl
        shadow-[4px_0_20px_rgba(0,0,0,0.05)]
        border-r border-gray-100
        sticky
        top-4
        h-[calc(100vh-2rem)]
        flex flex-col
      "
    >
      <div className="flex items-center gap-3 mb-10 px-2">
        <div className="w-8 h-8 bg-black rounded-lg flex items-center justify-center text-white">
          <ChartBarIcon className="w-5 h-5" />
        </div>
        <h2 className="text-xl font-bold text-gray-900 tracking-tight">Fitness Tracker</h2>
      </div>

      <nav className="flex flex-col gap-2">
        {navItems.map((item) => {
          const isActive = pathname === item.path;
          const Icon = item.icon;

          return (
            <Link
              key={item.path}
              href={item.path}
              className={`
                        flex items-center gap-3 px-4 py-3 rounded-xl transition-all duration-200 group
                        ${isActive
                  ? "bg-black text-white shadow-lg shadow-black/10 font-bold"
                  : "text-gray-500 font-medium hover:bg-gray-50 hover:text-gray-900"
                }
                    `}
            >
              <Icon className={`w-6 h-6 ${isActive ? "stroke-2" : "stroke-[1.5px] group-hover:scale-105 transition-transform"}`} />
              {item.name}
            </Link>
          );
        })}
      </nav>

      <div className="mt-auto pt-6 border-t border-gray-100">
        <button
          onClick={handleLogout}
          className="flex items-center gap-3 px-4 py-3 rounded-xl transition-all duration-200 text-gray-500 font-medium hover:bg-red-50 hover:text-red-600 w-full text-left group"
        >
          <ArrowRightOnRectangleIcon className="w-6 h-6 stroke-[1.5px] group-hover:scale-105 transition-transform" />
          Logout
        </button>
      </div>
    </aside>
  );
};

export default Sidebar;
