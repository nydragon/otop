import "react";

import "./App.scss";

import { useEffect, useState } from "react";

import MeterPie from "./components/atomes/MeterPie";
import CpuChart from "./components/atomes/Chart";
import ProcessGrid from "./components/atomes/ProcessGrid";
import ProcessModal from "./components/atomes/ProcessModal";
import { Process } from "./types";
import useWebSocket from "./hooks/useWebSocket";
import { Graph } from "./types/graph";
import { extractData } from "./utils/otop";
import Waiting from "./components/molecules/Waiting";

export default () => {
  const { sendMessage, lastMessage, reload, ready } = useWebSocket({
    url: "ws://localhost:3000/ws",
  });
  const [lastHBUpdate, setLastHBUpdate] = useState(0.0); // 0
  const [lastHB, setLastHB] = useState(0); // 0

  const [lastDUpdate, setLastDUpdate] = useState(0.0); // 0
  const [lastD, setLastD] = useState(0); // 0

  const [pies, setPies] = useState<Graph[]>([]); // []
  const [cpususage, setCpusUsage] = useState<Graph[]>([]);
  const [networkusage, setNetworkUsage] = useState<Graph[]>([]);

  const [processes, setProcesses] = useState<Process[]>([]); // [
  const [selectedProcess, setSelectedProcess] = useState<number | undefined>();

  useEffect(() => {
    //ready ? sendMessage({ op: 1 }) : reload();
    const interval = setInterval(() => {
      ready ? sendMessage({ op: 1 }) : reload();
    }, 2000);

    return () => {
      clearInterval(interval);
    };
  }, [ready]);

  useEffect(() => {
    const interval = setInterval(() => {
      setLastHBUpdate(
        lastHB > 0 ? (Math.floor(Date.now()) - lastHB) / 1000.0 : 0
      );
      setLastDUpdate(lastD > 0 ? (Math.floor(Date.now()) - lastD) / 1000.0 : 0);
    }, 50);

    return () => {
      clearInterval(interval);
    };
  }, [lastHB]);

  useEffect(() => {
    if (!lastMessage) return;
    if (lastMessage.op === 2) {
      const data = extractData(lastMessage?.d);
      if (!data) return;
      console.log(data);
      setPies(data.pies);
      setCpusUsage(data.cpus);
      setProcesses(data.processes);
      setLastD(data.timestamp);
      setNetworkUsage(data.network);
    } else if (lastMessage.op === 11) {
      const lastHB = lastMessage.d?.last_heartbeat;
      if (lastHB) setLastHB(lastHB);
    }
  }, [lastMessage]);

  if (!ready) return <Waiting />;

  return (
    <>
      {selectedProcess && processes.find((p) => p.pid === selectedProcess) && (
        <ProcessModal
          kill={(pid: number) => sendMessage({ op: 3, d: { pid, signal: 9 } })}
          close={() => setSelectedProcess(undefined)}
          process={processes.find((p) => p.pid === selectedProcess)}
        />
      )}
      <header>
        <img src="/logo.png" alt="logo" width={75} height={75} />
        <h1>Otop - Dashboard</h1>
        <h2>
          Last heartbeat was {lastHBUpdate}s ago / Last update was {lastDUpdate}
          s ago
        </h2>
      </header>
      <main>
        <div className="gl-container">
          {/* <MetersRadar graphs={pies} /> */}
          <CpuChart graphs={networkusage} suffix="MB/s" width="350px" />
          <CpuChart graphs={cpususage} />
          <ul className="meters-pie">
            {pies.map((pie, index) => (
              <li
                key={index}
                style={{
                  gridArea: `${index >= 2 ? "2" : "1"} / ${(index % 2) + 1} / ${
                    index >= 2 ? "3" : "2"
                  } / ${(index % 2) + 2}`,
                }}
              >
                <MeterPie
                  label={pie.id + ""}
                  used={pie.used}
                  total={pie.total}
                  width="150px"
                  height="150px"
                />
              </li>
            ))}
          </ul>
        </div>
        <div className="proc-container">
          <ProcessGrid
            OpenProcess={(pid: number) => setSelectedProcess(pid)}
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
