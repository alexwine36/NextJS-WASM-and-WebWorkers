import { useCallback, useMemo } from 'react';
import { createComlink } from './use-comlink';


const url = new URL('./sampleWorker.ts', import.meta.url);

const useSampleWorker = createComlink<
  typeof import('./sampleWorker').SampleWorker
>(
  () =>
    new Worker(url, {
      type: 'module',
    }),
);


export const useSample = () => {
    const {worker} = useSampleWorker();
    const instance = useMemo(() => {
      return new worker.proxy();
    }, [worker]);

    const runReturnNumber = useCallback(async (n: number) => {
        const use = await instance;
        console.log("use", use);
        console.log("fibb", await use.fibonacci(12));
        return await use.returnNumber(n);

    }, [instance]);

    const runFibonacci = useCallback(async (n: number) => {
      const use = await instance;
      
      return await use.fibonacci(n)

  }, [instance]);

  const runJsFibonacci = useCallback(async (n: number) => {
    const use = await instance;
    
    return await use.jsFibonacci(n)

}, [instance]);

    return {
        runReturnNumber,
        runFibonacci,
        runJsFibonacci
    }
}