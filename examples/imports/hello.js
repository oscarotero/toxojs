import { hello as hello2 } from "./hello.js";
console.log(hello2("This should be run only once"));

export function hello(message = "Hello world!") {
  return message;
}
