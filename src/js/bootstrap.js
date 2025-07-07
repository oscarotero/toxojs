import { core, primordials } from "ext:core/mod.js";

const {
  ObjectDefineProperties,
  ObjectPrototypeIsPrototypeOf,
  SymbolFor,
} = primordials;

function propNonEnumerable(name, value) {
  Object.defineProperty(
    globalThis,
    name,
    core.propNonEnumerable(value),
  );
}

function propNonEnumerableLazyLoaded(name, getter, loadFn) {
  Object.defineProperty(
    globalThis,
    name,
    core.propNonEnumerableLazyLoaded(getter, loadFn),
  );
}

function propWritable(name, value) {
  Object.defineProperty(
    globalThis,
    name,
    core.propWritable(value),
  );
}

function propReadOnly(name, value) {
  Object.defineProperty(
    globalThis,
    name,
    core.propReadOnly(value),
  );
}

function propGetterOnly(name, getter) {
  Object.defineProperty(
    globalThis,
    name,
    core.propGetterOnly(getter),
  );
}

/** deno_webidl */
import * as webidl from "ext:deno_webidl/00_webidl.js";

Object.defineProperty(globalThis, webidl.brand, {
  value: webidl.brand,
  enumerable: false,
  configurable: true,
  writable: true,
});

/** deno_console */
import * as console from "ext:deno_console/01_console.js";
const Deno = globalThis.Deno || {};

propNonEnumerable(
  "console",
  new console.Console((msg, level) => core.print(msg, level > 1)),
);

/** deno_url */
import * as url from "ext:deno_url/00_url.js";
import * as urlPattern from "ext:deno_url/01_urlpattern.js";

propNonEnumerable("URL", url.URL);
propNonEnumerable("URLSearchParams", url.URLSearchParams);
propNonEnumerable("URLPattern", urlPattern.URLPattern);

/** deno_web */
import * as infra from "ext:deno_web/00_infra.js";
import * as domException from "ext:deno_web/01_dom_exception.js";
import * as mimesniff from "ext:deno_web/01_mimesniff.js";
import * as event from "ext:deno_web/02_event.js";
import * as structuredClone from "ext:deno_web/02_structured_clone.js";
import * as timers from "ext:deno_web/02_timers.js";
import * as abortSignal from "ext:deno_web/03_abort_signal.js";
import * as globalInterfaces from "ext:deno_web/04_global_interfaces.js";
import * as base64 from "ext:deno_web/05_base64.js";
import * as streams from "ext:deno_web/06_streams.js";
import * as encoding from "ext:deno_web/08_text_encoding.js";
import * as file from "ext:deno_web/09_file.js";
import * as fileReader from "ext:deno_web/10_filereader.js";
import * as location from "ext:deno_web/12_location.js";
import * as messagePort from "ext:deno_web/13_message_port.js";
import * as compression from "ext:deno_web/14_compression.js";
import * as performance from "ext:deno_web/15_performance.js";
import * as imageData from "ext:deno_web/16_image_data.js";

// deno_web (abortSignal)
propNonEnumerable("AbortController", abortSignal.AbortController);
propNonEnumerable("AbortSignal", abortSignal.AbortSignal);

// deno_web (base64)
propWritable("atob", base64.atob);
propWritable("btoa", base64.btoa);

// deno_web (compression)
propNonEnumerable("CompressionStream", compression.CompressionStream);
propNonEnumerable("DecompressionStream", compression.DecompressionStream);

// deno_web (DOMException)
propNonEnumerable("DOMException", domException.DOMException);

// deno_web (encoding)
propNonEnumerable("TextDecoder", encoding.TextDecoder);
propNonEnumerable("TextEncoder", encoding.TextEncoder);
propNonEnumerable("TextDecoderStream", encoding.TextDecoderStream);
propNonEnumerable("TextEncoderStream", encoding.TextEncoderStream);

// deno_web (event)
propNonEnumerable("CloseEvent", event.CloseEvent);
propNonEnumerable("CustomEvent", event.CustomEvent);
propNonEnumerable("ErrorEvent", event.ErrorEvent);
propNonEnumerable("Event", event.Event);
propNonEnumerable("EventTarget", event.EventTarget);
propNonEnumerable("MessageEvent", event.MessageEvent);
propNonEnumerable("PromiseRejectionEvent", event.PromiseRejectionEvent);
propNonEnumerable("ProgressEvent", event.ProgressEvent);
propWritable("reportError", event.reportError);

// deno_web (file)
propNonEnumerable("Blob", file.Blob);
propNonEnumerable("File", file.File);

// deno_web (fileReader)
propNonEnumerable("FileReader", fileReader.FileReader);

// deno_web (imageData)
propNonEnumerable("ImageData", imageData.ImageData);

// deno_web (messagePort)
propNonEnumerable("MessageChannel", messagePort.MessageChannel);
propNonEnumerable("MessagePort", messagePort.MessagePort);
propWritable("structuredClone", structuredClone.structuredClone);

