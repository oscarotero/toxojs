const handler = await navigator.storage.getDirectory();

// Get or create the data directory
const data = await handler.getDirectoryHandle("data", { create: true });

// Get or create a file handle for "example.txt"
const example = await data.getFileHandle("example.txt", { create: true });

// Create a writable stream to write to the file
const writer = await example.createWritable();
await writer.write("Hello, Toxo!");
await writer.close();

// Read the file back
const file = await example.getFile();
const contents = await file.text();
console.log("File contents:", contents);
