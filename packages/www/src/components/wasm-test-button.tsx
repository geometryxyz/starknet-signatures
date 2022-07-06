import React, { useState, useEffect } from 'react';

import { getGlobalWasmState, getWasmLibIfLoaded } from 'wasm/wasm-loader';

import toBuffer from 'typedarray-to-buffer';
import { toBufferLE, toBigIntLE } from 'bigint-buffer';

import { joinSignature } from '@ethersproject/bytes';

function isNumeric(value: any) {
	return /^-?\d+$/.test(value);
}

const BUFF_LEN = 32;

export default function WasmTestButton() {
	const [wasmGreeting, setWasmGreeting] = useState('');

	const [privateKey, setPrivateKey] = useState<string>();
	const [isPrivateKeyValid, setIsPrivateKeyValid] = useState<boolean>(false);

	const [isMessageCorrect, setIsMessageCorrect] = useState<boolean>(false);
	const [message, setMessage] = useState<string[]>([]);

	const [signature, setSignature] = useState<{ r: string; s: string }>();

	const [wasmLoaded, setWasmLoaded] = useState<false | true | 'failed'>(false);

	// Wait for wasm to be loaded
	useEffect(() => {
		const waitForWasm = async () => {
			const { promise } = getGlobalWasmState();
			await promise;
			const { failedToLoad } = getGlobalWasmState();
			setWasmLoaded(failedToLoad ? 'failed' : true);
		};
		waitForWasm();
	}, []);

	const onClick = () => {
		const wasm = getWasmLibIfLoaded();
		wasm.load_sk(toBufferLE(BigInt(privateKey!), BUFF_LEN));

		const felts = message.map((e) => BigInt(e));
		const felts_le = felts.map((felt) => toBufferLE(felt, BUFF_LEN));

		if (wasm) {
			const sig = wasm.sign(felts_le);
			const r = toBigIntLE(toBuffer(sig.get_r()));
			const s = toBigIntLE(toBuffer(sig.get_s()));
			setSignature({ r: r.toString(), s: s.toString() });
		}
	};

	const textToDisplay = () => {
		switch (wasmLoaded) {
			case true:
				return wasmGreeting;
			case false:
				return 'Loading....';
			case 'failed':
				return 'Failed to load';
		}
	};

	return (
		<div>
			<p>Your private key: </p>
			<input
				onChange={(e: any) => {
					const isInputNumberic = isNumeric(e.target.value);
					setIsPrivateKeyValid(isNumeric(e.target.value));
					if (isInputNumberic) {
						setPrivateKey(e.target.value);
					}
				}}
			/>
			<br />
			<a>Key correct: {isPrivateKeyValid ? '✓' : '⛔'}</a>
			<br />
			<p>Message:</p>
			<input
				onChange={(e: any) => {
					const input = e.target.value as string;
					const felts = input.split(',');
					const isInputValid = felts.every((e) => isNumeric(e));
					if (isInputValid) {
						setIsMessageCorrect(true);
						setMessage(felts);
					} else {
						setIsMessageCorrect(false);
					}
				}}
			/>
			<br />
			<a>Message correct: {isMessageCorrect ? '✓' : '⛔'}</a>
			<br />
			<button disabled={!(isMessageCorrect && isPrivateKeyValid)} onClick={onClick}>
				Sign
			</button>
			<br />
			<div>
				{signature && (
					<div>
						<a>Signature r: {'0x' + BigInt(signature.r).toString(16)}</a>
						<br />
						<a>Signature s: {'0x' + BigInt(signature.s).toString(16)}</a>
					</div>
				)}
			</div>
		</div>
	);
}
