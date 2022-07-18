import React from "react";
import { StarknetProvider, getInstalledInjectedConnectors, useStarknet, useConnectors } from '@starknet-react/core';
import { StarknetModule } from 'starknet-signature';
import { toBufferLE, toBigIntLE } from 'bigint-buffer';
import * as toBuffer from "typedarray-to-buffer";

const BUFF_LEN = 32;
const private_key = 5n;

const felts = [1n, 2n, 3n, 4n, 5n];
const felts_le = felts.map((felt) => toBufferLE(felt, BUFF_LEN));

const starknet = new StarknetModule();

starknet.load_sk(toBufferLE(private_key, BUFF_LEN))

const pk = starknet.get_public_key()
console.log('x', toBigIntLE(toBuffer(pk.get_x())))
console.log('y', toBigIntLE(toBuffer(pk.get_y())))

const signature = starknet.sign(felts_le)
console.log('r', toBigIntLE(toBuffer(signature.get_r())))
console.log('s', toBigIntLE(toBuffer(signature.get_s())))

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
  
    return <div>gm {account}</div>
}

const App = () => {
    const connectors = getInstalledInjectedConnectors()

    return (
      <StarknetProvider connectors={connectors}>
        <WalletComponent />
        <AccComponent />
      </StarknetProvider>
    )
}

export default App;