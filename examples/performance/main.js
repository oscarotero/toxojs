performance.mark("start");

await new Promise((resolve) => setTimeout(resolve, 1000));

performance.mark("end");

const { duration } = performance.measure("Execution time", "start", "end");

console.log("Execution time:", duration, "ms");
