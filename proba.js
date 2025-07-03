import foo from "./proba.js";

// performance.mark("start");

console.log(foo());
console.log(new URL("https://example.com"));
console.log(new URLSearchParams("key=value&foo=bar"));
console.log(new URLPattern({ pathname: "/path/:id" }).exec("/path/123"));

console.log(atob("SGVsbG8sIFdvcmxkIQ=="));
console.log(btoa("Hello, World!"));
console.log(
  new TextDecoder().decode(
    new Uint8Array([
      72,
      101,
      108,
      108,
      111,
      44,
      32,
      87,
      111,
      114,
      108,
      100,
      33,
    ]),
  ),
);
console.log(new TextEncoder().encode("Hello, World!"));

console.log(
  await crypto.subtle.digest(
    "SHA-256",
    new TextEncoder().encode("Hello, World!"),
  ),
);

console.log(
  new Request("https://example.com", {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  }),
);

// const response = fetch("http://httpforever.com/")

// performance.mark("end");
// console.log("Execution time:", performance.measure("Execution time", "start", "end").duration, "ms");

export default function bar() {
  return 3 + 4;
}

setTimeout(() => {
  console.log("Timeout executed after 1 second");
}, 1000);
