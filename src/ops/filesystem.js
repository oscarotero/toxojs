import { primordials } from "ext:core/mod.js";
import * as console from "ext:deno_console/01_console.js";
import * as webidl from "ext:deno_webidl/00_webidl.js";
import * as fs from "ext:deno_fs/30_fs.js";
import { WritableStream } from "ext:deno_web/06_streams.js";
import { SeekMode } from "ext:deno_io/12_io.js";
import { op_get_location } from "ext:core/ops";

export const errors = {
  INVALID: ["seeking position failed.", "InvalidStateError"],
  GONE: [
    "A requested file or directory could not be found at the time an operation was processed.",
    "NotFoundError",
  ],
  MISMATCH: [
    "The path supplied exists, but was not an entry of requested type.",
    "TypeMismatchError",
  ],
  MOD_ERR: [
    "The object can not be modified in this way.",
    "InvalidModificationError",
  ],
  SYNTAX: (m) => [
    `Failed to execute 'write' on 'UnderlyingSinkBase': Invalid params passed. ${m}`,
    "SyntaxError",
  ],
  SECURITY: [
    "It was determined that certain files are unsafe for access within a Web application, or that too many calls are being made on file resources.",
    "SecurityError",
  ],
  DISALLOWED: [
    "The request is not allowed by the user agent or the platform in the current context.",
    "NotAllowedError",
  ],
};

const {
  ObjectPrototypeIsPrototypeOf,
  SymbolFor,
} = primordials;

