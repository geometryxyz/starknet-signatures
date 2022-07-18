#[macro_use]
extern crate lazy_static;

mod constants;
mod error;
mod pedersen;
mod rfc6979;
mod signature;
mod util;

use ark_ec::ProjectiveCurve;
use ark_ff::UniformRand;
use ark_ff::{BigInteger, PrimeField};
use js_sys::Uint8Array;
use rand::rngs::OsRng;
use starknet_curve::{Fq, Fr};

use error::Error;
use pedersen::compute_hash_on_elements;
use signature::{parameters, private_key_to_public_key, sign as starknet_sign};
use util::{bytes_safe, try_bytes_to_field};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/*
   Bytes that are being sent from JS are in LE endianness
   For the sake of simplicity we store all bytes in LE endianness too
   This can be parametrized such that lib user can choose endianness if needed
*/

#[wasm_bindgen]
pub struct PublicKey {
    // store bytes in LE endianness as a convention
    x: Vec<u8>,
    y: Vec<u8>,
}

#[wasm_bindgen]
impl PublicKey {
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
    // store bytes in LE endianness as a convention
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
    // store bytes in LE endianness as a convention
    private_key: Option<Vec<u8>>,
}

#[wasm_bindgen]
impl StarknetModule {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { private_key: None }
    }

    pub fn new_sk(&mut self) {
        self.private_key = Some(Fr::rand(&mut OsRng).into_repr().to_bytes_be());
    }

    /// bytes are expected to be in LE representation
    pub fn load_sk(&mut self, private_key: Vec<u8>) -> Result<(), JsValue> {
        let repr = bytes_safe::<Fr>(&private_key).map_err(|e| e.to_jsval())?;
        self.private_key = Some(repr.to_bytes_le());

        Ok(())
    }

    #[wasm_bindgen(catch)]
    pub fn get_private_key(&self) -> Result<Uint8Array, JsValue> {
        let pk_bytes = self.private_key.clone().ok_or("No private key provided")?;
        Ok(Uint8Array::from(&pk_bytes[..]))
    }

    #[wasm_bindgen(catch)]
    pub fn get_public_key(&self) -> Result<PublicKey, JsValue> {
        let pk_bytes = self.private_key.clone().ok_or("No private key provided")?;
        let private_key = Fr::from_le_bytes_mod_order(pk_bytes.as_slice());

        let parameters = parameters();
        let public_key = private_key_to_public_key(&parameters, private_key).into_affine();

        Ok(PublicKey::new(
            public_key.x.into_repr().to_bytes_le(),
            public_key.y.into_repr().to_bytes_le(),
        ))
    }

    #[wasm_bindgen(catch)]
    pub fn sign(&self, felts: js_sys::Array) -> Result<Signature, JsValue> {
        let parameters = parameters();

        let pk_bytes = self.private_key.clone().ok_or("No private key provided")?;
        let private_key = Fr::from_le_bytes_mod_order(pk_bytes.as_slice());

        let felts = self.parse_felts(felts).map_err(|e| e.to_jsval())?;
        let msg_hash = compute_hash_on_elements(&felts).map_err(|e| e.to_jsval())?;

        let sig =
            starknet_sign(&parameters, private_key, msg_hash, None).map_err(|e| e.to_jsval())?;

        Ok(Signature::new(
            sig.r.into_repr().to_bytes_le(),
            sig.s.into_repr().to_bytes_le(),
        ))
    }

    #[wasm_bindgen(catch)]
    pub fn sign_with_external_sk(
        &self,
        private_key_bytes: Vec<u8>,
        felts: js_sys::Array,
    ) -> Result<Signature, JsValue> {
        let parameters = parameters();

        let private_key: Fr = try_bytes_to_field(&private_key_bytes).map_err(|e| e.to_jsval())?;
        let felts = self.parse_felts(felts).map_err(|e| e.to_jsval())?;
        let msg_hash = compute_hash_on_elements(&felts).map_err(|e| e.to_jsval())?;

        let sig =
            starknet_sign(&parameters, private_key, msg_hash, None).map_err(|e| e.to_jsval())?;

        Ok(Signature::new(
            sig.r.into_repr().to_bytes_le(),
            sig.s.into_repr().to_bytes_le(),
        ))
    }

    #[wasm_bindgen]
    pub fn sign_hashed(&self, msg_hash_bytes: Vec<u8>) -> Result<Signature, JsValue> {
        let parameters = parameters();

        let pk_bytes = self.private_key.clone().ok_or("No private key provided")?;
        let private_key = Fr::from_le_bytes_mod_order(pk_bytes.as_slice());

        let msg_hash = try_bytes_to_field(&msg_hash_bytes).map_err(|e| e.to_jsval())?;

        let sig =
            starknet_sign(&parameters, private_key, msg_hash, None).map_err(|e| e.to_jsval())?;

        Ok(Signature::new(
            sig.r.into_repr().to_bytes_le(),
            sig.s.into_repr().to_bytes_le(),
        ))
    }

    #[wasm_bindgen]
    pub fn hash_felts(&self, felts: js_sys::Array) -> Result<Vec<u8>, JsValue> {
        let felts = self.parse_felts(felts).map_err(|e| e.to_jsval())?;
        let msg_hash = compute_hash_on_elements(&felts).map_err(|e| e.to_jsval())?;

        Ok(msg_hash.into_repr().to_bytes_le())
    }

    /// felts are interpreted in le form since FromBytes expects LE representation
    fn parse_felts(&self, felts: js_sys::Array) -> Result<Vec<Fq>, Error> {
        let felts: Result<Vec<Uint8Array>, JsValue> = felts
            .values()
            .into_iter()
            .map(|felt| felt.unwrap_throw().dyn_into::<Uint8Array>())
            .collect();

        let felts: Vec<Vec<u8>> = felts?.iter().map(|x| x.to_vec()).collect();

        felts
            .iter()
            .map(|felt_bytes| -> Result<Fq, Error> { try_bytes_to_field(felt_bytes) })
            .collect::<Result<Vec<_>, Error>>()
    }
}
