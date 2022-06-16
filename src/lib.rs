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
    }
}