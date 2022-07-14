const { pedersen } = require("../stark-cpp.js");

const computeHashOnElements = (data) => {
    return [...data, BigInt(data.length)].reduce((x, y) => pedersen(x, y), BigInt(0)).toString();
}

const one = BigInt(1); 
const two = BigInt(2);
const three = BigInt(3); 
const four = BigInt(4); 
const five = BigInt(5); 

const felts = [one, two, three, four, five, one, two, three, four, five, one, two, three, four, five, one, two, three, four, five, one, two, three, four, five, one, two, three, four, five, 
    one, two, three, four, five, one, two, three, four, five];

computeHashOnElements(felts);