import { StarknetModule } from "starknet-signature";
import { bufToBigint } from 'bigint-conversion'

const starknet = new StarknetModule(); 

const private_key = Uint8Array.from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5])
starknet.load_pk(private_key)
console.log('private key: ', bufToBigint(starknet.get_private_key()))

const pk = starknet.get_public_key();
console.log('x: ', bufToBigint(pk.get_x()))
console.log('y: ', bufToBigint(pk.get_y()))

const signature1 = starknet.sign("hello hackers!")
console.log('r1: ', bufToBigint(signature1.get_r()))
console.log('s1: ', bufToBigint(signature1.get_s()))

const signature2 = starknet.sign_with_pk(private_key, "hello hackers!")
console.log('r2: ', bufToBigint(signature2.get_r()))
console.log('s2: ', bufToBigint(signature2.get_s()))