# this file should provide playground for generating safe randomness k used in ecdsa algorithm
# this exact method is used in starknet, so we use this as a helper to test client(rust) side
# with small modification of adding should_shift argument which helps us in debugging

import hmac
import binascii

import hashlib
import math

def orderlen(order):
    return (1 + len("%x" % order)) // 2  # bytes

def number_to_string(num, order):
    l = orderlen(order)
    fmt_str = "%0" + str(2 * l) + "x"
    string = binascii.unhexlify((fmt_str % num).encode())
    assert len(string) == l, (len(string), l)
    return string


def number_to_string_crop(num, order):
    l = orderlen(order)
    fmt_str = "%0" + str(2 * l) + "x"
    string = binascii.unhexlify((fmt_str % num).encode())
    return string[:l]

def bit_length(x):
    return x.bit_length() or 1

def hmac_compat(data):
    return data


def bits2int(data, qlen, should_shift = True):
    x = int(binascii.hexlify(data), 16)
    l = len(data) * 8

    if should_shift and l > qlen:
        return x >> (l - qlen)
    return x


def bits2octets(data, order):
    z1 = bits2int(data, bit_length(order))
    z2 = z1 - order

    if z2 < 0:
        z2 = z1

    return number_to_string_crop(z2, order)


# https://tools.ietf.org/html/rfc6979#section-3.2
def generate_k(order, secexp, hash_func, data, retry_gen=0, extra_entropy=b"", should_shift = True):
    """
    Generate the ``k`` value - the nonce for DSA.
    :param int order: order of the DSA generator used in the signature
    :param int secexp: secure exponent (private key) in numeric form
    :param hash_func: reference to the same hash function used for generating
        hash, like :py:class:`hashlib.sha1`
    :param bytes data: hash in binary form of the signing data
    :param int retry_gen: how many good 'k' values to skip before returning
    :param bytes extra_entropy: additional added data in binary form as per
        section-3.6 of rfc6979
    :rtype: int
    """

    qlen = bit_length(order)
    holen = hash_func().digest_size
    rolen = (qlen + 7) // 8
    bx = (
        hmac_compat(number_to_string(secexp, order)),
        hmac_compat(bits2octets(data, order)),
        hmac_compat(extra_entropy),
    )

    # Step B
    v = b"\x01" * holen

    # Step C
    k = b"\x00" * holen

    # Step D

    k = hmac.new(k, digestmod=hash_func)
    k.update(v + b"\x00")
    for i in bx:
        k.update(i)
    k = k.digest()

    # Step E
    v = hmac.new(k, v, hash_func).digest()

    # Step F
    k = hmac.new(k, digestmod=hash_func)
    k.update(v + b"\x01")
    for i in bx:
        k.update(i)
    k = k.digest()

    # Step G
    v = hmac.new(k, v, hash_func).digest()

    # Step H
    while True:
        # Step H1
        t = b""

        # Step H2
        while len(t) < rolen:
            v = hmac.new(k, v, hash_func).digest()
            t += v

        # Step H3
        secret = bits2int(t, qlen, should_shift)

        if 1 <= secret < order:
            if retry_gen <= 0:
                return secret
            retry_gen -= 1

        k = hmac.new(k, v + b"\x00", hash_func).digest()
        v = hmac.new(k, v, hash_func).digest()

key = 1
msg_hash = 5
EC_ORDER = 3618502788666131213697322783095070105526743751716087489154079457884512865583


k_with_shifting = generate_k(
    EC_ORDER,
    key,
    hashlib.sha256,
    msg_hash.to_bytes(math.ceil(msg_hash.bit_length() / 8), "big"),
)

print(k_with_shifting)
# k = 1103382293768787051960919618868228286379034699699304773921950668690828917527
# k_hex = 02707E03E7F40F39667D5ACD867D25D6E29FF18976642E7F9BD45D0F07D57B17

k_without_shifting = generate_k(
    EC_ORDER,
    key,
    hashlib.sha256,
    msg_hash.to_bytes(math.ceil(msg_hash.bit_length() / 8), "big"),
    should_shift=False
)

print(k_without_shifting)
# k = 730205928169535723067059682807017292793780113497803892355151814545642599552
# k_hex = 019D482B334A0B9F7E335A96AF94AB94DAE0F18D40E7DBC8A47D4427E0EFB480