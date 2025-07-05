const text = "Hello, world!";
const encoder = new TextEncoder();
const data = encoder.encode(text);

// Hash the data using SHA-256
const hashBuffer = await crypto.subtle.digest("SHA-256", data);

// Convert ArrayBuffer to hex string
const hashArray = Array.from(new Uint8Array(hashBuffer));
const hashHex = hashArray.map((b) => b.toString(16).padStart(2, "0")).join("");
console.log(`SHA-256 hash: ${hashHex}`);
