const { ec } = require("starknet");

const private_key = 5n;
let keyPair = ec.getKeyPair(private_key);
let msgHash = 1n.toString();
ec.sign(keyPair, msgHash);