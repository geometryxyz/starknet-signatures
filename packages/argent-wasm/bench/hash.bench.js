const Benchmark = require('benchmark');
// in order to generate pkg_node build prover with: wasm-pack build --out-dir pkg_node --target nodejs
const { StarknetModule } = require("../../prover/pkg_node");
const { toBufferLE } = require('bigint-buffer');
const { hash } = require("starknet");

// wasm init
const BUFF_LEN = 32;
const starknet = new StarknetModule(); 

// shared 
const felts = [1n, 2n, 3n, 4n, 5n, 1n, 2n, 3n, 4n, 5n, 1n, 2n, 3n, 4n, 5n, 1n, 2n, 3n, 4n, 5n, 1n, 2n, 3n, 4n, 5n, 1n, 2n, 3n, 4n, 5n, 1n, 2n, 3n, 4n, 5n, 1n, 2n, 3n, 4n, 5n];

const suite = new Benchmark.Suite();

suite
    .add('wasm-hash', () => {
        const felts_le = felts.map((felt) => toBufferLE(felt, BUFF_LEN));
        starknet.hash_felts(felts_le)
    })
    .add('native-hash', () => {
        hash.computeHashOnElements(felts);
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