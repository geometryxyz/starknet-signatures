import { StarknetModule } from 'starknet-signature';
import * as toBuffer from 'typedarray-to-buffer'; // toBuffer function enables converting Uint8Array to Buff without copying
import { toBufferLE, toBigIntLE } from 'bigint-buffer';

const BUFF_LEN = 32;

const private_key = 5n;

const felts = [1n, 2n, 3n, 4n, 5n];
const felts_le = felts.map((felt) => toBufferLE(felt, BUFF_LEN));
// element bigger than Fq
const overflow_felts = [3618502788666131213697322783095070105623107215331596699973092056135872020485n];
const overflow_felts_le = overflow_felts.map((felt) => toBufferLE(felt, BUFF_LEN));

const wrong_len_felts = [1n];
const wrong_len_felts_le = wrong_len_felts.map((felt) => toBufferLE(felt, 31));

const incorrect_type_felts = [1, 2, 3];

///STARKNET MODULE USAGE
const starknet = new StarknetModule(); 

// expect error since no private key is provided
try { 
    starknet.get_private_key()
} catch(err) { 
    console.log('Err: ', err)
}

starknet.load_sk(toBufferLE(private_key, BUFF_LEN))
console.log('private key: ', toBigIntLE(toBuffer(starknet.get_private_key())))

const pk = starknet.get_public_key()
console.log('x', toBigIntLE(toBuffer(pk.get_x())))
console.log('y', toBigIntLE(toBuffer(pk.get_y())))

// valid signature
const signature = starknet.sign(felts_le)
console.log('r', toBigIntLE(toBuffer(signature.get_r())))
console.log('s', toBigIntLE(toBuffer(signature.get_s())))

// valid signature
const signature2 = starknet.sign_with_external_sk(toBufferLE(private_key, BUFF_LEN), felts_le)
console.log('r2', toBigIntLE(toBuffer(signature2.get_r())))
console.log('s2', toBigIntLE(toBuffer(signature2.get_s())))

// expect overflow
try { 
    starknet.sign(overflow_felts_le)
} catch(err) { 
    console.log('Err: ', err)
}

// expect len error
try { 
    starknet.sign(wrong_len_felts_le)
} catch(err) { 
    console.log('Err: ', err)
}

// expect wrong type
try { 
    starknet.sign(incorrect_type_felts)
} catch(err) { 
    console.log('Err: ', err)
}