import { getGlobalWasmState, getWasmLibIfLoaded } from 'wasm/wasm-loader';

describe('wasm global test', () => {
	beforeAll(async () => {
		await getGlobalWasmState().promise;
	});

	test('wasm loaded', () => {
		const state = getGlobalWasmState();
		expect(state.isLoading).toBeFalsy();
		expect(state.failedToLoad).toBeFalsy();
		const wasmLib = getWasmLibIfLoaded();
		expect(wasmLib).toBeTruthy();
	});
});
