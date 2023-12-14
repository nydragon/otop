import { Process } from "../types";
import { Graph } from "../types/graph";


export const extractData = (data: {
    [key: string]: any;
} | undefined): {
    memory: Graph,
    cpus: Graph[],
    processes: Process[],
} | null => {
    if (!data) return null;
    const { memory, cpu, processes, misc } = data;
    if (!memory || !cpu || !processes) {
        return null;
    }

    const { total: memtotal, active: memactive } = memory;
    if (!memtotal || !memactive) {
        return null;
    }

    const cpus: Graph[] = cpu.map((cpu: any) => ({
        id: cpu?.processor,
        used: cpu?.cpu_mhz,
        total: cpu?.bogomips,
    }));

    let fprocesses: Process[] = [];

    for (const process of processes) {
        const { pid, user, priority, nice, vsize, rss, starttime, utime, stime, command } = process;
        console.log(process);
        //if (!pid || !priority || !nice || !vsize || !rss || !starttime || !utime || !stime || !command) continue;
        const utime_s = utime / 100;
        const stime_s = stime / 100;

        const elapsed_s = misc?.uptime - stime_s;
        const usage_s = utime_s + stime_s;
        const cpu = misc ? (usage_s * 100) / elapsed_s : 0;

        fprocesses.push({
            pid,
            user,
            priority,
            nice,
            virt: vsize,
            resident: rss,
            cpu,
            mem: (rss * 100) / memtotal,
            time: starttime,
            command,
        });
    }

    return {
        memory: {
            used: memactive,
            total: memtotal,
        },
        cpus,
        processes: fprocesses,
    };
}