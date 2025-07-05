import { ola } from "./ola.js";
import { ola2 } from "data:text/javascript,export function ola2 () { console.log('Ola mundo from data URL!'); }";
import ola3 from "./hello.json" with { type: "json" };
import ola4 from "./ola.js" with { type: "text" };
import ola5 from "./ola.js" with { type: "bytes" };
import { extname } from "https://cdn.jsdelivr.net/gh/oscarotero/std@1.3.0/path/extname.js";
import { hello } from "hello";
import { hello2 } from "hello2";
import { extname as getExtension } from "path/extname.js";

console.log(ola());
console.log(ola2());
console.log(ola3);
console.log(ola4);
console.log(ola5);
console.log("extname result:", extname("example.txt"));
console.log(hello());
console.log(hello2());
console.log(getExtension("example.txt"));
