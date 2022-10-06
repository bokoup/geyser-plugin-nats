#!/bin/bash
# #coinbas
# #a16z
# jump
# #jito
#--known-validator XkCriyrNwS3G4rzAXtG5B1nnvb5Ka1JtCku93VqeKAr \
#--known-validator BpyjeG4SY9r3TNnc3rP22DP3wpsStFA1FaZWywNPy6dr \
#--known-validator Certusm1sa411sMpV9FPqU5dXAYhmmhygvxJ23S6hJ24 \
#--known-validator A4hyMd3FyvUJSRafDUSwtLLaQcxRP4r1BRC9w2AJ1to2 \
#--only-known-rpc \

exec solana-validator \
    --identity /home/caleb/validator-keypair.json \
    --no-voting \
    --full-rpc-api \
    --private-rpc \
    --rpc-port 8899 \
    --ledger /home/caleb/ledger \
    --log /home/caleb/solana-validator.log \
    --entrypoint entrypoint.devnet.solana.com:8001 \
    --entrypoint entrypoint2.devnet.solana.com:8001 \
    --entrypoint entrypoint3.devnet.solana.com:8001 \
    --entrypoint entrypoint4.devnet.solana.com:8001 \
    --entrypoint entrypoint5.devnet.solana.com:8001 \
    --wal-recovery-mode skip_any_corrupted_record \
    --geyser-plugin-config /home/caleb/geyser-plugin-nats/config.json