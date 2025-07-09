import { primordials } from "ext:core/mod.js";
import * as console from "ext:deno_console/01_console.js";
import * as webidl from "ext:deno_webidl/00_webidl.js";
import * as fs from "ext:deno_fs/30_fs.js";
import { WritableStream } from "ext:deno_web/06_streams.js";

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
  async getDirectory(name) {
    return new FileSystemDirectoryHandle(name);
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
  #name;

  constructor(kind, name) {
    this.#kind = kind;
    this.#name = name;
  }

  get kind() {
    return this.#kind;
  }

  get name() {
    return this.#name;
  }

  async isSameEntry(other) {
    if (this === other) {
      return true;
    }
    return other instanceof FileSystemHandle &&
      this.kind == other.kind &&
      this.name == other.name;
  }
}

class FileSystemDirectoryHandle extends FileSystemHandle {
  constructor(name) {
    super("directory", name);
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getDirectoryHandle
  async getDirectoryHandle(name, options = {}) {
    if (name === "") {
      throw new TypeError(`Name can't be an empty string.`);
    }
    if (name === "." || name === ".." || name.includes("/")) {
      throw new TypeError(`Name contains invalid characters.`);
    }
    options.create = !!options.create;

    return new FileSystemDirectoryHandle(join(this.name, name));
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/getFileHandle
  async getFileHandle(name, options = {}) {
    if (name === "") {
      throw new TypeError(`Name can't be an empty string.`);
    }
    if (name === "." || name === ".." || name.includes("/")) {
      throw new TypeError(`Name contains invalid characters.`);
    }
    options.create = !!options.create;

    return new FileSystemFileHandle(join(this.name, name));
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/entries
  async *entries() {
    for await (const entry of fs.readDir(new URL(this.name))) {
      if (entry.isDirectory) {
        yield [entry.name, this.getDirectoryHandle(entry.name)];
      } else if (entry.isFile) {
        yield [entry.name, this.getFileHandle(entry.name)];
      }
    }
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/FileSystemDirectoryHandle/removeEntry
  async removeEntry(name, options = {}) {
    throw new Error("Not implemented");
    // if (name === '') {
    //   throw new TypeError(`Name can't be an empty string.`)
    // }
    // if (name === '.' || name === '..' || name.includes('/')) {
    //   throw new TypeError(`Name contains invalid characters.`)
    // }
    // options.recursive = !!options.recursive // cuz node's fs.rm require boolean
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
  constructor(name) {
    super("file", name);
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/createWritable
  async createWritable(options = {}) {
    throw new Error("Not implemented");
    // return new FileSystemWritableFileStream(
    //   await this[kAdapter].createWritable(options)
    // )
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/FileSystemFileHandle/getFile
  async getFile() {
    throw new Error("Not implemented");
    // return new File(await this[kAdapter].getFile())
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
  async close() {
    throw new Error("Not implemented");
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/seek
  seek(position) {
    throw new Error("Not implemented");
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/truncate
  truncate(size) {
    throw new Error("Not implemented");
  }

  // https://developer.mozilla.org/en-US/docs/Web/API/FileSystemWritableFileStream/write
  write(data) {
    throw new Error("Not implemented");
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
