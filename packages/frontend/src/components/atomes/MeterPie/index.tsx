import "react";

import { Pie } from "../../../utils/chartjs";
import { getColor } from "../../../utils/color";

import "./style.scss";

/////////////////////////////////////////////////////////////////////////
// Component
/////////////////////////////////////////////////////////////////////////

type Props = {
  label: string;
  used: number;
  total?: number;
  width?: string;
  height?: string;
  showLegend?: boolean;
};

export default (params: Props) => {

  const used = (params.used / (params.total || 100)) * 100;

  /* if (params.used < 0 || params.used > 100) {
    return null;
  } */

  return (
    <div className="container-pie" style={{ width: params.width || '350px' , height: params.height || '350px' }}>
      <Pie
        data={{
          labels: ["Used", "Not used"],
          datasets: [
            {
              data: [used, 100 - used],
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
                  return " " + context.raw.toFixed(2) + "%";
                },
              },
            },
            legend: {
              display: params.showLegend !== undefined,
              position: "bottom",
              align: "center",
              labels: {
                filter: (item: any, _chart: any) => {
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
