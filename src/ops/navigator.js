import { primordials } from "ext:core/mod.js";
import { op_get_languages, op_get_user_agent } from "ext:core/ops";
import * as webidl from "ext:deno_webidl/00_webidl.js";
import * as console from "ext:deno_console/01_console.js";
import { loadWebGPU } from "ext:deno_webgpu/00_init.js";
import { storage } from "ext:toxo_filesystem/filesystem.js";

const {
  ObjectDefineProperties,
  ObjectPrototypeIsPrototypeOf,
  SymbolFor,
} = primordials;

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
          "userAgent",
          "language",
          "languages",
          "storage",
        ],
      }),
      inspectOptions,
    );
  }
}

const navigator = webidl.createBranded(Navigator);

function memoizeLazy(f) {
  let v_ = null;
  return () => {
    if (v_ === null) {
      v_ = f();
    }
    return v_;
  };
}

const userAgent = memoizeLazy(() => op_get_user_agent());
const languages = memoizeLazy(() =>
  op_get_languages()
    .split(",")
    .map((lang) => lang.trim())
    .filter((lang) => lang.length > 0)
);

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
  userAgent: {
    __proto__: null,
    configurable: true,
    enumerable: true,
    get() {
      webidl.assertBranded(this, NavigatorPrototype);
      return userAgent();
    },
  },
  language: {
    __proto__: null,
    configurable: true,
    enumerable: true,
    get() {
      webidl.assertBranded(this, NavigatorPrototype);
      return languages()[0] || "en-US";
    },
  },
  languages: {
    __proto__: null,
    configurable: true,
    enumerable: true,
    get() {
      webidl.assertBranded(this, NavigatorPrototype);
      return languages();
    },
  },
  storage: {
    __proto__: null,
    configurable: true,
    enumerable: true,
    get() {
      webidl.assertBranded(this, NavigatorPrototype);
      return storage;
    },
  },
});

const NavigatorPrototype = Navigator.prototype;

export { Navigator, navigator };
