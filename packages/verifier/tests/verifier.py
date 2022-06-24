import pytest
import asyncio
from typing import NamedTuple, List
from starkware.crypto.signature.fast_pedersen_hash import pedersen_hash, pedersen_hash_func
from starkware.cairo.common.hash_state import compute_hash_on_elements

from starkware.starknet.testing.contract import StarknetContract
from starkware.starknet.testing.starknet import Starknet
from starkware.python.utils import from_bytes
from nile.signer import Signer


def format_input(input: bytes) -> List[int]:
    (q, r) = divmod(len(input), 31)
    output = []

    for i in range(0, q * 31, 31):
        output.append(from_bytes(input[i:i+31]))
    remaining_bytes = input[q*31: q*31+r]
    output.append(from_bytes(remaining_bytes))
    output.append(8*len(input))

    return output


class TestsDeps(NamedTuple):
    starknet: Starknet
    verifier: StarknetContract

@pytest.fixture(scope='module')
def event_loop():
    return asyncio.new_event_loop()

async def setup():
    starknet = await Starknet.empty()
    verifier = await starknet.deploy("packages/verifier/src/Verifier.cairo", cairo_path=["packages/verifier/src"])
    return TestsDeps(starknet=starknet, verifier=verifier)

@pytest.fixture(scope='module')
async def factory():
    return await setup()


@pytest.mark.asyncio
async def test_verifier(factory):
    starknet, verifier = factory

    signer = Signer(0xbeef)

    msg = b"Hello Marcello! This is a long message from the Rust people. We wrote this unsafe hash to field and would like you to try implementing the same function in Cairo. If we get the same hash, we can then move on to publishing our demo :)"
    msg_to_words = format_input(msg)
    msg_hash = compute_hash_on_elements(msg_to_words)

    (sig_r, sig_s) = signer.sign(msg_hash)

    await verifier.verify_sig(msg_to_words, signer.public_key, (sig_r, sig_s)).call()

@pytest.mark.asyncio
async def test_pedesen():
    input = [2, 4, 8, 16, 32]
    output = compute_hash_on_elements(input)

