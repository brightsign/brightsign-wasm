import init, { add } from './brightsign-wasm.js';
(async () => {
	console.log('Loading wasm');
	await init(new URL('brightsign-wasm.wasm', import.meta.url));
	console.log(add(9, 4));
})()
