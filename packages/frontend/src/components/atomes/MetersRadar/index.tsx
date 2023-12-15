import "./style.scss";
import { Graph } from "../../../types/graph";
import { Radar } from "../../../utils/chartjs";

export default ({
  graphs,
}: {
  graphs: Graph[];
}) => {
  return (
    <div className="container-radar">
      <Radar
        data={{
          labels: graphs.map((graph) => `${graph.id}`),
          datasets: [
            {
              data: graphs.map((graph) => (graph.used / graph.total) * 100),
              backgroundColor: "rgba(255, 99, 132, 0.2)",
              borderColor: "rgba(255, 99, 132, 1)",
              borderWidth: 1,

              pointBackgroundColor: "rgb(255, 99, 132)",
              pointBorderColor: "#fff",
              pointHoverBackgroundColor: "#fff",
              pointHoverBorderColor: "rgb(255, 99, 132)",
            },
          ],
        }}
        options={{
          responsive: true,
          maintainAspectRatio: false,
          plugins: {
            legend: {
              display: false,
            },

            tooltip: {
              enabled: true,
              mode: "nearest",
              callbacks: {
                label: (context: any) => {
                  return " " + context.raw.toFixed(2) + "%";
                },
              },
            },
          },
          scales: {
            r: {
              ticks: {
                display: false,
              },
            },
          },
        }}
      />
    </div>
  );
};
