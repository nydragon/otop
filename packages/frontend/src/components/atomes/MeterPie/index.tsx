import "react";

import { Pie } from "../../../utils/chartjs";

import "./style.scss";

/////////////////////////////////////////////////////////////////////////
// Component
/////////////////////////////////////////////////////////////////////////

type Props = {
  label: string;
  used: number;
  width?: string;
  height?: string;
};

export default (params: Props) => {

  if (params.used < 0 || params.used > 100) {
    return null;
  }

  return (
    <div className="container-pie" style={{ width: params.width || '350px' , height: params.height || '350px' }}>
      <Pie
        data={{
          labels: ["Used", "Not used"],
          datasets: [
            {
              data: [params.used, 100 -  params.used],
              backgroundColor: [
                "rgb(255, 99, 132)",
                "rgb(54, 162, 235, 0)"
              ],
            },
          ],
        }}
        width={"100%"}
        height={"100%"}
        options={{
          responsive: true,
          maintainAspectRatio: false,
          plugins: {
            title: {
              display: true,
              text: params.label,
            },
            legend: {
              display: false,
            },
          },
        }}
      />
    </div>
  );
};
