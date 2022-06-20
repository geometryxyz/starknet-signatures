#!/bin/bash
mkdir artifacts;
mkdir  artifacts/abis;
./.tox/py37/bin/starknet-compile packages/verifier/src/Verifier.cairo --cairo_path=packages/verifier/src --output=artifacts/Verifier.json --abi=artifacts/abis/Verifier.json;
./.tox/py37/bin/starknet deploy --contract=artifacts/Verifier.json --network=alpha-goerli;