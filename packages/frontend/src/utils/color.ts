export const getColor = (p: number) => {
    // transparent to red based on p
    const r = Math.floor((255 * p) / 100);
    const g = Math.floor((255 * (70 - p)) / 100);
    return `rgba(${r},${g},0,0.5)`;
};