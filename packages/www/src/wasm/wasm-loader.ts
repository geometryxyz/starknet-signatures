import path from 'path';
import { makeDeferred } from 'library/utils/deferred';
import { StarknetModule } from 'starknet-signature';

export type WasmLibT = typeof import('starknet-signature');

async function loadUnsafe(): Promise<WasmLibT> {
	const isTest = process.env.JEST_WORKER_ID !== undefined || typeof jest !== 'undefined';

	// If running test in jest we have to load the node version of the package
	if (isTest)
		return await import(path.resolve(__dirname, '../../rust-wasm/pkg-node', 'rust_wasm_lib'));
	return await import('starknet-signature');
}

// Allows the wasm library to be loaded and awaited
export async function loadWasmLib() {
	let wasm: WasmLibT | undefined;

	try {
		wasm = await loadUnsafe();

		console.log('successfully loaded starknet-signature');

		console.log(wasm);

		// wasm?.init();
	} catch {
		console.log('failed to load starknet-signature');
	}

	return new wasm!.StarknetModule();
}

const globalDeferredLoad = makeDeferred();

const globalState = {
	isLoading: false,
	failedToLoad: false,
	promise: globalDeferredLoad.promise,
};

export function getGlobalWasmState() {
	return { ...globalState };
}

function loadAsyncHelper() {
	let wasm: StarknetModule;

	async function loadWasm() {
		globalState.isLoading = true;

		wasm = await loadWasmLib();

		globalDeferredLoad.resolve();

		globalState.failedToLoad = wasm === undefined;
		globalState.isLoading = false;
	}

	loadWasm();

	return () => wasm;
}

// A function that will return the wasm lib if loaded globally or return undefined if not
export const getWasmLibIfLoaded = loadAsyncHelper();