// deno_web (performance)
propNonEnumerable("Performance", performance.Performance);
propNonEnumerable("PerformanceEntry", performance.PerformanceEntry);
propNonEnumerable("PerformanceMark", performance.PerformanceMark);
propNonEnumerable("PerformanceMeasure", performance.PerformanceMeasure);
propWritable("performance", performance.performance);

// deno_web (streams)
propNonEnumerable(
  "ByteLengthQueuingStrategy",
  streams.ByteLengthQueuingStrategy,
);
propNonEnumerable("CountQueuingStrategy", streams.CountQueuingStrategy);
propNonEnumerable("ReadableStream", streams.ReadableStream);
propNonEnumerable(
  "ReadableStreamDefaultReader",
  streams.ReadableStreamDefaultReader,
);
propNonEnumerable("TransformStream", streams.TransformStream);
propNonEnumerable("WritableStream", streams.WritableStream);
propNonEnumerable(
  "WritableStreamDefaultWriter",
  streams.WritableStreamDefaultWriter,
);
propNonEnumerable(
  "WritableStreamDefaultController",
  streams.WritableStreamDefaultController,
);
propNonEnumerable(
  "ReadableStreamDefaultController",
  streams.ReadableStreamDefaultController,
);
propNonEnumerable("ReadableStreamBYOBReader", streams.ReadableStreamBYOBReader);
propNonEnumerable(
  "ReadableStreamBYOBRequest",
  streams.ReadableStreamBYOBRequest,
);
propNonEnumerable(
  "ReadableStreamDefaultController",
  streams.ReadableStreamDefaultController,
);
propNonEnumerable(
  "TransformStreamDefaultController",
  streams.TransformStreamDefaultController,
);

// deno_web (timers)
propWritable("clearInterval", timers.clearInterval);
propWritable("clearTimeout", timers.clearTimeout);
propWritable("setInterval", timers.setInterval);
propWritable("setTimeout", timers.setTimeout);

/** deno_crypto */
import * as crypto from "ext:deno_crypto/00_crypto.js";

propNonEnumerable("CryptoKey", crypto.CryptoKey);
propNonEnumerable("Crypto", crypto.Crypto);
propNonEnumerable("SubtleCrypto", crypto.SubtleCrypto);
propReadOnly("crypto", crypto.crypto);

/** deno_net */
import * as net from "ext:deno_net/01_net.js";
import * as tls from "ext:deno_net/02_tls.js";

/** deno_fetch */
import * as headers from "ext:deno_fetch/20_headers.js";
import * as formData from "ext:deno_fetch/21_formdata.js";
import * as request from "ext:deno_fetch/23_request.js";
import * as response from "ext:deno_fetch/23_response.js";
import * as fetch from "ext:deno_fetch/26_fetch.js";
import * as eventSource from "ext:deno_fetch/27_eventsource.js";

// Set up the callback for Wasm streaming ops
Deno.core.setWasmStreamingCallback(fetch.handleWasmStreaming);

propWritable("fetch", fetch.fetch);

propNonEnumerable("Request", request.Request);
propNonEnumerable("Response", response.Response);
propNonEnumerable("Headers", headers.Headers);
propNonEnumerable("FormData", formData.FormData);

/** deno_webstorage */
import * as webStorage from "ext:deno_webstorage/01_webstorage.js";

propWritable("localStorage", webStorage.localStorage());
propWritable("sessionStorage", webStorage.sessionStorage());

/** deno_webgpu */
import { loadWebGPU } from "ext:deno_webgpu/00_init.js";
import * as webgpuSurface from "ext:deno_webgpu/02_surface.js";

