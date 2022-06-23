use crate::constants::*;
use crate::error::Error;

use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::PrimeField;
use ark_ff::{BigInteger, Zero};
use starknet_curve::{Affine, Fr};

struct Constants {
    // bits
    pub low_part_bits: u32,

    // points
    pub hash_shift_point: Affine,
    pub p0: Affine,
    pub p1: Affine,
    pub p2: Affine,
    pub p3: Affine,
}

lazy_static! {
    static ref HASH_SHIFT_POINT: Affine = Affine::new(HASH_SHIFT_POINT_X, HASH_SHIFT_POINT_Y, false);
    static ref P0_STATIC: Affine = Affine::new(P0_STATIC_X, P0_STATIC_Y, false);
    static ref P1_STATIC: Affine = Affine::new(P1_STATIC_X, P1_STATIC_Y, false);
    static ref P2_STATIC: Affine = Affine::new(P2_STATIC_X, P2_STATIC_Y, false);
    static ref P3_STATIC: Affine = Affine::new(P3_STATIC_X, P3_STATIC_Y, false);

    static ref CONSTANTS: Constants = Constants {
        // bits
        low_part_bits: LOW_PART_BITS,
        // points
        hash_shift_point: *HASH_SHIFT_POINT,
        p0: *P0_STATIC,
        p1: *P1_STATIC,
        p2: *P2_STATIC,
        p3: *P3_STATIC,
    };
}

fn process_single_element(element: Fr, p1: Affine, p2: Affine) -> Affine {
    let mut high_nibble = element.into_repr();
    high_nibble.divn(CONSTANTS.low_part_bits);

    // element will be array of 32 bytes, each element 8 bits
    // so if we put first byte to zero we ensure that the next 248 bits are unchanged
    // and first 8 bits are zero, which is same as performing: low_part = element & low_part_mask
    let mut bytes = element.into_repr().to_bytes_be();
    bytes[0] = 0u8;
    let low_part = Fr::from_be_bytes_mod_order(&bytes);

    (p1.mul(low_part) + p2.mul(high_nibble)).into_affine()
}

/// Computes the Starkware version of the Pedersen hash of x and y.
/// The hash is defined by:
/// shift_point + x_low * P_0 + x_high * P1 + y_low * P2  + y_high * P3
/// where x_low is the 248 low bits of x, x_high is the 4 high bits of x and similarly for y.
/// shift_point, P_0, P_1, P_2, P_3 are constant points generated from the digits of pi.
fn pedersen_hash(x: &Fr, y: &Fr) -> Result<Fr, Error> {
    let pedersen_point = CONSTANTS.hash_shift_point
        + process_single_element(*x, CONSTANTS.p0, CONSTANTS.p1)
        + process_single_element(*y, CONSTANTS.p2, CONSTANTS.p3);

    let pedersen_hash = pedersen_point.x.into_repr();

    // this is negligable
    if pedersen_hash > TWO_MODULUS_BITS {
        return Err(Error::HashError);
    }

    let pedersen_hash = Fr::from_repr(pedersen_hash).unwrap();
    Ok(pedersen_hash)
}

/// Computes a hash chain over the data, in the following order:
///     h(h(h(h(0, data[0]), data[1]), ...), data[n-1]), n).
/// The hash is initialized with 0 and ends with the data length appended.
/// The length is appended in order to avoid collisions of the following kind:
/// H([x,y,z]) = h(h(x,y),z) = H([w, z]) where w = h(x,y).
fn compute_hash_on_elements(data: &Vec<Fr>) -> Result<Fr, Error> {
    if data.len() == 0 {
        return Err(Error::EmptyData);
    }

    let mut acc = Fr::zero();
    let data_len = Fr::from(data.len() as u64);
    for y in data.iter().chain(std::iter::once(&data_len)) {
        acc = pedersen_hash(&acc, y)?;
    }

    Ok(acc)
}

pub fn unsafe_hash_to_field(data: Vec<u8>) -> Result<Fr, Error> {
    if data.len() == 0 {
        return Err(Error::EmptyData);
    }

    let chunks = data.chunks(31);

    let mut elements: Vec<Fr> = chunks.map(|c| Fr::from_be_bytes_mod_order(c)).collect();
    elements.push(Fr::from(8 * data.len() as u64));
    compute_hash_on_elements(&elements)
}

#[cfg(test)]
mod tests {
    use super::{compute_hash_on_elements, pedersen_hash, unsafe_hash_to_field};
    use ark_ff::field_new;
    use starknet_curve::Fr;

    #[test]
    fn test_pedersen_with_cairo() {
        // CAIRO: pedersen_hash(17, 71) -> 1785999660572583615240258164082465668299482253941125073628479392605449162275

        let seventeen = Fr::from(17u64);
        let seventy_one = Fr::from(71u64);

        let expected = field_new!(
            Fr,
            "1785999660572583615240258164082465668299482253941125073628479392605449162275"
        );

        let pedersen_h = pedersen_hash(&seventeen, &seventy_one).unwrap();
        assert_eq!(expected, pedersen_h)
    }

    #[test]
    fn test_hash_on_long_data() {
        let data = vec![
            Fr::from(2u64),
            Fr::from(4u64),
            Fr::from(8u64),
            Fr::from(16u64),
            Fr::from(32u64),
        ];

        let expected = field_new!(
            Fr,
            "2811736568068244484902543134224269103996353337662770485859146392457932405098"
        );

        let pedersen_h = compute_hash_on_elements(&data).unwrap();
        assert_eq!(expected, pedersen_h)
    }

    #[test]
    fn test_hash_to_field() {
        let message = b"Hello Marcello! This is a long message from the Rust people. We wrote this unsafe hash to field and would like you to try implementing the same function in Cairo. If we get the same hash, we can then move on to publishing our demo :)";
        
        let hashed = unsafe_hash_to_field(message.to_vec()).unwrap();

        println!("{}", hashed);
        // 04828D901704C8D1B6A82F1C256BE2B95C55A8FAA4309CAAF37A3378434AFF1C
    }
}
