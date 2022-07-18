// in order to generate pkg_node build prover with: wasm-pack build --out-dir pkg_node --target nodejs
const { StarknetModule } = require("../../../prover/pkg_node/starknet_signature");
const { toBufferLE } = require('bigint-buffer');


// wasm init
const BUFF_LEN = 32;
const starknet = new StarknetModule(); 

// shared 
const felts = [1n, 2n, 3n, 4n, 5n, 1n, 2n, 3n, 4n, 5n, 1n, 2n, 3n, 4n, 5n, 1n, 2n, 3n, 4n, 5n, 1n, 2n, 3n, 4n, 5n, 1n, 2n, 3n, 4n, 5n, 1n, 2n, 3n, 4n, 5n, 1n, 2n, 3n, 4n, 5n];
const felts_le = felts.map((felt) => toBufferLE(felt, BUFF_LEN));
starknet.hash_felts(felts_le)