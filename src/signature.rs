use crate::rfc6979::generate_k_rfc6979;

use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::{BigInteger, BigInteger256, Field, FpParameters, One, PrimeField, Zero};
use ark_std::UniformRand;
use rand::thread_rng;
use starknet_curve::{Affine, Fr, Projective};

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

pub fn sign(
    parameters: &SigningParameters,
    priv_key: Fr,
    msg_hash: Fr,
    seed: Option<u64>,
) -> Option<Signature> {
    // Fr::MODULUS_BITS = 251, 2**251 = 3618502788666131106986593281521497120414687020801267626233049500247285301248
    //                                = 0x0800000000000000000000000000000000000000000000000000000000000000
    let two_pow_modulus_bits = Fr::from(BigInteger256::new([
        0x0000000000000000,
        0x0000000000000000,
        0x0000000000000000,
        0x0800000000000000,
    ]));

    // Note: msg_hash must be smaller than 2**N_ELEMENT_BITS_ECDSA.
    // Message whose hash is >= 2**N_ELEMENT_BITS_ECDSA cannot be signed.
    // This happens with a very small probability.
    if !(Fr::zero() <= msg_hash && msg_hash < two_pow_modulus_bits) {
        return None;
    }

    loop {
        // replace with rfc6979 as in https://github.com/starkware-libs/cairo-lang/blob/167b28bcd940fd25ea3816204fa882a0b0a49603/src/starkware/crypto/starkware/crypto/signature/signature.py#L145
        let k = generate_k_rfc6979(
            &starknet_curve::FqParameters::MODULUS,
            &priv_key,
            &msg_hash,
            seed,
        );

        let seed = match seed {
            Some(seed) => Some(seed + 1),
            None => Some(1),
        };

        let unchecked_r = parameters
            .generator
            .mul(k.into_repr())
            .into_affine()
            .x
            .into_repr();

        if !(unchecked_r >= Fr::one().into_repr() && unchecked_r < two_pow_modulus_bits.into_repr())
        {
            // Bad value. This fails with negligible probability.
            continue;
        }

        // since we checked r, it's safe to unwrap
        let r = Fr::from_repr(unchecked_r).unwrap();

        let temp = msg_hash + r * priv_key;

        // this check in starkware: "temp.into_repr() % starknet_curve::FqParameters::MODULUS"
        // but since arkworks fr already does operations by modulus we just check that temp != 0
        if temp == Fr::zero() {
            // Bad value. This fails with negligible probability.
            continue;
        }

        // temp is not a zero so it's safe to unwrap
        let w = k * temp.inverse().unwrap();

        if !(w >= Fr::one() && w < two_pow_modulus_bits) {
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

    use super::{parameters, private_key_to_public_key, sign, Signature, SigningParameters};
    use ark_ec::{AffineCurve, ProjectiveCurve};
    use ark_ff::{Field, PrimeField};
    use ark_std::UniformRand;
    use rand::thread_rng;
    use starknet_curve::{Affine, Fr};

    pub fn verify_signature(
        parameters: &SigningParameters,
        pub_key: &Affine,
        msg_hash: &Fr,
        signature: &Signature,
    ) -> bool {
        let gen = parameters.generator;

        let r = signature.r;
        let s = signature.s;
        // # Compute w = s^-1 (mod EC_ORDER).
        let w = s.inverse().unwrap();

        let point = gen.mul((*msg_hash * w).into_repr()).into_affine()
            + pub_key.mul((r * w).into_repr()).into_affine();

        let x = Fr::from_repr(point.x.into_repr()).unwrap();

        x == r
    }

    #[test]
    fn random_signature() {
        let rng = &mut thread_rng();
        let parameters = parameters();

        let private_key = Fr::rand(rng);
        let public_key = private_key_to_public_key(&parameters, private_key).into_affine();
        let msg_hash = Fr::rand(rng);

        let sig = sign(&parameters, private_key, msg_hash, None).expect("Message is out of bound");
        println!("msg_hash: {}", msg_hash);
        println!("public key x: {}", public_key.x);
        println!("public key y: {}", public_key.y);
        println!("sig r: {}", sig.r);
        println!("sig s: {}", sig.s);

        assert_eq!(
            true,
            verify_signature(&parameters, &public_key, &msg_hash, &sig)
        )
    }
}
