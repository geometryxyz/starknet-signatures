use ark_ff::{BigInteger, PrimeField};
use crypto_bigint::{
    subtle::ConstantTimeLess, ArrayEncoding, ByteArray, Encoding, Integer, Zero as BigIntZero, U256,
};
use generic_array::GenericArray;
use rand::thread_rng;
use rfc6979::HmacDrbg;
use sha2::Sha256;
use starknet_curve::{Affine, Fr, Projective};
use zeroize::{Zeroize, Zeroizing};

pub fn generate_k_rfc6979_arkworks<B: BigInteger, F: PrimeField>(
    ec_order: &B,
    key: &F,
    msg_hash: &F,
    seed: Option<u64>,
) -> F {
    assert!(F::size_in_bits() <= 256);
    // assert that field num of bits is less then 256 (until we make this more generic)

    let ec_order = U256::from_be_byte_array(GenericArray::clone_from_slice(
        ec_order.to_bytes_be().as_slice(),
    ));
    let key = U256::from_be_byte_array(GenericArray::clone_from_slice(
        key.into_repr().to_bytes_be().as_slice(),
    ));
    let msg_hash = U256::from_be_byte_array(GenericArray::clone_from_slice(
        msg_hash.into_repr().to_bytes_be().as_slice(),
    ));

    let seed = match seed {
        None => b"".to_vec(),
        Some(value) => value.to_be_bytes().to_vec(),
    };

    let mut key = key.to_be_byte_array();
    let mut hmac_drbg = HmacDrbg::<Sha256>::new(&key, &msg_hash.to_be_byte_array(), &seed);
    key.zeroize();

    loop {
        let mut bytes = ByteArray::<U256>::default();
        hmac_drbg.fill_bytes(&mut bytes);
        let k = U256::from_be_byte_array(bytes) >> 4;

        if (!k.is_zero() & k.ct_lt(&ec_order)).into() {
            // return Zeroizing::new(k);
            return F::from_be_bytes_mod_order(&k.to_be_byte_array());
        }
    }
}

pub fn generate_k_rfc6979(
    ec_order: &U256,
    key: &U256,
    msg_hash: &U256,
    additional_data: &[u8],
) -> Zeroizing<U256> {
    let mut key = key.to_be_byte_array();
    let mut hmac_drbg =
        HmacDrbg::<Sha256>::new(&key, &msg_hash.to_be_byte_array(), additional_data);
    key.zeroize();

    loop {
        let mut bytes = ByteArray::<U256>::default();
        hmac_drbg.fill_bytes(&mut bytes);
        let k = U256::from_be_byte_array(bytes) >> 4;

        if (!k.is_zero() & k.ct_lt(&ec_order)).into() {
            return Zeroizing::new(k);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::generate_k_rfc6979;
    use crypto_bigint::{ArrayEncoding, U256};
    use rfc6979::{generate_k, HmacDrbg};
    use sha2::Sha256;
    use zeroize::{Zeroize, Zeroizing};

    #[test]
    pub fn test_k_without_shifting() {
        let ec_order =
            U256::from_be_hex("0800000000000010ffffffffffffffffb781126dcae7b2321e66a241adc64d2f");
        let key =
            U256::from_be_hex("0000000000000000000000000000000000000000000000000000000000000001");
        let msg_hash =
            U256::from_be_hex("0000000000000000000000000000000000000000000000000000000000000005");

        let empty_data = b"";
        let k = generate_k::<Sha256, U256>(
            &key,
            &ec_order,
            &msg_hash.to_be_byte_array(),
            empty_data.as_slice(),
        );

        let k = U256::from_be_byte_array(k.to_be_byte_array());
        assert_eq!(
            k,
            U256::from_be_hex("019D482B334A0B9F7E335A96AF94AB94DAE0F18D40E7DBC8A47D4427E0EFB480")
        );
    }

    pub fn generate_k_with_shifting() -> Zeroizing<U256> {
        let ec_order =
            U256::from_be_hex("0800000000000010ffffffffffffffffb781126dcae7b2321e66a241adc64d2f");
        let key =
            U256::from_be_hex("0000000000000000000000000000000000000000000000000000000000000001");
        let msg_hash =
            U256::from_be_hex("0000000000000000000000000000000000000000000000000000000000000005");
        // let msg_hash = msg_hash.to_be_byte_array();
        let empty_data = b"";
        let empty_data = empty_data.as_slice();

        generate_k_rfc6979(&ec_order, &key, &msg_hash, &empty_data)
    }

    #[test]
    fn test_generate_k_with_shifting() {
        let k = generate_k_with_shifting();

        let k = U256::from_be_byte_array(k.to_be_byte_array());
        assert_eq!(
            k,
            U256::from_be_hex("02707E03E7F40F39667D5ACD867D25D6E29FF18976642E7F9BD45D0F07D57B17")
        );
    }
}
