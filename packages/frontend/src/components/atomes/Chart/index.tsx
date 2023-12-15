import "react";

import { Bar } from "../../../utils/chartjs";
import { getColor } from "../../../utils/color";

import "./style.scss";
import { Graph } from "../../../types/graph";

/////////////////////////////////////////////////////////////////////////
// Component
/////////////////////////////////////////////////////////////////////////

type Props = {
  graphs: Graph[];
  suffix?: string;
  width?: string;
};

export default (params: Props) => {
  return (
    <div className="container-bar" style={{ width: params.width || "800px" }}>
      <Bar
        data={{
          labels: params.graphs.map((graph) => `${graph.id}`),
          datasets: [
            {
              label: "Usage",
              data: params.graphs.map((graph) => graph.used),
              backgroundColor: params.graphs.map((graph) => graph.color || getColor(graph.used)),
            },
          ],
        }}
        width={"100%"}
        height={"100%"}
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
                  return " " + context.raw.toFixed(2) + (params.suffix || "%");
                },
              },
            },
          },
        }}
      />
    </div>
  );
};
