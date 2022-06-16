use ark_ec::ProjectiveCurve;
use ark_ff::{BigInteger256, Field, One, PrimeField, Zero};
use ark_std::UniformRand;
use rand::thread_rng;
use starknet_curve::{Fr, Projective};

pub struct SigningParameters {
    pub generator: Projective,
}

pub struct Signature {
    pub r: Fr,
    pub s: Fr,
}

pub fn parameters() -> SigningParameters {
    SigningParameters {
        generator: Projective::prime_subgroup_generator(),
    }
}

pub fn sign(parameters: &SigningParameters, priv_key: Fr, msg_hash: Fr) -> Option<Signature> {
    // Fr::MODULUS_BITS = 251, 2**251 = 3618502788666131106986593281521497120414687020801267626233049500247285301248
    //                                = 0x0800000000000000000000000000000000000000000000000000000000000000
    let two_pow_modulus = Fr::from(BigInteger256::new([
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0800000000000000,
    ]));

    // # Note: msg_hash must be smaller than 2**N_ELEMENT_BITS_ECDSA.
    // # Message whose hash is >= 2**N_ELEMENT_BITS_ECDSA cannot be signed.
    // # This happens with a very small probability.
    if !(Fr::zero() <= msg_hash && msg_hash < two_pow_modulus) {
        return None;
    }

    loop {
        // replace with rfc6979 as in https://github.com/starkware-libs/cairo-lang/blob/167b28bcd940fd25ea3816204fa882a0b0a49603/src/starkware/crypto/starkware/crypto/signature/signature.py#L145
        let k = Fr::rand(&mut thread_rng());
        let r = Fr::from_repr(parameters.generator.mul(k.into_repr()).x.into_repr()).unwrap();

        if !(r >= Fr::one() && r < two_pow_modulus) {
            // Bad value. This fails with negligible probability.
            continue;
        }

        let temp = msg_hash + r * priv_key;

        // this check in starkware: "temp.into_repr() % starknet_curve::FqParameters::MODULUS"
        // but since arkworks fr already does operations by modulus we just check that temp != 0
        if temp == Fr::zero() {
            // Bad value. This fails with negligible probability.
            continue;
        }

        // temp is not a zero so it's safe to unwrap
        let w = k * temp.inverse().unwrap();

        if !(w >= Fr::one() && w < two_pow_modulus) {
            // Bad value. This fails with negligible probability.
            continue;
        }

        let s = w.inverse().unwrap();

        break Some(Signature { r, s });
    }
}

pub fn private_key_to_public_key(parameters: &SigningParameters, priv_key: Fr) -> Projective {
    parameters.generator.mul(priv_key.into_repr())
}

#[cfg(test)]
mod tests {

    use super::{parameters, private_key_to_public_key, sign};
    use ark_ec::ProjectiveCurve;
    use ark_ff::{BigInteger, BigInteger256, FpParameters, One};
    use ark_std::UniformRand;
    use rand::thread_rng;
    use starknet_curve::Fr;

    #[test]
    fn random_signature() {
        let rng = &mut thread_rng();
        let parameters = parameters();

        let private_key = Fr::rand(rng);
        let public_key = private_key_to_public_key(&parameters, private_key).into_affine();
        let msg_hash = Fr::rand(rng);

        let sig = sign(&parameters, private_key, msg_hash).expect("Message is out of bound");
        println!("msg_hash: {}", msg_hash);
        println!("public key x: {}", public_key.x);
        println!("public key y: {}", public_key.y);
        println!("sig r: {}", sig.r);
        println!("sig s: {}", sig.s);
    }

    #[test]
    fn check_modulus_overflow() {
        let mut modulus_minus_five = starknet_curve::FrParameters::MODULUS;
        let five = BigInteger256::new([5u64, 0, 0, 0]);
        let _ = modulus_minus_five.sub_noborrow(&five);

        let big_number = Fr::from(modulus_minus_five);
        let bigger_number = big_number + Fr::from(6u64);
        assert_eq!(bigger_number, Fr::one());
    }

    #[test]
    fn instantiate_2_pow_modulus() {
        let two_pow_modulus = BigInteger256::new([
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0800000000000000,
        ]);
        let _ = Fr::from(two_pow_modulus);
    }
}
