export type Process = {
    pid: number; // attr: pid
    user: string; // attr: user
    state: string;
    priority: number; // attr: priority
    nice: number; // attr: nice
    virt: number; // attr: vsize
    resident: number; // attr: rss
    cpu: number;
    mem: number;
    time: string; // attr: starttime
    command: string; // attr: command
};