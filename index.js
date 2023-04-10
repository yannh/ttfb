import wasmInit, {
    run
} from "./pkg/ttfb.js";

const runWasm = async () => {
    await wasmInit("./pkg/ttfb_bg.wasm");
    run();
}

runWasm();
