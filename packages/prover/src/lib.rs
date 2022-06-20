use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::{PrimeField, Field};
use ark_std::UniformRand;
use rand::thread_rng;
use starknet_curve::{Fr, Fq, G_GENERATOR_X, G_GENERATOR_Y, Projective};

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

pub fn sign(parameters: &SigningParameters, priv_key: Fr, msg_hash: Fr) -> Signature {
    // replace with rfc6979 as in https://github.com/starkware-libs/cairo-lang/blob/167b28bcd940fd25ea3816204fa882a0b0a49603/src/starkware/crypto/starkware/crypto/signature/signature.py#L145
    let k = Fr::rand(&mut thread_rng());
    let r = Fr::from_repr(parameters.generator.mul(k.into_repr()).x.into_repr()).unwrap();
    // add assertions as in https://github.com/starkware-libs/cairo-lang/blob/167b28bcd940fd25ea3816204fa882a0b0a49603/src/starkware/crypto/starkware/crypto/signature/signature.py#L157
    let w = msg_hash + r*priv_key;
    let s = w.inverse().unwrap();

    Signature {
        r,
        s,
    }
}

pub fn private_key_to_public_key(parameters: &SigningParameters, priv_key: Fr) -> Projective {
    parameters.generator.mul(priv_key.into_repr())
}

#[cfg(test)]
mod tests {

    use rand::thread_rng;
    use super::{sign, parameters, private_key_to_public_key};
    use starknet_curve::Fr;
    use ark_std::UniformRand;
    use ark_ec::ProjectiveCurve;
    use ark_ff::Fp256;

    use starknet::{
        core::{types::{InvokeFunctionTransactionRequest, BlockId}},
        providers::{SequencerGatewayProvider, Provider},
        core::{types::FieldElement, utils::get_selector_from_name},
    };

    #[test]
    fn random_signature() {
        let rng = &mut thread_rng();
        let parameters = parameters();

        let private_key = Fr::rand(rng);
        let public_key = private_key_to_public_key(&parameters, private_key).into_affine();
        let msg_hash = Fr::rand(rng);

        let sig = sign(&parameters, private_key, msg_hash);
        println!("msg_hash: {}", msg_hash);
        println!("public key x: {}", public_key.x);
        println!("public key y: {}", public_key.y);
        println!("sig r: {}", sig.r);
        println!("sig s: {}", sig.s);

        let sig_verification_contract_address = FieldElement::from_hex_be(
            "07b1f0242f3a45fa81a9192d503b8a203fc3e8579c4a43517cfdc551a618b663",
        )
        .unwrap();

        let msg_hash_fe = FieldElement::from_dec_str(msg_hash.to_string().as_str()).unwrap();
        let public_key_fe = FieldElement::from_dec_str(public_key.to_string().as_str()).unwrap();
        let sig_r_fe = FieldElement::from_dec_str(sig.r.to_string().as_str()).unwrap();
        let sig_s_fe = FieldElement::from_dec_str(sig.s.to_string().as_str()).unwrap();

        let provider = SequencerGatewayProvider::starknet_alpha_goerli();
        let call = provider.call_contract(InvokeFunctionTransactionRequest {
                contract_address: sig_verification_contract_address,
                calldata: vec![msg_hash_fe, public_key_fe, sig_r_fe, sig_s_fe],
                entry_point_selector: get_selector_from_name("verify_sig").unwrap(),
                max_fee: FieldElement::from_dec_str("0").unwrap(),
                signature: vec![]}
            ,BlockId::Latest);
    }
}