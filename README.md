# TOXO

Minimalist JavaScript Runtime.

This project is an experimental JavaScript runtime built as a learning exercise
in Rust. Most of the code is based on [Deno](https://github.com/denoland/). Huge
thanks to the Deno team for their amazing open source work!

## How to use it?

```sh
toxo example.js
```

## Import supported

- File imports
- HTTP imports
- data: imports
- Import attributes (json, text, bytes)
- Import maps (Place a `import_map.json` file in the same directory as the entry
  point)

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
  > [!info]:
  >
  > The data is stored in the `local_storage` file in the same directory as the
  > main module.
- sessionStorage
- WebAssembly
