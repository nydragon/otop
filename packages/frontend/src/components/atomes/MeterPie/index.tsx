import "react";

import { Pie } from "../../../utils/chartjs";
import { getColor } from "../../../types/color";

import "./style.scss";

/////////////////////////////////////////////////////////////////////////
// Component
/////////////////////////////////////////////////////////////////////////

type Props = {
  label: string;
  used: number;
  width?: string;
  height?: string;
  showLegend?: boolean;
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
                getColor(params.used),
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
          color: "white",
          plugins: {
            title: {
              display: params.label !== "",
              text: params.label,
              color: "white",
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
            legend: {
              display: params.showLegend !== undefined,
              position: "bottom",
              align: "center",
              labels: {
                filter: (item: any, chart: any) => {
                  return !item.text.includes("Not used");
                },
                color: "white",
                boxWidth: 10,
              }
            },
          },
        }}
      />
    </div>
  );
};
