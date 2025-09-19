import init, { start } from "../pkg/hello_wasm.js";

init().then(() => {
  start();
});
