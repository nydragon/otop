import { Process } from "../types";
import { Graph } from "../types/graph";


export const extractData = (data: {
    [key: string]: any;
} | undefined): {
    pies: Graph[],
    cpus: Graph[],
    processes: Process[],
    timestamp: number;
    network: Graph[],
} | null => {
    if (!data) return null;
    const { memory, cpu, processes, misc, network, timestamp } = data;
    if (!memory || !cpu || !processes) {
        return null;
    }

    const { total: memtotal, active: memactive, swap_total: swaptotal, swap_free: swapfree } = memory;
    if (!memtotal || !memactive) {
        return null;
    }

    const cpus: Graph[] = cpu.map((cpu: any) => ({
        id: `CPU ${cpu?.processor}`,
        used: (cpu?.cpu_mhz / cpu?.bogomips) * 100,
        total: 0,
    }));

    let fprocesses: Process[] = [];

    for (const process of processes) {
        const { pid, user_name: user, state, priority, nice, vsize, rss, starttime, utime, stime, command } = process;
        //console.log(process);
        //if (!pid || !priority || !nice || !vsize || !rss || !starttime || !utime || !stime || !command) continue;
        const utime_s = utime / (misc ? misc.clk_tck : 100);
        const stime_s = stime / (misc ? misc.clk_tck : 100);
        const starttime_s = starttime / (misc ? misc.clk_tck : 100);

        const elapsed_s = misc?.uptime - starttime_s;
        const usage_s = utime_s + stime_s;
        const cpu = misc ? (usage_s * 100) / elapsed_s : 0;

        const date_time = new Date(0);
        date_time.setSeconds(starttime);

        fprocesses.push({
            pid,
            user,
            state,
            priority,
            nice,
            virt: vsize,
            resident: rss,
            cpu,
            mem: (rss * 100) / memtotal,
            time: date_time.getHours() + ":" + date_time.getMinutes() + ":" + date_time.getSeconds(),
            command,
        });
    }

    let n_sec = Object.entries(network.devices).reduce((acc, [_, value] : [string, any]) : any => {
        return [value.total_tx + acc[0], value.total_rx + acc[1]];
    }, [0, 0]);

    return {
        pies: [{
            id: 'Memory',
            used: memactive,
            total: memtotal,
        }, {
            id: 'Swap',
            used: swaptotal - swapfree,
            total: swaptotal,
        }, {
            id: 'CPU',
            used: cpus.reduce((acc, cpu) => acc + cpu.used, 0) / cpus.length,
            total: 100,
        }],
        cpus,
        processes: fprocesses,
        timestamp,
        network: [
            {
                id: 'Download',
                used: n_sec[0] / 10000000,
                total: 0,
                color: '#aa1d89',
            },
            {
                id: 'Upload',
                used: n_sec[1] / 10000000,
                total: 0,
                color: '#4a00fc',
            }
        ]
    };
}