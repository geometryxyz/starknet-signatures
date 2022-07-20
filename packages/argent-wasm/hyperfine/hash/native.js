const { hash } = require("starknet");
const felts = [1n, 2n, 3n, 4n, 5n, 1n, 2n, 3n, 4n, 5n, 1n, 2n, 3n, 4n, 5n, 1n, 2n, 3n, 4n, 5n, 1n, 2n, 3n, 4n, 5n, 1n, 2n, 3n, 4n, 5n, 1n, 2n, 3n, 4n, 5n, 1n, 2n, 3n, 4n, 5n];
hash.computeHashOnElements(felts);