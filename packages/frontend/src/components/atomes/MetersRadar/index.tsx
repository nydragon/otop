import "./style.scss";

import { faker } from "@faker-js/faker";

import { Radar } from "../../../utils/chartjs";

export default () => {
  return (
    <div className="container-radar">
      <Radar
        data={{
          labels: ["Memory", "CPU", "Swap", "Disk", "Network"],
          datasets: [
            {
              data: new Array(5).fill(0).map(() => faker.number.int() % 100),
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
