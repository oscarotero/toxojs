import foo from "./proba.js";
Deno.core.print(bar() + "\n");

export default function bar() {
  return 3 + 4;
}