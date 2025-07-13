# TOXO

Minimalist JavaScript Runtime.

This project is an experimental JavaScript runtime built as a learning exercise
in Rust. Most of the code is based on [Deno](https://github.com/denoland/), so
all credits belong to them.

## How to use it?

Install TOXO (macOS & Linux only):

```sh
curl -fsSL https://raw.githubusercontent.com/oscarotero/toxojs/refs/heads/main/install.sh | sh
```

Run JavaScript files:

```sh
toxo example.js
```

## Why another runtime?

This is an experimental project created to learn Rust and explore the internals
of Deno. The majority of the code is based on Deno, and I am deeply grateful to
its authors for building such an impressive runtime, making it open source, and
enabling others to create custom runtimes using their work. The goal of TOXO is
to build a JavaScript runtime with the following design principles:

### No tooling

Many runtimes like Deno or Bun include built-in tools for testing, benchmarking,
script running, compiling, package management, formatting, and more. TOXO, on
the other hand, is solely a JavaScript runner. It doesn't support subcommands
(there's no `toxo run [filename]`); you simply use `toxo [filename]`. This
design choice helps keep the project focused and minimal.

### No TypeScript support

TypeScript is a powerful tool, but it introduces additional complexity: it
creates a parallel ecosystem and adds extra steps between writing and executing
your code. It requires configuration, supports JSX by default (which may need
further setup), and relies on source maps and caching systems to manage
compilation and debugging. By supporting only web-standard formats that run
natively in the JavaScript engine, TOXO avoids this complexity: what you write
is exactly what gets executed.

### Standard imports

TOXO supports modern, standard ES module imports from URLs and local files. You
can use import attributes to load modules as JSON, text, bytes, or even WASM
files. Import maps are also supported so you can use bare imports in your code.
[More info below](#import-supported).

CommonJS modules and package registries such as NPM or JSR are not supported.
These systems are centralized, non-standard, and introduce vendor lock-in with
complex and opaque module resolution mechanisms.

### Only Web APIs

TOXO implements only standard Web APIs available in browsers, enabling you to
write code that runs consistently across environments. There is no support for
`node:*` modules or a global `Toxo.*` object with additional features.
[See the list below](#web-apis-supported) to know the Web APIs supported
currently.

### No configuration file

There is no `toxo.json` or support for `package.json` file. Just as web
technologies typically don't require configuration files, TOXO follows the same
philosophy.

However, two files are loaded automatically if present in the same directory as
the main module:

- `import_map.json`: If available, this file is automatically loaded and used
  for module resolution.
- `.env`: Environment variables can be used to change some behaviors (such as
  the User Agent for HTTP requests or the value of `navigator.languages`). If a
  `.env` file exists, it is automatically loaded.
  [More info about environment variables](#environment-variables)

### Vendoring by default

Unlike Node, which requires running `npm install` to fetch dependencies, or
Deno, which caches dependencies in a centralized folder, TOXO automatically
vendors all remote dependencies in a `vendor` folder located alongside your main
module. This process happens transparently and there's no need to modify module
URLs or update your `import_map.json`. For example, a module imported from
`https://example.com/modules/main.js` will be saved as
`vendor/example.com/modules.main.js`. This approach enables offline execution
and makes all dependencies easily accessible and editable.

You can disable vendoring by setting the `TOXO_VENDOR=none` environment
variable.

### Storage folder

When you use Web APIs like `localStorage` or `Cache`, TOXO automatically creates
the `storage` folder in the same directory as the main module to store this
data. This allows to access to this data easily. You can add this file to your
`.gitignore` if you want to keep it local, or commit it to your repository to
share the stored data.

## Environment variables

TOXO does not require a configuration file. However, if a `.env` file is present
in the same directory as the main module, it will be loaded automatically. You
can use environment variables in this file to configure certain behaviors.

- `TOXO_LANGUAGES`: Comma-separated list of languages to set the values of
  `Navigator.language` and `Navigator.languages`. Defaults to `en-US`.
- `TOXO_USER_AGENT`: Sets the value returned by `Navigator.userAgent` and used
  for HTTP requests when fetching modules. Defaults to `TOXO/{version}`.
- `TOXO_VENDOR`: Set to `none` to disable the automatic vendoring.

## Import supported

- File imports
- HTTP imports
- data: imports
- Import attributes (json, text, bytes)
- Import maps (Place a `import_map.json` file in the same directory as the entry
  point)
- WASM (Import a module with the `.wasm` extension)

## Web APIs supported

- console
- URL
- URLSearchParams
- URLPattern
- AbortController
- AbortSignal
- atob
- btoa
- CompressionStream
- DecompressionStream
- DOMException
- TextDecoder
- TextEncoder
- TextDecoderStream
- TextEncoderStream
- CloseEvent
- CustomEvent
- ErrorEvent
- Event
- EventTarget
- MessageEvent
- PromiseRejectionEvent
- ProgressEvent
- reportError
- Blob
- File
- FileReader
- ImageData
- MessageChannel
- MessagePort
- structuredClone
- Performance
- PerformanceEntry
- PerformanceMark
- PerformanceMeasure
- performance
- ByteLengthQueuingStrategy
- CountQueuingStrategy
- ReadableStream
- ReadableStreamDefaultReader
- TransformStream
- WritableStream
- WritableStreamDefaultWriter
- WritableStreamDefaultController
- ReadableStreamDefaultController
- ReadableStreamBYOBReader
- ReadableStreamBYOBRequest
- ReadableStreamDefaultController
- TransformStreamDefaultController
- clearInterval
- clearTimeout
- setInterval
- setTimeout
- CryptoKey
- crypto
- Crypto
- SubtleCrypto
- fetch
- Request
- Response
- Headers
- FormData
- localStorage
  > Data stored in the `./storage/local_storage` file.
- sessionStorage
- WebAssembly
- Temporal
- Intl
- Navigator
- GPU
- GPUAdapter
- GPUAdapterInfo
- GPUBuffer
- GPUBufferUsage
- GPUCanvasContext
- GPUColorWrite
- GPUCommandBuffer
- GPUCommandEncoder
- GPUComputePassEncoder
- GPUComputePipeline
- GPUDevice
- GPUDeviceLostInfo
- GPUError
- GPUBindGroup
- GPUBindGroupLayout
- GPUInternalError
- GPUPipelineError
- GPUUncapturedErrorEvent
- GPUPipelineLayout
- GPUQueue
- GPUQuerySet
- GPUMapMode
- GPUOutOfMemoryError
- GPURenderBundle
- GPURenderBundleEncoder
- GPURenderPassEncoder
- GPURenderPipeline
- GPUSampler
- GPUShaderModule
- GPUShaderStage
- GPUSupportedFeatures
- GPUSupportedLimits
- GPUTexture
- GPUTextureView
- GPUTextureUsage
- GPUValidationError
- alert
- confirm
- prompt
- FileSystemHandle
- FileSystemFileHandle
- FileSystemDirectoryHandle
- FileSystemWritableFileStream
- caches
  > Data stored in the `./storage/caches` folder.
- CacheStorage
- Cache
