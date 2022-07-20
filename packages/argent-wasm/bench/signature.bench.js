const Benchmark = require('benchmark');
// in order to generate pkg_node build prover with: wasm-pack build --out-dir pkg_node --target nodejs
const { StarknetModule } = require("../../prover/pkg_node");
const { toBufferLE } = require('bigint-buffer');
const { ec } = require("starknet");

// wasm init
const BUFF_LEN = 32;
const private_key = 5n;

const starknet = new StarknetModule(); 
starknet.load_sk(toBufferLE(private_key, BUFF_LEN))

const wasmMsgHash = toBufferLE(1n, BUFF_LEN);

// native init
let keyPair = ec.getKeyPair(private_key);
let msgHash = 1n.toString();

const suite = new Benchmark.Suite();

suite
    .add('wasm-sign', () => {
        starknet.sign_hashed(wasmMsgHash)
    })
    .add('native-sign', () => {
        ec.sign(keyPair, msgHash);
    })
    .on('cycle', event => {
        const benchmark = event.target;
        console.log(benchmark.toString());
    })
    .on('complete', event => {
        const suite = event.currentTarget;
        const fastestOption = suite.filter('fastest').map('name');

        console.log(`Faster option is ${fastestOption}`);
    })
    .run();