class StorageManager {
  constructor() {
    webidl.illegalConstructor();
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/estimate
  async estimate() {
    throw new Error("Not implemented");
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/persist
  async persist() {
    throw new Error("Not implemented");
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/persisted
  async persisted() {
    throw new Error("Not implemented");
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/StorageManager/getDirectory
  async getDirectory() {
    const location = new URL(".", op_get_location());

    if (location.protocol !== "file:") {
      throw new DOMException(
        "The getDirectory method is only available for file URLs.",
        "SecurityError",
      );
    }

    return new FileSystemDirectoryHandle(location.href, "");
  }

  [SymbolFor("Deno.privateCustomInspect")](inspect, inspectOptions) {
    return inspect(
      console.createFilteredInspectProxy({
        object: this,
        evaluate: ObjectPrototypeIsPrototypeOf(StorageManager, this),
        keys: [],
      }),
      inspectOptions,
    );
  }
}

const storage = webidl.createBranded(StorageManager);

class FileSystemHandle {
  #kind;
  #root;
  #name;

  constructor(kind, root, name) {
    this.#kind = kind;
    this.#root = root;
    this.#name = name;
  }

  get kind() {
    return this.#kind;
  }

  get name() {
    return this.#name;
  }

  get ref() {
    return join(this.#root, this.#name);
  }

  async isSameEntry(other) {
    if (this === other) {
      return true;
    }

    return other instanceof FileSystemHandle &&
      this.kind == other.kind &&
      this.ref == other.ref;
  }
}

class FileSystemDirectoryHandle extends FileSystemHandle {
  constructor(root, name) {
    super("directory", root, name);
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getDirectoryHandle
  async getDirectoryHandle(name, options = {}) {
    checkName(name);
    options.create = !!options.create;

    const url = new URL(join(this.ref, name));
    const stat = await fs.lstat(url).catch(() => {});
    const isDirectory = stat?.isDirectory;
    if (isDirectory) {
      return new FileSystemDirectoryHandle(this.ref, name);
    }
    if (isDirectory === false) {
      throw new DOMException(...errors.MISMATCH);
    }
    if (!options.create) {
      throw new DOMException(...errors.GONE);
    }
    await fs.mkdir(url);
    return new FileSystemDirectoryHandle(this.ref, name);
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getFileHandle
  async getFileHandle(name, options = {}) {
    checkName(name);
    options.create = !!options.create;

    const url = new URL(join(this.ref, name));
    const stat = await fs.lstat(url).catch(() => {});
    const isFile = stat?.isFile;
    if (isFile) {
      return new FileSystemFileHandle(this.ref, name);
    }
    if (isFile === false) {
      throw new DOMException(...errors.MISMATCH);
    }
    if (!options.create) {
      throw new DOMException(...errors.GONE);
    }
    const c = await fs.open(url, { create: true, write: true });
    c.close();
    return new FileSystemFileHandle(this.ref, name);
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/entries
  async *entries() {
    for await (const entry of fs.readDir(new URL(this.ref))) {
      if (entry.isDirectory) {
        yield [entry.name, new FileSystemDirectoryHandle(this.ref, entry.name)];
      } else if (entry.isFile) {
        yield [entry.name, new FileSystemFileHandle(this.ref, entry.name)];
      }
    }
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/removeEntry
  async removeEntry(name, options = {}) {
    checkName(name);
    const url = new URL(join(this.ref, name));
    try {
      const stat = await fs.lstat(url);

      if (stat.isDirectory) {
        if (options.recursive) {
          try {
            await fs.remove(path, { recursive: true });
          } catch (err) {
            if (err.code === "ENOTEMPTY") {
              throw new DOMException(...errors.MOD_ERR);
            }
            throw err;
          }
        } else {
          try {
            await fs.remove(path);
          } catch (err) {
            throw new DOMException(...MOD_ERR);
          }
        }
      } else {
        await fs.remove(path);
      }
    } catch (err) {
      if (err.name === "NotFound") {
        throw new DOMException(...errors.GONE);
      }
      throw err;
    }
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/resolve
  async resolve(possibleDescendant) {
    if (await possibleDescendant.isSameEntry(this)) {
      return [];
    }

    const openSet = [{ handle: this, path: [] }];

    while (openSet.length) {
      let { handle: current, path } = openSet.pop();

      for await (const entry of current.values()) {
        if (await entry.isSameEntry(possibleDescendant)) {
          return [...path, entry.name];
        }
        if (entry.kind === "directory") {
          openSet.push({ handle: entry, path: [...path, entry.name] });
        }
      }
    }

    return null;
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/keys
  async *keys() {
    for await (const [name] of this.entries()) {
      yield name;
    }
  }

  async *values() {
    for await (const [_, entry] of this) {
      yield entry;
    }
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/values
  [Symbol.asyncIterator]() {
    return this.entries();
  }
}

Object.defineProperty(FileSystemDirectoryHandle.prototype, Symbol.toStringTag, {
  value: "FileSystemDirectoryHandle",
  writable: false,
  enumerable: false,
  configurable: true,
});

Object.defineProperties(FileSystemDirectoryHandle.prototype, {
  getDirectoryHandle: { enumerable: true },
  entries: { enumerable: true },
  getFileHandle: { enumerable: true },
  removeEntry: { enumerable: true },
});

class FileSystemFileHandle extends FileSystemHandle {
  constructor(root, name) {
    super("file", root, name);
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/createWritable
  async createWritable(options = {}) {
    try {
      const url = new URL(this.ref);
      const fileHandle = await fs.open(url, {
        write: true,
        truncate: !options.keepExistingData,
      });
      const { size } = await fileHandle.stat();
      return new FileSystemWritableFileStream(new Sink(fileHandle, size));
    } catch (err) {
      if (err.name === "NotFound") {
        throw new DOMException(...errors.GONE);
      }
      throw err;
    }
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/getFile
  async getFile() {
    try {
      const url = new URL(this.ref);
      const [stats, content] = await Promise.all([
        fs.stat(url),
        fs.readFile(url),
      ]);
      return new File([content], this.name, {
        lastModified: Number(stats.mtime),
      });
    } catch (err) {
      if (err.name === "NotFound") {
        throw new DOMException(...errors.GONE);
      }
      throw err;
    }
  }
}

Object.defineProperty(FileSystemFileHandle.prototype, Symbol.toStringTag, {
  value: "FileSystemFileHandle",
  writable: false,
  enumerable: false,
  configurable: true,
});

Object.defineProperties(FileSystemFileHandle.prototype, {
  createWritable: { enumerable: true },
  getFile: { enumerable: true },
});

class FileSystemWritableFileStream extends WritableStream {
  #closed = false;

  constructor(writer) {
    super(writer);
  }

  async close() {
    this.#closed = true;
    const writer = this.getWriter();
    const p = writer.close();
    writer.releaseLock();
    return p;
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/seek
  seek(position) {
    return this.write({ type: "seek", position });
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/truncate
  truncate(size) {
    return this.write({ type: "truncate", size });
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/write
  write(data) {
    if (this.#closed) {
      return Promise.reject(
        new TypeError("Cannot write to a CLOSED writable stream"),
      );
    }

    const writer = this.getWriter();
    const result = writer.write(data);
    writer.releaseLock();
    return result;
  }
}

Object.defineProperty(
  FileSystemWritableFileStream.prototype,
  Symbol.toStringTag,
  {
    value: "FileSystemWritableFileStream",
    writable: false,
    enumerable: false,
    configurable: true,
  },
);

Object.defineProperties(FileSystemWritableFileStream.prototype, {
  close: { enumerable: true },
  seek: { enumerable: true },
  truncate: { enumerable: true },
  write: { enumerable: true },
});

export class Sink {
  constructor(fileHandle, size) {
    this.fileHandle = fileHandle;
    this.size = size;
    this.position = 0;
  }

  async abort() {
    await this.fileHandle.close();
  }

  async write(chunk) {
    if (typeof chunk === "object") {
      if (chunk.type === "write") {
        if (Number.isInteger(chunk.position) && chunk.position >= 0) {
          this.position = chunk.position;
        }
        if (!("data" in chunk)) {
          await this.fileHandle.close();
          throw new DOMException(
            ...errors.SYNTAX("write requires a data argument"),
          );
        }
        chunk = chunk.data;
      } else if (chunk.type === "seek") {
        if (Number.isInteger(chunk.position) && chunk.position >= 0) {
          if (this.size < chunk.position) {
            throw new DOMException(...errors.INVALID);
          }
          this.position = chunk.position;
          return;
        } else {
          await this.fileHandle.close();
          throw new DOMException(
            ...errors.SYNTAX("seek requires a position argument"),
          );
        }
      } else if (chunk.type === "truncate") {
        if (Number.isInteger(chunk.size) && chunk.size >= 0) {
          await this.fileHandle.truncate(chunk.size);
          this.size = chunk.size;
          if (this.position > this.size) {
            this.position = this.size;
          }
          return;
        } else {
          await this.fileHandle.close();
          throw new DOMException(
            ...errors.SYNTAX("truncate requires a size argument"),
          );
        }
      }
    }

    if (chunk instanceof ArrayBuffer) {
      chunk = new Uint8Array(chunk);
    } else if (typeof chunk === "string") {
      chunk = new TextEncoder().encode(chunk);
    } else if (chunk instanceof Blob) {
      await this.fileHandle.seek(this.position, SeekMode.Start);
      for await (const data of chunk.stream()) {
        const written = await this.fileHandle.write(data);
        this.position += written;
        this.size += written;
      }
      return;
    }
    await this.fileHandle.seek(this.position, SeekMode.Start);
    const written = await this.fileHandle.write(chunk);
    this.position += written;
    this.size += written;
  }

  async close() {
    await this.fileHandle.close();
  }
}

export {
  FileSystemDirectoryHandle,
  FileSystemFileHandle,
  FileSystemHandle,
  FileSystemWritableFileStream,
  storage,
};

function join(root, name) {
  if (root.endsWith("/")) {
    return root + name;
  }
  return root + "/" + name;
}

function checkName(name) {
  if (name === "") {
    throw new TypeError(`Name can't be an empty string.`);
  }
  if (name === "." || name === ".." || name.includes("/")) {
    throw new TypeError(`Name contains invalid characters.`);
  }
}
