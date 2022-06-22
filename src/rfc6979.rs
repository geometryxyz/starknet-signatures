use ark_ff::{BigInteger, PrimeField};
use crypto_bigint::Integer;
use crypto_bigint::{subtle::ConstantTimeLess, ArrayEncoding, ByteArray, Zero as BigIntZero, U256};
use generic_array::GenericArray;
use rfc6979::HmacDrbg;
use sha2::Sha256;
use zeroize::{Zeroize, Zeroizing};

pub fn generate_k_rfc6979<B: BigInteger, F: PrimeField>(
    ec_order: &B,
    key: &F,
    msg_hash: &F,
    seed: Option<u64>,
) -> F {
    // assert that field num of bits is less then 256 (until we make this more generic)
    let block_size = 256;
    let shifting_factor: i64 = block_size as i64 - 252 as i64; // TODO replace 252 with F::size_in_bits()
    assert!(shifting_factor >= 0);

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
        let k = U256::from_be_byte_array(bytes) >> shifting_factor as usize;

        if (!k.is_zero() & k.ct_lt(&ec_order)).into() {
            return F::from_be_bytes_mod_order(&k.to_be_byte_array());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::generate_k_rfc6979;
    use ark_ff::{field_new, BigInteger, BigInteger256, FpParameters, PrimeField};
    use crypto_bigint::{ArrayEncoding, U256};
    use rfc6979::{generate_k, HmacDrbg};
    use sha2::Sha256;
    use starknet_curve::Fr;
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

    pub fn generate_k_with_shifting<F: PrimeField>() -> F {
        let ec_order = starknet_curve::FqParameters::MODULUS;

        let key = F::from(1u64);
        let msg_hash = F::from(5u64);
        generate_k_rfc6979(&ec_order, &key, &msg_hash, None)
    }

    #[test]
    fn test_generate_k_with_shifting() {
        let k: Fr = generate_k_with_shifting();

        println!("{}", k);

        // Hardcode the value from `generate_k.py`
        let expected = Fr::from_repr(BigInteger256::new([
            0x9BD45D0F07D57B17,
            0xE29FF18976642E7F,
            0x667D5ACD867D25D6,
            0x02707E03E7F40F39,
        ]))
        .unwrap();

        assert_eq!(k, expected);
    }
}
