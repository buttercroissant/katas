// Event loop
// - Synchronous operations comes before Asynchronous operations

Promise.resolve("").then((_) => console.log("A"));

setTimeout(() => console.log("B"), 0);

// Tick - every time the event loop takes a full trip
process.nextTick(() => console.log("C"));

console.log("D");
