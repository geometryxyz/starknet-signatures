use crate::rfc6979::generate_k_rfc6979;

use ark_ec::ProjectiveCurve;
use ark_ff::{field_new, Field, FpParameters, One, PrimeField, Zero};
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

pub fn sign(
    parameters: &SigningParameters,
    priv_key: Fr,
    msg_hash: Fr,
    seed: Option<u64>,
) -> Option<Signature> {
    // Fr::MODULUS_BITS = 251, 2**251 =
    let two_pow_modulus_bits = field_new!(
        Fr,
        "3618502788666131106986593281521497120414687020801267626233049500247285301248"
    );

    // Note: msg_hash must be smaller than 2**N_ELEMENT_BITS_ECDSA.
    // Message whose hash is >= 2**N_ELEMENT_BITS_ECDSA cannot be signed.
    // This happens with a very small probability.
    // https://github.com/starkware-libs/cairo-lang/blob/167b28bcd940fd25ea3816204fa882a0b0a49603/src/starkware/crypto/starkware/crypto/signature/signature.py#L136
    if !(Fr::zero() <= msg_hash && msg_hash < two_pow_modulus_bits) {
        return None;
    }

    let mut seed = seed;
    loop {
        // replace with rfc6979 as in https://github.com/starkware-libs/cairo-lang/blob/167b28bcd940fd25ea3816204fa882a0b0a49603/src/starkware/crypto/starkware/crypto/signature/signature.py#L145
        let k = generate_k_rfc6979(
            &starknet_curve::FqParameters::MODULUS,
            &priv_key,
            &msg_hash,
            seed,
        );

        seed = match seed {
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
    use crate::pedersen::compute_hash_on_elements;
    use ark_ec::{AffineCurve, ProjectiveCurve};
    use ark_ff::{Field, PrimeField};
    use ark_std::UniformRand;
    use rand::thread_rng;
    use starknet::{
        core::{
            types::{BlockId, FieldElement, InvokeFunctionTransactionRequest},
            utils::get_selector_from_name,
        },
        providers::{Provider, SequencerGatewayProvider},
    };
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

        let msg = vec![
            Fr::from(1u64),
            Fr::from(2u64),
            Fr::from(3u64),
            Fr::from(4u64),
            Fr::from(5u64),
        ];
        let msg_hash = compute_hash_on_elements(&msg).unwrap();

        let sig = sign(&parameters, private_key, msg_hash, None).expect("Message is out of bound");

        assert_eq!(
            true,
            verify_signature(&parameters, &public_key, &msg_hash, &sig)
        );

        let sig_verification_contract_address = FieldElement::from_hex_be(
            "04b7e9f16515962136d9836af263840146214e8df1a6d841fed055b00d9d8df6",
        )
        .unwrap();

        let public_key_x = FieldElement::from_hex_be(public_key.x.0.to_string().as_str()).unwrap();
        let sig_r_fe = FieldElement::from_hex_be(sig.r.0.to_string().as_str()).unwrap();
        let sig_s_fe = FieldElement::from_hex_be(sig.s.0.to_string().as_str()).unwrap();

        let calldata = vec![
            //len of msg
            FieldElement::from_mont([msg.len() as u64, 0, 0, 0]),
            //msg elements
            FieldElement::from_hex_be(msg[0].0.to_string().as_str()).unwrap(),
            FieldElement::from_hex_be(msg[1].0.to_string().as_str()).unwrap(),
            FieldElement::from_hex_be(msg[2].0.to_string().as_str()).unwrap(),
            FieldElement::from_hex_be(msg[3].0.to_string().as_str()).unwrap(),
            FieldElement::from_hex_be(msg[4].0.to_string().as_str()).unwrap(),
            // public key x
            public_key_x,
            //r
            sig_r_fe,
            //s
            sig_s_fe,
        ];

        let provider = SequencerGatewayProvider::starknet_alpha_goerli();
        let _ = provider.call_contract(
            InvokeFunctionTransactionRequest {
                contract_address: sig_verification_contract_address,
                calldata,
                entry_point_selector: get_selector_from_name("verify_sig").unwrap(),
                max_fee: FieldElement::from_mont([0, 0, 0, 0]),
                signature: vec![],
            },
            BlockId::Latest,
        );
    }
}
