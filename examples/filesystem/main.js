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
console.log(file.name, file.size, file.type);
console.log(file.lastModified);

const reader = new FileReader();

reader.onload = (event) => {
  console.log("File content:", event.target.result);
};
reader.onerror = (error) => {
  console.error("Error reading file:", error);
};
reader.onloadend = () => {
  console.log("File reading completed.");
};
reader.readAsText(file);
