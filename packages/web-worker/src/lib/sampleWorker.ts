import * as Comlink from 'comlink';


class SampleWorker {
    returnNumber(n: number) {
        return n * 2;
    }
}

const exports = SampleWorker
Comlink.expose(exports);
export { type SampleWorker };
