import pytest
import asyncio
from typing import NamedTuple
from starkware.crypto.signature.fast_pedersen_hash import pedersen_hash, pedersen_hash_func
from starkware.cairo.common.hash_state import compute_hash_on_elements

from starkware.starknet.testing.contract import StarknetContract
from starkware.starknet.testing.starknet import Starknet
from nile.signer import Signer


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
    msg = [2, 4, 8, 16, 32]
    msg_hash = compute_hash_on_elements(msg)

    (sig_r, sig_s) = signer.sign(msg_hash)

    await verifier.verify_sig(msg, signer.public_key, (sig_r, sig_s)).call()

@pytest.mark.asyncio
async def test_pedesen():
    input = [2, 4, 8, 16, 32]
    output = compute_hash_on_elements(input)