propNonEnumerableLazyLoaded("GPU", (webgpu) => webgpu.GPU, loadWebGPU);
propNonEnumerableLazyLoaded(
  "GPUAdapter",
  (webgpu) => webgpu.GPUAdapter,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUAdapterInfo",
  (webgpu) => webgpu.GPUAdapterInfo,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUBuffer",
  (webgpu) => webgpu.GPUBuffer,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUBufferUsage",
  (webgpu) => webgpu.GPUBufferUsage,
  loadWebGPU,
);
propNonEnumerable("GPUCanvasContext", webgpuSurface.GPUCanvasContext);
propNonEnumerableLazyLoaded(
  "GPUColorWrite",
  (webgpu) => webgpu.GPUColorWrite,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUCommandBuffer",
  (webgpu) => webgpu.GPUCommandBuffer,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUCommandEncoder",
  (webgpu) => webgpu.GPUCommandEncoder,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUComputePassEncoder",
  (webgpu) => webgpu.GPUComputePassEncoder,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUComputePipeline",
  (webgpu) => webgpu.GPUComputePipeline,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUDevice",
  (webgpu) => webgpu.GPUDevice,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUDeviceLostInfo",
  (webgpu) => webgpu.GPUDeviceLostInfo,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUError",
  (webgpu) => webgpu.GPUError,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUBindGroup",
  (webgpu) => webgpu.GPUBindGroup,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUBindGroupLayout",
  (webgpu) => webgpu.GPUBindGroupLayout,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUInternalError",
  (webgpu) => webgpu.GPUInternalError,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUPipelineError",
  (webgpu) => webgpu.GPUPipelineError,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUUncapturedErrorEvent",
  (webgpu) => webgpu.GPUUncapturedErrorEvent,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUPipelineLayout",
  (webgpu) => webgpu.GPUPipelineLayout,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUQueue",
  (webgpu) => webgpu.GPUQueue,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUQuerySet",
  (webgpu) => webgpu.GPUQuerySet,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUMapMode",
  (webgpu) => webgpu.GPUMapMode,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUOutOfMemoryError",
  (webgpu) => webgpu.GPUOutOfMemoryError,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPURenderBundle",
  (webgpu) => webgpu.GPURenderBundle,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPURenderBundleEncoder",
  (webgpu) => webgpu.GPURenderBundleEncoder,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPURenderPassEncoder",
  (webgpu) => webgpu.GPURenderPassEncoder,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPURenderPipeline",
  (webgpu) => webgpu.GPURenderPipeline,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUSampler",
  (webgpu) => webgpu.GPUSampler,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUShaderModule",
  (webgpu) => webgpu.GPUShaderModule,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUShaderStage",
  (webgpu) => webgpu.GPUShaderStage,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUSupportedFeatures",
  (webgpu) => webgpu.GPUSupportedFeatures,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUSupportedLimits",
  (webgpu) => webgpu.GPUSupportedLimits,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUTexture",
  (webgpu) => webgpu.GPUTexture,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUTextureView",
  (webgpu) => webgpu.GPUTextureView,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUTextureUsage",
  (webgpu) => webgpu.GPUTextureUsage,
  loadWebGPU,
);
propNonEnumerableLazyLoaded(
  "GPUValidationError",
  (webgpu) => webgpu.GPUValidationError,
  loadWebGPU,
);

/** Navigator API */
// import {
//   op_bootstrap_language,
//   op_bootstrap_numcpus,
//   op_bootstrap_user_agent,
// } from "ext:core/ops";

class Navigator {
  constructor() {
    webidl.illegalConstructor();
  }

  [SymbolFor("Deno.privateCustomInspect")](inspect, inspectOptions) {
    return inspect(
      console.createFilteredInspectProxy({
        object: this,
        evaluate: ObjectPrototypeIsPrototypeOf(NavigatorPrototype, this),
        keys: [
          // "hardwareConcurrency",
          // "userAgent",
          // "language",
          // "languages",
        ],
      }),
      inspectOptions,
    );
  }
}

const navigator = webidl.createBranded(Navigator);

// function memoizeLazy(f) {
//   let v_ = null;
//   return () => {
//     if (v_ === null) {
//       v_ = f();
//     }
//     return v_;
//   };
// }

// const numCpus = memoizeLazy(() => op_bootstrap_numcpus());
// const userAgent = memoizeLazy(() => op_bootstrap_user_agent());
// const language = memoizeLazy(() => op_bootstrap_language());

ObjectDefineProperties(Navigator.prototype, {
  gpu: {
    __proto__: null,
    configurable: true,
    enumerable: true,
    get() {
      webidl.assertBranded(this, NavigatorPrototype);
      const webgpu = loadWebGPU();
      webgpu.initGPU();
      return webgpu.gpu;
    },
  },
  //   hardwareConcurrency: {
  //     __proto__: null,
  //     configurable: true,
  //     enumerable: true,
  //     get() {
  //       webidl.assertBranded(this, NavigatorPrototype);
  //       return numCpus();
  //     },
  //   },
  //   userAgent: {
  //     __proto__: null,
  //     configurable: true,
  //     enumerable: true,
  //     get() {
  //       webidl.assertBranded(this, NavigatorPrototype);
  //       return userAgent();
  //     },
  //   },
  //   language: {
  //     __proto__: null,
  //     configurable: true,
  //     enumerable: true,
  //     get() {
  //       webidl.assertBranded(this, NavigatorPrototype);
  //       return language();
  //     },
  //   },
  //   languages: {
  //     __proto__: null,
  //     configurable: true,
  //     enumerable: true,
  //     get() {
  //       webidl.assertBranded(this, NavigatorPrototype);
  //       return [language()];
  //     },
  //   },
});

const NavigatorPrototype = Navigator.prototype;

propNonEnumerable("Navigator", Navigator);
propGetterOnly("navigator", () => navigator);

// Remove Deno global
Object.defineProperty(globalThis, "Deno", {
  value: undefined,
  enumerable: false,
  configurable: true,
  writable: true,
});
