#!/bin/bash
exec solana-validator \
    --identity validator-keypair.json \
    --known-validator XkCriyrNwS3G4rzAXtG5B1nnvb5Ka1JtCku93VqeKAr \  #coinbase
    --known-validator BpyjeG4SY9r3TNnc3rP22DP3wpsStFA1FaZWywNPy6dr \ #a16z
    --known-validator Certusm1sa411sMpV9FPqU5dXAYhmmhygvxJ23S6hJ24 \ #jump
    --known-validator A4hyMd3FyvUJSRafDUSwtLLaQcxRP4r1BRC9w2AJ1to2 \ #jito
    --no-voting \
    --only-known-rpc \
    --ledger ~/ledger \
    --log ~/solana-validator.log \
    --entrypoint entrypoint.devnet.solana.com:8001 \
    --entrypoint entrypoint2.devnet.solana.com:8001 \
    --entrypoint entrypoint3.devnet.solana.com:8001 \
    --entrypoint entrypoint4.devnet.solana.com:8001 \
    --entrypoint entrypoint5.devnet.solana.com:8001 \
    --wal-recovery-mode skip_any_corrupted_record \
    --geyser-plugin-config config.json