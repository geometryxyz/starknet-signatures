console.log(process.cwd());
console.log(__dirname);

const { StarknetModule } = require(__dirname + '/../../prover/pkg-node');
const { toBufferLE, toBigIntLE } = require('bigint-buffer');

const BUFF_LEN = 32;

const private_key = 5n;

const felts = [1n, 2n, 3n, 4n, 5n];
const felts_le = felts.map((felt) => toBufferLE(felt, BUFF_LEN));
// element bigger than Fq
const overflow_felts = [
	3618502788666131213697322783095070105623107215331596699973092056135872020485n,
];
const overflow_felts_le = overflow_felts.map((felt) => toBufferLE(felt, BUFF_LEN));

const wrong_len_felts = [1n];
const wrong_len_felts_le = wrong_len_felts.map((felt) => toBufferLE(felt, 31));

const incorrect_type_felts = [1, 2, 3];

///STARKNET MODULE USAGE
const starknet = new StarknetModule();

// expect error since no private key is provided
try {
	starknet.get_private_key();
} catch (err) {
	console.log('Err: ', err);
}

starknet.load_sk(toBufferLE(private_key, BUFF_LEN));

const pk = starknet.get_public_key();

// valid signature
const signature = starknet.sign(felts_le);

// valid signature
const signature2 = starknet.sign_with_external_sk(toBufferLE(private_key, BUFF_LEN), felts_le);

// expect overflow
try {
	starknet.sign(overflow_felts_le);
} catch (err) {
	console.log('Err: ', err);
}

// expect len error
try {
	starknet.sign(wrong_len_felts_le);
} catch (err) {
	console.log('Err: ', err);
}

// expect wrong type
try {
	starknet.sign(incorrect_type_felts);
} catch (err) {
	console.log('Err: ', err);
}
