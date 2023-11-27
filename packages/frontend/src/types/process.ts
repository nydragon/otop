export type Process = {
    name: string;
    pid: number;
    user: string;
    priority: number;
    nice: number;
    virt: number;
    resident: number;
    share: number;
    cpu: number;
    mem: number;
    time: number;
    command: string;
};