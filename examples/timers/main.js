const interval = setInterval(() => {
  console.log("This message is logged every second");
}, 1000);

setTimeout(() => {
  console.log("This message is logged after 5 seconds");
  console.log("Clearing the interval now");
  clearInterval(interval);
}, 5000);
