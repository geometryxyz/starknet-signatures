// in order to generate pkg_node build prover with: wasm-pack build --out-dir pkg_node --target nodejs
const { StarknetModule } = require("../../../prover/pkg_node");
const { toBufferLE } = require('bigint-buffer');

// wasm init
const BUFF_LEN = 32;
const private_key = 5n;

const starknet = new StarknetModule(); 
starknet.load_sk(toBufferLE(private_key, BUFF_LEN))
const wasmMsgHash = toBufferLE(1n, BUFF_LEN);
starknet.sign_hashed(wasmMsgHash)
