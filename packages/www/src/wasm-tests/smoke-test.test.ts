import { WasmLibT, loadWasmLib } from 'wasm/wasm-loader';

describe('wasm smoke test', () => {
	let wasm: WasmLibT;

	beforeAll(async () => {
		wasm = (await loadWasmLib())!;
		if (!wasm) fail();
	});

	test('functions exist', () => {
		expect(wasm.init).toBeTruthy();
		expect(typeof wasm.init).toBe('function');

		expect(wasm.get_greeting).toBeTruthy();
		expect(typeof wasm.get_greeting).toBe('function');
		expect(typeof wasm.get_greeting()).toBe('string');

		expect(wasm.greet).toBeTruthy();
		expect(typeof wasm.greet).toBe('function');
	});
});
