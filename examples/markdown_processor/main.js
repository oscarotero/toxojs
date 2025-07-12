import "markdown-it";

const md = markdownit();

const folder = await navigator.storage.getDirectory();
const input = await folder.getDirectoryHandle("input");
const output = await folder.getDirectoryHandle("output", { create: true });

// Iterate through all files in the input directory
for await (const entry of input.values()) {
  if (entry.kind === "file" && entry.name.endsWith(".md")) {
    const file = await entry.getFile();
    const text = await file.text();

    const processedText = md.render(text);

    const filename = entry.name.replace(".md", ".html");
    const out = await output.getFileHandle(filename, { create: true });
    const writable = await out.createWritable();
    await writable.write(processedText);
    await writable.close();
  }
}
