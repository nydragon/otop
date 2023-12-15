import * as ChartJS from "chart.js";
import { Bar, Pie, Radar } from "react-chartjs-2";

ChartJS.Chart.register(
    ChartJS.CategoryScale,
    ChartJS.LinearScale,
    ChartJS.RadialLinearScale,
    ChartJS.PointElement,
    ChartJS.LineElement,
    ChartJS.BarElement,
    ChartJS.Filler,
    ChartJS.Title,
    ChartJS.Tooltip,
    ChartJS.Legend,
    ChartJS.ArcElement
);

export { Bar, Pie, Radar };
export default ChartJS;