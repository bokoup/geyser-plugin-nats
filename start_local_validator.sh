#!/bin/bash
# Change mint to address of local test solana wallet address
# Change bpf-program to be id from anchor program

BPF_PROGRAM=CjSoZrc2DBZTv1UdoMx8fTcCpqEMXCyfm2EuTwy8yiGi
WALLET=61mVTaw6hBtwWnSaGXRSJePFWEQqipeCka3evytEVNUp

# build plugin
cargo build

# build bpl-token-metadata
# cd ../bokoup-program-library
# anchor build
# cd ../geyser-plugin-nats

# start nats server
if [ ! "$(docker ps -q -f name=nats-server)" ]; then
    if [ "$(docker ps -aq -f status=exited -f name=nats-server)" ]; then
        # cleanup
        docker rm nats-server
    fi
    # run your container
    docker run -d --name nats-server --rm -p 4222:4222 -p 8222:8222 nats
fi

# start validator
solana-test-validator \
--reset \
--mint $WALLET \
--bind-address 0.0.0.0 \
--bpf-program \
    $BPF_PROGRAM \
    ../bokoup-program-library/target/deploy/bpl_token_metadata.so \
--clone metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s \
--clone PwDiXFxQsGra4sFFTT8r1QWRMd4vfumiWC1jfWNfdYT \
--clone hausS13jsjafwWwGqZTUQRmWyvyxn9EQpqMwV1PBBmk \
--clone AfsUqnMuZ54ieBDvdHGyJ9Apm29UE4zvfGPjkQf65ztc \
--rpc-port 8899 \
--url https://api.devnet.solana.com \
--geyser-plugin-config config.json