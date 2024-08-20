// import wasm, { InitOutput } from '@no-modules/sample-wasm';
import * as Comlink from 'comlink';
// const url = new URL('@no-modules/sample-wasm/sample_wasm_bg.wasm', import.meta.url);
// import module from '@no-modules/sample-wasm/sample_wasm_bg.wasm';
import webWasm, { InitOutput } from '@no-modules/sample-wasm';
// import type wasm from 'sample-wasm';
// const wasmModule = import('sample-wasm/sample_wasm_bg.wasm')
import wasmModule from '../../node_modules/@no-modules/sample-wasm/sample_wasm_bg.wasm';
class SampleWorker {
    wasm!: InitOutput;
    
    constructor() {

        this.setup().then((wasm) => {
            this.wasm = wasm
        })
        
    }

    private async setup() {
        const data = await fetch(wasmModule as any);

        const wasm = await webWasm(data); //await WebAssembly.instantiate(buffer, {})

        console.log("wasm", wasm);
        return wasm
    }

    returnNumber(n: number) {
        return n * 2;

    }
    fibonacci(n: number) {
        return this.wasm.fibonacci(n);
    }
}

const exports = SampleWorker
Comlink.expose(exports);
export { type SampleWorker };
