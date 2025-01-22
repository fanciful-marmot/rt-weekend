import init, { render } from './pkg/wasm_rt.js';

let memory;

init().then((args) => {
  memory = args.memory;
  postMessage({ type: 'ready' });
});

onmessage = e => {
  const msg = e.data;

  switch (msg.type) {
    case 'init':
      break;

    case 'render':
      const start = performance.now();
      render(msg.script, ptr => {
        console.log(ptr);
        const buffer =(new Uint8ClampedArray(memory.buffer, ptr, 600 * 300 * 4)).slice();
        
        postMessage({
          type: 'frame',
          data: buffer.buffer
        }, [buffer.buffer]);
      });
      postMessage({
        type: 'done',
        time: performance.now() - start,
      });
      break;
    
    default:
      console.log('Unkown message from worker', msg.type);
      break;
  }
};
