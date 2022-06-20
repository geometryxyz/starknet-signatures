#[cfg(test)]
mod tests {
    use crypto_bigint::{subtle::ConstantTimeLess, ArrayEncoding, ByteArray, Zero, U256};
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
        let msg_hash = msg_hash.to_be_byte_array();
        let empty_data = b"";
        let empty_data = empty_data.as_slice();

        let mut key = key.to_be_byte_array();
        let mut hmac_drbg = HmacDrbg::<Sha256>::new(&key, &msg_hash, empty_data);
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
