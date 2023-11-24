import "react";

import { Bar } from "../../../utils/chartjs";

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
              backgroundColor: "rgb(255, 99, 132)",
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
          },
        }}
      />
    </div>
  );
};
