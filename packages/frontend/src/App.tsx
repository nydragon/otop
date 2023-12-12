import "react";

import "./App.scss";

import { faker } from "@faker-js/faker";
import { useEffect, useState } from "react";

import MeterPie from "./components/atomes/MeterPie";
import CpuChart from "./components/atomes/CpuChart";
import ProcessGrid from "./components/atomes/ProcessGrid";
import MetersRadar from "./components/atomes/MetersRadar";

import { generateProcess } from "./utils/faker";
import ProcessModal from "./components/atomes/ProcessModal";
import { Process } from "./types";
import useWebSocket from "./hooks/useWebSocket";

export default () => {
  const { sendMessage, lastMessage, reload, ready } = useWebSocket({
    url: "ws://localhost:3000/ws",
  });
  const [usedMemory, setUsedMemory] = useState(0); // 0

  const [processes, setProcesses] = useState<Process[]>(
    new Array(10).fill(0).map(() => generateProcess())
  ); // [
  const [selectedProcess, setSelectedProcess] = useState<Process | null>(null);

  useEffect(() => {
    let interval = setInterval(() => {
      ready ? sendMessage({ op: 1 }) : reload();
    }, 5000);

    return () => {
      clearInterval(interval);
    };
  }, [ready]);

  useEffect(() => {
    if (!lastMessage) return;
    console.log(lastMessage);
    if (lastMessage.op === 2) {
      let memory = lastMessage.d?.memory;
      if (memory) {
        let ratio = (memory?.active / memory?.total) * 100;
        setUsedMemory(ratio);
      }
    }
  }, [lastMessage]);

  const nbCpu = (faker.number.int() % 120) + 8;
  const cpusUsage = Array.from(Array(nbCpu).keys()).map((i) => ({
    id: i + 1,
    usage: faker.number.int() % 100,
  }));

  if (!ready) return <div>Connecting...</div>;

  return (
    <>
      {selectedProcess && (
        <ProcessModal
          close={() => setSelectedProcess(null)}
          process={selectedProcess}
        />
      )}
      <header>
        <img src="/logo.png" alt="logo" width={75} height={75} />
        <h1>Otop - Dashboard</h1>
        <h2>Last update 2min ago</h2>
      </header>
      <main>
        <div className="gl-container">
          <MetersRadar />
          <CpuChart cpus={cpusUsage} />
          <div className="meters-pie">
            {new Array("Memory", "CPU", "Swap", "Network").map(
              (label, index) => (
                <div
                  key={index}
                  style={{
                    gridArea: `${index >= 2 ? "2" : "1"} / ${
                      (index % 2) + 1
                    } / ${index >= 2 ? "3" : "2"} / ${(index % 2) + 2}`,
                  }}
                >
                  <MeterPie
                    label={label}
                    used={usedMemory}
                    width="150px"
                    height="150px"
                  />
                </div>
              )
            )}
          </div>
        </div>
        <div className="proc-container">
          <ProcessGrid
            OpenProcess={(pid: number) => {
              const process = processes.find((p) => p.pid === pid);
              if (process) {
                setSelectedProcess(process);
              }
            }}
            processes={processes}
          />
        </div>
      </main>
      <footer>
        <span>
          Made with ❤️ by{" "}
          <a href="https://limeal.fr" target="_blank">
            Limeal
          </a>{" "}
          and{" "}
          <a href="https://github.com/Nydragon" target="_blank">
            Nydragon
          </a>
        </span>
      </footer>
    </>
  );
};
