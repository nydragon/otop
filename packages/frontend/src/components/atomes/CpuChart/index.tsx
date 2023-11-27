import "react";

import { Bar } from "../../../utils/chartjs";
import { getColor } from "../../../types/color";

import "./style.scss";

/////////////////////////////////////////////////////////////////////////
// Component
/////////////////////////////////////////////////////////////////////////

type Props = {
  cpus: Array<{
    id: number;
    usage: number;
  }>;
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
              data: params.cpus.map((cpu) => cpu.usage),
              backgroundColor: params.cpus.map((cpu) => getColor(cpu.usage)),
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
