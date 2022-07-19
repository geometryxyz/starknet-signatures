import React from "react";

import { StarknetModule } from "starknet-signature";
import * as toBuffer from "typedarray-to-buffer"; // toBuffer function enables converting Uint8Array to Buff without copying
import { toBufferLE, toBigIntLE } from "bigint-buffer";

import {
  StarknetProvider,
  useStarknet,
  useConnectors,
} from "@starknet-react/core";

const App = () => {
  const starknet = new StarknetModule();
  const { available, connect, disconnect } = useConnectors();
  const { account } = useStarknet();

  return (
    <StarknetProvider>
      <div>
        <p>account {account ?? "not connected"}</p>
        {account === undefined ? (
          <div>
            {available.map((connector) => (
              <button key={connector.id()} onClick={() => connect(connector)}>
                {`Connect ${connector.name()}`}
              </button>
            ))}
          </div>
        ) : (
          <button onClick={() => disconnect()}>Disconnect</button>
        )}
        <p>dddd</p>
      </div>
    </StarknetProvider>
  );
};

export default App;
