#[macro_use]
extern crate lazy_static;

mod constants;
mod error;
mod pedersen;
mod rfc6979;
mod signature;

use ark_ec::ProjectiveCurve;
use ark_ff::{BigInteger, PrimeField};
use js_sys::Uint8Array;
use starknet_curve::Fr;

use pedersen::unsafe_hash_to_field;
use signature::{parameters, private_key_to_public_key, sign as starknet_sign};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct PulicKey {
    x: Vec<u8>,
    y: Vec<u8>,
}

#[wasm_bindgen]
impl PulicKey {
    #[wasm_bindgen(constructor)]
    pub fn new(x: Vec<u8>, y: Vec<u8>) -> Self {
        Self { x, y }
    }

    pub fn get_x(&self) -> Uint8Array {
        Uint8Array::from(&self.x[..])
    }

    pub fn get_y(&self) -> Uint8Array {
        Uint8Array::from(&self.y[..])
    }
}

#[wasm_bindgen]
pub struct Signature {
    r: Vec<u8>,
    s: Vec<u8>,
}

#[wasm_bindgen]
impl Signature {
    #[wasm_bindgen(constructor)]
    pub fn new(r: Vec<u8>, s: Vec<u8>) -> Self {
        Self { r, s }
    }

    pub fn get_r(&self) -> Uint8Array {
        Uint8Array::from(&self.r[..])
    }

    pub fn get_s(&self) -> Uint8Array {
        Uint8Array::from(&self.s[..])
    }
}

#[wasm_bindgen]
pub struct StarknetModule {
    private_key: Option<Vec<u8>>,
}

#[wasm_bindgen]
impl StarknetModule {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { private_key: None }
    }

    pub fn load_pk(&mut self, private_key: Vec<u8>) {
        self.private_key = Some(private_key.clone())
    }

    pub fn get_private_key(&self) -> Result<Uint8Array, JsValue> {
        let pk_bytes = self.private_key.clone().expect("No private key provided");
        Ok(Uint8Array::from(&pk_bytes[..]))
    }

    pub fn get_public_key(&self) -> Result<PulicKey, JsValue> {
        let pk_bytes = self.private_key.clone().expect("No private key provided");
        let private_key = Fr::from_be_bytes_mod_order(pk_bytes.as_slice());

        let parameters = parameters();
        let public_key = private_key_to_public_key(&parameters, private_key).into_affine();

        Ok(PulicKey::new(
            public_key.x.into_repr().to_bytes_be(),
            public_key.y.into_repr().to_bytes_be(),
        ))
    }

    pub fn sign(&self, msg: &str) -> Result<Signature, JsValue> {
        let parameters = parameters();

        let pk_bytes = self.private_key.clone().expect("No private key provided");
        let private_key = Fr::from_be_bytes_mod_order(pk_bytes.as_slice());

        let msg_hash = unsafe_hash_to_field(msg.as_bytes()).expect("Hash failed");
        let sig = starknet_sign(&parameters, private_key, msg_hash, None)
            .expect("Message is out of bound");

        Ok(Signature::new(
            sig.r.into_repr().to_bytes_le(),
            sig.s.into_repr().to_bytes_le(),
        ))
    }

    pub fn sign_with_pk(&self, pk_bytes: Vec<u8>, msg: &str) -> Result<Signature, JsValue> {
        let parameters = parameters();

        let private_key = Fr::from_be_bytes_mod_order(pk_bytes.as_slice());
        let msg_hash = unsafe_hash_to_field(msg.as_bytes()).expect("Hash failed");
        let sig = starknet_sign(&parameters, private_key, msg_hash, None)
            .expect("Message is out of bound");

        Ok(Signature::new(
            sig.r.into_repr().to_bytes_le(),
            sig.s.into_repr().to_bytes_le(),
        ))
    }
}
