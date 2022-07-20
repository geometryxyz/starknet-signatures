%lang starknet

from starkware.cairo.common.cairo_builtins import (
    HashBuiltin,
    SignatureBuiltin,
)
from starkware.cairo.common.hash import hash2
from starkware.cairo.common.signature import (
    verify_ecdsa_signature,
)
from starkware.cairo.common.uint256 import Uint256
from starkware.cairo.common.hash_state import hash_felts


@view
func verify_sig{
    syscall_ptr : felt*,
    pedersen_ptr : HashBuiltin*,
    range_check_ptr,
    ecdsa_ptr : SignatureBuiltin*,
}(msg_len: felt, msg: felt*, signer_pubkey: felt, sig : (felt, felt)):
    let (msg_hash) = hash_felts{hash_ptr=pedersen_ptr}(msg, msg_len)
    verify_ecdsa_signature(
        message=msg_hash,
        public_key=signer_pubkey,
        signature_r=sig[0],
        signature_s=sig[1],
    )
    return ()
end

@external
func verify_signature{
    syscall_ptr : felt*,
    pedersen_ptr : HashBuiltin*,
    range_check_ptr,
    ecdsa_ptr : SignatureBuiltin*,
}(msg_len: felt, msg: felt*, signer_pubkey: felt, sig : (felt, felt)):
    let (msg_hash) = hash_felts{hash_ptr=pedersen_ptr}(msg, msg_len)
    verify_ecdsa_signature(
        message=msg_hash,
        public_key=signer_pubkey,
        signature_r=sig[0],
        signature_s=sig[1],
    )
    return ()
end

