use crate::Error;
use ark_ff::{BigInteger256, FpParameters, FromBytes, PrimeField};

pub fn bytes_safe<F: PrimeField<BigInt = BigInteger256>>(
    unchecked_bytes: &Vec<u8>,
) -> Result<BigInteger256, Error> {
    // FromBytes fails if len != 32, anyway we explicitly check for clear err handling
    if unchecked_bytes.len() != 32 {
        return Err(Error::IncorrectLenError);
    }

    let repr = BigInteger256::read(unchecked_bytes.as_slice()).map_err(|_| Error::IOError)?;

    if repr > F::Params::MODULUS {
        return Err(Error::OverflowError);
    }

    Ok(repr)
}

pub fn try_bytes_to_field<F: PrimeField<BigInt = BigInteger256>>(
    unchecked_bytes: &Vec<u8>,
) -> Result<F, Error> {
    let repr = bytes_safe::<F>(unchecked_bytes)?;

    // it's safe to unwrap
    Ok(F::from_repr(repr).unwrap())
}
