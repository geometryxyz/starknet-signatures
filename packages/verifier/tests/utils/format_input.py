import pytest
from typing import List
from starkware.cairo.common.hash_state import compute_hash_on_elements
from starkware.python.utils import from_bytes


def format_input(input: bytes) -> List[int]:
    (q, r) = divmod(len(input), 31)
    output = []

    for i in range(0, q * 31, 31):
        output.append(from_bytes(input[i:i+31]))
    remaining_bytes = input[q*31: q*31+r]
    output.append(from_bytes(remaining_bytes))
    output.append(8*len(input))

    return output


@pytest.mark.asyncio
async def test_format_input():
    input = b"Hello Marcello! This is a long message from the Rust people. We wrote this unsafe hash to field and would like you to try implementing the same function in Cairo. If we get the same hash, we can then move on to publishing our demo :)"

    output = format_input(input)
    output_hash = compute_hash_on_elements(output)

