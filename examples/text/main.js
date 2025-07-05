// Base64 encoding and decoding example
console.log(atob("SGVsbG8sIFdvcmxkIQ=="));
console.log(btoa("Hello, World!"));

// Text encoding and decoding example
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

// Stream example
const stream = new ReadableStream({
  start(controller) {
    controller.enqueue("Hello, ");
    controller.enqueue("World!");
    controller.close();
  },
});

const reader = stream.getReader();
reader.read().then(function processText({ value, done }) {
  if (!done) {
    console.log("Stream output:", value);
    return reader.read().then(processText);
  } else {
    console.log("Stream finished");
  }
});
