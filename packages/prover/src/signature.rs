use crate::{constants::TWO_MODULUS_BITS, error::Error, rfc6979::generate_k_rfc6979};

use ark_ec::ProjectiveCurve;
use ark_ff::{Field, FpParameters, One, PrimeField, Zero};
use starknet_curve::{Fq, Fr, Projective};

/*
starknet curve
Fq = 3618502788666131213697322783095070105623107215331596699973092056135872020481
Fr = 3618502788666131213697322783095070105526743751716087489154079457884512865583

cairo
FIELD_PRIME: 3618502788666131213697322783095070105623107215331596699973092056135872020481
EC_ORDER: 3618502788666131213697322783095070105526743751716087489154079457884512865583

Fq = FIELD_PRIME and Fr = EC_ORDER

Note: Fq is bigger than Fr => Fq can always fit in Fr
*/

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
    msg_hash: Fq,
    seed: Option<u64>,
) -> Result<Signature, Error> {
    // Note: msg_hash must be smaller than 2**N_ELEMENT_BITS_ECDSA.
    // Message whose hash is >= 2**N_ELEMENT_BITS_ECDSA cannot be signed.
    // This happens with a very small probability.
    // https://github.com/starkware-libs/cairo-lang/blob/167b28bcd940fd25ea3816204fa882a0b0a49603/src/starkware/crypto/starkware/crypto/signature/signature.py#L136
    // This also means that msg_hash can be safely converted to Fr
    if !(Fq::zero() <= msg_hash && msg_hash.into_repr() < TWO_MODULUS_BITS) {
        return Err(Error::EmptyDataError);
    }

    // since we check that msg hash is smaller then Fr it's safe to unwrap
    let msg_hash_as_r = Fr::from_repr(msg_hash.into_repr()).unwrap();
    let mut seed = seed;
    loop {
        // replace with rfc6979 as in https://github.com/starkware-libs/cairo-lang/blob/167b28bcd940fd25ea3816204fa882a0b0a49603/src/starkware/crypto/starkware/crypto/signature/signature.py#L145
        let k = generate_k_rfc6979(
            &starknet_curve::FrParameters::MODULUS,
            &priv_key,
            &msg_hash_as_r,
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

        // r is x coordinate of EcPoint so it's in Fq
        // here we check that it's safe to convert it into Fr
        if !(unchecked_r >= Fr::one().into_repr() && unchecked_r < TWO_MODULUS_BITS) {
            // Bad value. This fails with negligible probability.
            continue;
        }

        // since we checked that it's < TOW_MODULUS_BITS it will be safe to convert to Fr
        let r = Fr::from_repr(unchecked_r).unwrap();

        let temp = msg_hash_as_r + r * priv_key;
        // this check in starkware: "temp.into_repr() % Fr::MODULUS"
        // but since arkworks fr already does operations by modulus we just check that temp != 0
        if temp == Fr::zero() {
            // Bad value. This fails with negligible probability.
            continue;
        }

        // temp is not a zero so it's safe to unwrap
        let w = k * temp.inverse().unwrap();
        if !(w >= Fr::one() && w.into_repr() < TWO_MODULUS_BITS) {
            // Bad value. This fails with negligible probability.
            continue;
        }

        let s = w.inverse().unwrap();
        break Ok(Signature { r, s });
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
    use ark_ff::{Field, PrimeField, field_new};
    use ark_std::UniformRand;
    use rand::thread_rng;
    use starknet::{
        core::{
            types::{BlockId, FieldElement, InvokeFunctionTransactionRequest},
            utils::get_selector_from_name,
        },
        providers::{Provider, SequencerGatewayProvider},
    };
    use starknet_curve::{Affine, Fq, Fr};

    // here we skip range and on_curve checks since this is used just for local testing
    pub fn verify_signature(
        parameters: &SigningParameters,
        pub_key: &Affine,
        msg_hash: &Fq,
        signature: &Signature,
    ) -> bool {
        let gen = parameters.generator;

        let r = signature.r;
        let s = signature.s;
        // # Compute w = s^-1 (mod EC_ORDER).
        let w = s.inverse().unwrap();

        // since we check that msg hash is in bound, it's safe to unwrap
        let msg_hash = Fr::from_repr(msg_hash.into_repr()).unwrap();

        let point = gen.mul((msg_hash * w).into_repr()).into_affine()
            + pub_key.mul((r * w).into_repr()).into_affine();

        let x = Fr::from_repr(point.x.into_repr()).unwrap();

        x == r
    }

    #[test]
    fn random_signature() {
        let rng = &mut thread_rng();
        let parameters = parameters();

        let private_key = Fr::rand(rng);
        let private_key = Fr::from(10 as u64);
        // let private_key = field_new!(Fr, "172882690830337988349958037368858324155537522255755900471757931013892417958");
        let public_key = private_key_to_public_key(&parameters, private_key).into_affine();

        println!("pk x: {}", public_key.x);
        println!("pk y: {}", public_key.y);


        let msg = vec![
            Fq::from(10u64),
        ];

        let msg_hash = compute_hash_on_elements(&msg).unwrap();
        let sig = sign(&parameters, private_key, msg_hash, None).unwrap();

        assert_eq!(
            true,
            verify_signature(&parameters, &public_key, &msg_hash, &sig)
        );

        println!("sig r: {}", sig.r); //604545849778525062457762543482147741588851271534277738208145578562194622620
        println!("sig s: {}", sig.s); //1005023245858433631958764350290981577571376768099078396745908094681410828614

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
            // FieldElement::from_hex_be(msg[3].0.to_string().as_str()).unwrap(),
            // FieldElement::from_hex_be(msg[4].0.to_string().as_str()).unwrap(),
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
