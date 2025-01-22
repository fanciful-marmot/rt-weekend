const worker = new Worker(new URL('worker.mjs', import.meta.url), { type: 'module' });

const canvas = document.getElementById('output');
const ctx = canvas.getContext('2d', { alpha: false });
const button = document.getElementById('run');
const textArea = document.getElementById('script');
const timeDisplay = document.getElementById('time');

worker.postMessage({
  type: 'init',
});

worker.onmessage = e => {
  const msg = e.data;

  switch (msg.type) {
    case 'frame': {
      const image = new ImageData(new Uint8ClampedArray(msg.data), 600, 300);
      ctx.putImageData(image, 0, 0);
      break;
    }

    case 'ready':
      button.removeAttribute('disabled');
      break;

    case 'done':
      button.removeAttribute('disabled');
      timeDisplay.textContent = msg.time / 1000 + ' seconds';
      break;
    
    default:
      console.log('Unkown message from worker', msg.type);
      break;
  }
};

button.addEventListener('click', () => {
  timeDisplay.textContent = '';
  button.setAttribute('disabled', true);
  worker.postMessage({
    type: 'render',
    script: textArea.value,
  });
});
