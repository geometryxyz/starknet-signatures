# StarkNet signature
> Warning: this repo uses Cairo 0 and points to a deprecated verifier contract on the Starknet Goerli testnet.

## Arkworks and WASM meet StarkNet

This demo shows how you can use Rust and the <a href="https://github.com/arkworks-rs">arkworks ecosystem</a> to write webapps that perform cryptography compatible with <a href="https://starkware.co/starknet/">StarkNet</a>.

Following the steps on the [front end](https://geometryresearch.github.io/starknet-signatures/), you will be prompted to generate a key pair, sign a message and submit it to <a href="https://goerli.voyager.online/contract/0x026c8bc8bf071a54c4b0713ad52715fe92a471f85bf7f224322cbb0a29666ce1#transactions">our StarkNet contract</a> for verification. This final step requires an <a href="https://www.argent.xyz/argent-x/">Argent X wallet</a> (only compatible with Chrome and Firefox).

All the code performing cryptography was written in Rust using arkworks and <a href="https://github.com/geometryresearch/proof-toolbox/tree/main/starknet-curve">our implementation</a> of the <a href="https://starknet.io/docs/how_cairo_works/cairo_intro.html#field-elements">StarkNet finite field</a> and the <a href="https://docs.starkware.co/starkex-v4/crypto/stark-curve">STARK-friendly elliptic curve</a>. The code is then compiled to WebAssembly to be executed in browser. 


## License

&copy; 2022 [Geometry](https://geometryresearch.xyz).

This project is licensed under either of

- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([`LICENSE-APACHE`](LICENSE-APACHE))
- [MIT license](https://opensource.org/licenses/MIT) ([`LICENSE-MIT`](LICENSE-MIT))

at your option.

The [SPDX](https://spdx.dev) license identifier for this project is `MIT OR Apache-2.0`.
