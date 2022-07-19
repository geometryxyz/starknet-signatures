import { StarknetProvider, getInstalledInjectedConnectors, useStarknet, useConnectors } from '@starknet-react/core';
import { toBufferLE, toBigIntLE } from 'bigint-buffer';
import * as toBuffer from "typedarray-to-buffer";
import React, { useState } from 'react';
import { useBetween } from "use-between";

import {starknet} from "./";

const BUFF_LEN = 32;

function isNumeric(value) {
	return /^-?\d+$/.test(value);
}

const useFormState = () => {
  const [secretKey, setSecretKey] = useState(null);
  const [public_key_x, setPkX] = useState(null);
  const [public_key_y, setPkY] = useState(null);
  const [sig_x, setSigX] = useState(null);
  const [sig_y, setSigY] = useState(null);

  const [feltsToSign, setFelts] = useState([]);

  return {
    secretKey, setSecretKey, public_key_x, setPkX, public_key_y, setPkY, feltsToSign, setFelts, sig_x, setSigX, sig_y, setSigY
  };
};

const useSharedFormState = () => useBetween(useFormState);

const SKGeneratorComponent = () => {
  const { setSecretKey, setPkX, setPkY } = useSharedFormState();
  return (
    <div>
      <button onClick={() => {
        const private_key = 5n;
        starknet.load_sk(toBufferLE(private_key, BUFF_LEN));
        const pk = starknet.get_public_key()
        setPkX(toBigIntLE(toBuffer(pk.get_x())));
        setPkY(toBigIntLE(toBuffer(pk.get_y())));
        setSecretKey(private_key)
      }}>
        Generate new Private Key
      </button>
    </div>
  );
}

const PKDisplayComponent = () => {
  const { public_key_x, public_key_y, secretKey } = useSharedFormState();

  return(
    <div>
      sk:  {secretKey ? secretKey.toString() : "Empty" } <br></br>
      pk_x:  {public_key_x ? public_key_x.toString() : "Empty" } <br></br>
      pk_y:  {public_key_y ? public_key_y.toString() : "Empty" } <br></br>
    </div>
  )
}

const InputComponent = () => {
  const { setFelts } = useSharedFormState();

  const [message, setMessage] = useState([]);
	const [inputSize, setInputSize] = useState(0);
	const [currentFelt, setCurrentFelt] = useState('');
  
  return (
    <div>
      <table>
				<td key={0}>
					<th>Message</th>
				</td>
				{new Array(inputSize).fill(0).map((e, i) => (
					<td key={i + 1}>
						<th>Felt {message[i]}</th>
					</td>
				))}
				<div>
					<input
						onChange={(e) => {
							const input = e.target.value;
							setCurrentFelt(input);
						}}
					/>
					<button
						onClick={() => {
							if (isNumeric(currentFelt)) {
								setMessage([...message, currentFelt]);
								setInputSize(inputSize + 1);
                setFelts([...message, currentFelt])
							}
						}}
						disabled={!isNumeric(currentFelt)}
					>
						confirm
					</button>
				</div>
			</table>
    </div>
  )
}

const SignComponent = () => {
  const { feltsToSign, setSigX, setSigY } = useSharedFormState();
  return (
    <div>
      <button onClick={() => {
        const felts_le = feltsToSign.map((felt) => toBufferLE(felt, BUFF_LEN));
        const signature = starknet.sign(felts_le)

        setSigX(toBigIntLE(toBuffer(signature.get_r())));
        setSigY(toBigIntLE(toBuffer(signature.get_s())));
        
      }}>Sign</button>
    </div>
  )
}

const SignatureDisplayComponent = () => {
  const { sig_x, sig_y } = useSharedFormState();

  return(
    <div>
      sig_x:  {sig_x ? sig_x.toString() : "" } <br></br>
      sig_y:  {sig_y ? sig_y.toString() : "" } <br></br>
    </div>
  )
}

const WalletComponent = () => {
    const { connect, connectors } = useConnectors()
    return (
      <div>
        {connectors.map((connector) =>
          connector.available() ? (
            <button key={connector.id()} onClick={() => connect(connector)}>
              Connect {connector.name()}
            </button>
          ) : null
        )}
      </div>
    )
}

const AccComponent = () => {
    const { account } = useStarknet()
  
    return <div>Active account: {account}</div>
}

const App = () => {
    const connectors = getInstalledInjectedConnectors()

    return (
      <StarknetProvider connectors={connectors}>
        <WalletComponent />
        <AccComponent />
        <SKGeneratorComponent/>
        <PKDisplayComponent/>
        <InputComponent/>
        <SignComponent></SignComponent>
        <SignatureDisplayComponent></SignatureDisplayComponent>
      </StarknetProvider>
    )
}

export default App;