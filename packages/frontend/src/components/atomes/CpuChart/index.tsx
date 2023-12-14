import "react";

import { Bar } from "../../../utils/chartjs";
import { getColor } from "../../../types/color";

import "./style.scss";
import { Graph } from "../../../types/graph";

/////////////////////////////////////////////////////////////////////////
// Component
/////////////////////////////////////////////////////////////////////////

type Props = {
  cpus: Graph[];
};

export default (params: Props) => {
  return (
    <div className="container-bar">
      <Bar
        data={{
          labels: params.cpus.map((cpu) => `CPU ${cpu.id}`),
          datasets: [
            {
              label: "Usage",
              data: params.cpus.map((cpu) => (cpu.used / cpu.total) * 100),
              backgroundColor: params.cpus.map((cpu) => getColor((cpu.used / cpu.total) * 100)),
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
                  return " " + context.raw + "%";
                },
              },
            },
          },
        }}
      />
    </div>
  );
};
