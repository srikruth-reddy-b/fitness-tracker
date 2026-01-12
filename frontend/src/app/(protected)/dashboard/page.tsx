import HeatmapCard from "@/app/components/Heatmap";
import ProgressChart from "@/app/components/ProgressChart";
import VolumeBars from "@/app/components/VolumeBars";

export default function DashboardPage() {
  return (
    <div className="flex flex-col justify-center items-center gap-6">
      <HeatmapCard />
      <ProgressChart />
      <VolumeBars />
    </div>
  );
}
