import {
  StarknetProvider,
  getInstalledInjectedConnectors,
  useStarknet,
  useConnectors,
  useStarknetInvoke,
  useContract,
} from "@starknet-react/core";
import { toBufferLE, toBigIntLE } from "bigint-buffer";
import * as toBuffer from "typedarray-to-buffer";
import React, { useEffect, useState } from "react";
import { useBetween } from "use-between";

import VerifySigAbi from "./abi/contract.json";
import Erc20Abi from "./abi/erc20.json";

import { starknet } from "./";

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
    secretKey,
    setSecretKey,
    public_key_x,
    setPkX,
    public_key_y,
    setPkY,
    feltsToSign,
    setFelts,
    sig_x,
    setSigX,
    sig_y,
    setSigY,
  };
};

const useSharedFormState = () => useBetween(useFormState);

function SKGeneratorComponent() {
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
        Generate random key pair
      </button>
    </div>
  );
};

function KeyGeneration() {
  return(
    <div id="keygen_module">
      <h2>Step 1: Key Generation</h2>
      <SKGeneratorComponent />
      <PKDisplayComponent/>
      <br></br>
    </div>
  );
}

function Signature() {
  return(
    <div id="signing_module">
      <h2>Step 2: Create and Sign a Message</h2> 
      <p>Messages should be input as Cairo "felts" in hexadecimal representation.</p> 
        <MessageInputComponent/>
        <SignComponent></SignComponent>
        <SignatureDisplayComponent></SignatureDisplayComponent>
        <br></br>
    </div>
  );
}

function SubmitToStarkNet() {
  return(
    <div id="submission_module">
        <h2>Step 3: Submit to StarkNet for Verification</h2>
        <WalletComponent />
        <AccComponent />
    </div>
  );
}

const PKDisplayComponent = () => {
  const { public_key_x, public_key_y, secretKey } = useSharedFormState();

  return (
    <div>
      sk: {secretKey ? secretKey.toString() : "Empty"} <br></br>
      pk_x: {public_key_x ? public_key_x.toString() : "Empty"} <br></br>
      pk_y: {public_key_y ? public_key_y.toString() : "Empty"} <br></br>
    </div>
  );
};


const MessageInputComponent = () => {
  const { setFelts } = useSharedFormState();

  const [message, setMessage] = useState([]);
  const [inputSize, setInputSize] = useState(0);
  const [currentFelt, setCurrentFelt] = useState("");

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
                setFelts([...message, currentFelt]);
              }
            }}
            disabled={!isNumeric(currentFelt)}
          >
            confirm
          </button>
        </div>
      </table>
    </div>
  );
};

const SignComponent = () => {
  const { feltsToSign, setSigX, setSigY } = useSharedFormState();
  return (
    <div>
      <button
        onClick={() => {
          const felts_le = feltsToSign.map((felt) =>
            toBufferLE(felt, BUFF_LEN)
          );
          const signature = starknet.sign(felts_le);

          setSigX(toBigIntLE(toBuffer(signature.get_r())));
          setSigY(toBigIntLE(toBuffer(signature.get_s())));
        }}
      >
        Sign
      </button>
    </div>
  );
};

const SignatureDisplayComponent = () => {
  const { sig_x, sig_y } = useSharedFormState();

  return (
    <div>
      sig_x: {sig_x ? sig_x.toString() : ""} <br></br>
      sig_y: {sig_y ? sig_y.toString() : ""} <br></br>
    </div>
  );
};

const WalletComponent = () => {
  const { connect, connectors } = useConnectors();
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
  );
};

const VerifySignatureComponent = () => {
  const contractAddress =
    "0x026c8bc8bf071a54c4b0713ad52715fe92a471f85bf7f224322cbb0a29666ce1";
  const contract = useContract({
    abi: VerifySigAbi,
    address: contractAddress,
  });
  const method = "verify_signature";
  const { invoke } = useStarknetInvoke({
    contract,
    method,
  });
  const { sig_x, sig_y, feltsToSign, public_key_x } = useSharedFormState();

  const onClick = async () => {
    const res = await invoke({
      args: [feltsToSign.length, ...feltsToSign, public_key_x, [sig_x, sig_y]],
      metadata: {
        method: "verifySignature",
        message: "verifying signature",
      },
    });
    console.log(res);
  };

  return (
    <button onClick={onClick} disabled={!sig_x && !sig_y}>
      Verify on starknet
    </button>
  );
};

const FaucetComponent = () => {
  // const { account } = useStarknet();
  // const token = useContract({
  //   abi: Erc20Abi,
  //   address:
  //     "0x07394cbe418daa16e42b87ba67372d4ab4a5df0b05c6e554d158458ce245bc10",
  // });
  // const { data, loading, error, refresh } = useStarknetCall({
  //   contract: token,
  //   method: balanceOf,
  //   args: [account],
  // });

  // const [balance, setBalance] = useState("");

  return <a href="https://faucet.goerli.starknet.io/#">Request testnet eth</a>;
};

const AccComponent = () => {
  const { account } = useStarknet();

  return <div>Active account: {account}</div>;
};

function App() {
  const connectors = getInstalledInjectedConnectors()

  return (
    <StarknetProvider connectors={connectors}>
      <KeyGeneration />
      <Signature />
      <SubmitToStarkNet />
    </StarknetProvider>
  )
}

export default App;
