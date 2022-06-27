#[macro_use]
extern crate lazy_static;

mod constants;
mod error;
mod pedersen;
mod rfc6979;
mod signature;

use ark_ec::ProjectiveCurve;
use ark_ff::{bytes::FromBytes, BigInteger, BigInteger256, FpParameters, PrimeField};
use js_sys::Uint8Array;
use starknet_curve::{Fr, FrParameters};

use error::Error;
use pedersen::compute_hash_on_elements;
use signature::{parameters, private_key_to_public_key, sign as starknet_sign};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub struct PublicKey {
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

    pub fn load_sk(&mut self, private_key: Vec<u8>) {
        self.private_key = Some(private_key.clone())
    }

    #[wasm_bindgen(catch)]
    pub fn get_private_key(&self) -> Result<Uint8Array, JsValue> {
        let pk_bytes = self.private_key.clone().ok_or("No private key provided")?;
        Ok(Uint8Array::from(&pk_bytes[..]))
    }

    #[wasm_bindgen(catch)]
    pub fn get_public_key(&self) -> Result<PublicKey, JsValue> {
        let pk_bytes = self.private_key.clone().ok_or("No private key provided")?;
        let private_key = Fr::from_be_bytes_mod_order(pk_bytes.as_slice());

        let parameters = parameters();
        let public_key = private_key_to_public_key(&parameters, private_key).into_affine();

        Ok(PublicKey::new(
            public_key.x.into_repr().to_bytes_be(),
            public_key.y.into_repr().to_bytes_be(),
        ))
    }

    #[wasm_bindgen(catch)]
    pub fn sign(&self, felts: js_sys::Array) -> Result<Signature, JsValue> {
        let parameters = parameters();

        let pk_bytes = self.private_key.clone().ok_or("No private key provided")?;
        let private_key = Fr::from_be_bytes_mod_order(pk_bytes.as_slice());

        let felts = self.parse_felts(felts).map_err(|e| e.to_jsval())?;
        let msg_hash = compute_hash_on_elements(&felts).map_err(|e| e.to_jsval())?;

        let sig = starknet_sign(&parameters, private_key, msg_hash, None)
            .ok_or("message is out of bound")?;

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
        let private_key = Fr::from_be_bytes_mod_order(private_key_bytes.as_slice());

        let felts = self.parse_felts(felts).map_err(|e| e.to_jsval())?;
        let msg_hash = compute_hash_on_elements(&felts).map_err(|e| e.to_jsval())?;

        let sig = starknet_sign(&parameters, private_key, msg_hash, None)
            .ok_or("message is out of bound")?;

        Ok(Signature::new(
            sig.r.into_repr().to_bytes_le(),
            sig.s.into_repr().to_bytes_le(),
        ))
    }

    fn parse_felts(&self, felts: js_sys::Array) -> Result<Vec<Fr>, Error> {
        let felts: Result<Vec<Uint8Array>, JsValue> = felts
            .values()
            .into_iter()
            .map(|felt| felt.unwrap_throw().dyn_into::<Uint8Array>())
            .collect();

        let felts: Vec<Vec<u8>> = felts?.iter().map(|x| x.to_vec()).collect();

        felts
            .iter()
            .map(|felt_bytes| -> Result<Fr, Error> {
                if felt_bytes.len() != 32 {
                    return Err(Error::IncorrectLenError);
                }

                let repr =
                    BigInteger256::read(felt_bytes.as_slice()).map_err(|_| Error::IOError)?;

                if repr > FrParameters::MODULUS {
                    return Err(Error::OverflowError);
                }

                Ok(Fr::from_repr(repr).unwrap())
            })
            .collect::<Result<Vec<_>, Error>>()
    }
}
