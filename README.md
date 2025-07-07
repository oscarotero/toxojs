# TOXO

Minimalist JavaScript Runtime.

This project is an experimental JavaScript runtime built as a learning exercise
in Rust. Most of the code is based on [Deno](https://github.com/denoland/), so
all credits belong to them.

## How to use it?

```sh
toxo example.js
```

## Environment variables

TOXO doesn't have a configuration file. But it detects automatically the `.env`
file if it's in the same directory as the main module. You can use environment
variables to configure some behaviors.

- `TOXO_LANGUAGES`: Comma-separated list of languages to configure the
  `Navigator.language` and `Navigator.languages` values. By default is `en-US`.
- `TOXO_USER_AGENT`: To configure the value returned by `Navigator.userAgent`
  and used to fetch the URL modules. By default is `TOXO/{version}`.

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
  > Data stored in the `local_storage` file in the same directory as the main
  > module.
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
