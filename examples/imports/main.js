import { ola } from "./ola.js";
import { ola2 } from "data:text/javascript,export function ola2 () { console.log('Ola mundo from data URL!'); }";
import ola3 from "./hello.json" with { type: "json" };
import ola4 from "./ola.js" with { type: "text" };
import ola5 from "./ola.js" with { type: "bytes" };
import { extname } from "https://cdn.jsdelivr.net/gh/oscarotero/std@1.3.0/path/extname.js";
import extnameBytes from "https://cdn.jsdelivr.net/gh/oscarotero/std@1.3.0/path/extname.js" with {
  type: "bytes",
};
import { extname as posixExtname } from "https://cdn.jsdelivr.net/gh/oscarotero/std@1.3.0/path/posix/extname.js";
import { hello } from "hello";
import { hello2 } from "hello2";
import { extname as getExtension } from "path/extname.js";

console.log(ola());
console.log(ola2());
console.log(ola3);
console.log(ola4);
console.log(ola5);
console.log("extname result:", extname("example.txt"));
console.log("extname result bytes:", extnameBytes.length);
console.log("extname result2:", posixExtname("example.txt"));
console.log(hello());
console.log(hello2());
console.log(getExtension("example.txt"));

console.log("hello from", import.meta.url);
console.log("hello resolved", import.meta.resolve("./ola.js"));

if (true) {
  import("./ola.js").then((module) => {
    console.log("dynamic import:", module.ola());
  });
  import("./import_map.json", { with: { type: "json" } }).then((module) => {
    console.log("dynamic import with import map:", module.default);
  });
}
