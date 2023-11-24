import "react";
import MeterPie from "./components/atomes/MeterPie";
import { faker } from "@faker-js/faker";
import CpuChart from "./components/atomes/CpuChart";
import "./App.scss";
import ProcessGrid from "./components/atomes/ProcessGrid";
import MetersRadar from "./components/atomes/MetersRadar";

export default () => {
  const t = faker.number.int() % 100;
  const nbCpu = (faker.number.int() % 120) + 8;
  const cpusUsage = Array.from(Array(nbCpu).keys()).map((i) => ({
    id: i + 1,
    usage: faker.number.int() % 100,
  }));

  return (
    <>
      <header>
        <h1>Otop - Dashboard</h1>
        <h2>Last update 2min ago</h2>
      </header>
      <main>
        <div className="gl-container">
          <MetersRadar />
          <CpuChart cpus={cpusUsage} />
          <div className="meters-pie">
            {new Array("Memory", "CPU", "Swap", "Network").map((label, index) => (
              <div key={index} style={{gridArea: `${index >= 2 ? '2' : '1'} / ${(index % 2) + 1} / ${index >= 2 ? '3' : '2'} / ${(index % 2) + 2}`}}>
                <MeterPie label={label} used={t} width="150px" height="150px" />
              </div>
            ))}
          </div>
        </div>
        <div className="proc-container">
          <ProcessGrid
            processes={new Array(10).fill(0).map(() => ({
              pid: faker.number.int(),
              user: faker.internet.userName(),
              priority: faker.number.int() % 100,
              nice: faker.number.int({ min: -20, max: 20 }),
              virt: faker.number.int(),
              resident: faker.number.int(),
              share: faker.number.int(),
              cpu: faker.number.int() % 100,
              mem: faker.number.int() % 100,
              time: faker.date.recent().getTime(),
              command: faker.lorem.word(),
            }))}
          />
        </div>
      </main>
      <footer>
        <span>
          Made with ❤️ by{" "}
          <a href="https://limeal.fr" target="_blank">
            Limeal
          </a>
        </span>
      </footer>
    </>
  );
};
