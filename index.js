import wasmInit, {
    ttfb_js
} from "./pkg/ttfb.js";

const runWasm = async () => {
    await wasmInit("./pkg/ttfb_bg.wasm");
    console.log(await ttfb_js("foo", false));
}

runWasm();
