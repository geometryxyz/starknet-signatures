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
	const [privateKey, setPrivateKey] = useState<string>();
	const [isPrivateKeyValid, setIsPrivateKeyValid] = useState<boolean>(false);

	const [message, setMessage] = useState<string[]>([]);
	const [inputSize, setInputSize] = useState<number>(0);
	const [currentFelt, setCurrentFelt] = useState('');

	const [signature, setSignature] = useState<{ r: string; s: string }>();

	// Wait for wasm to be loaded
	useEffect(() => {
		const waitForWasm = async () => {
			const { promise } = getGlobalWasmState();
			await promise;
			const { failedToLoad } = getGlobalWasmState();
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

			<table>
				<tr>
					<th>Message</th>
				</tr>
				{new Array(inputSize).fill(0).map((e, i) => (
					<tr>
						<th>Felt {message[i]}</th>
					</tr>
				))}
				<div>
					<input
						onChange={(e: any) => {
							const input = e.target.value as string;
							setCurrentFelt(input);
						}}
					/>
					<button
						onClick={() => {
							if (isNumeric(currentFelt)) {
								setMessage([...message, currentFelt]);
								setInputSize(inputSize + 1);
							}
						}}
						disabled={!isNumeric(currentFelt)}
					>
						confirm
					</button>
				</div>
			</table>
			<br />
			<br />
			<button disabled={!isPrivateKeyValid} onClick={onClick}>
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
