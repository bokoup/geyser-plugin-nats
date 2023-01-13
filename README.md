# geyser-plugin-nats

## localnet validator setup

0. install rust tool chain

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

1. install solana cli

```
sh -c "$(curl -sSfL https://release.solana.com/v1.13.3/install)"`
```

See [Install the Solana Tool Suite](https://docs.solana.com/cli/install-solana-cli-tools) for additional instructions.

2. create local keypair

```
solana-keygen new -o "~/.config/solana/id.json"`
```

3. copy environment variable file to `bpl-program-library/`

```
cp .env bpl-program-library/
```

4. copy program keypair to `target/deploy`

```
cp bpl_token_metadata-keypair.json target/deploy/
```

5. build token-metadata program

```
git clone git@github.com:bokoup/bokoup-program-library.git
cd ~/bokup-program-library
anchor build
```

6. build geyser-plugin-nats program

```
cd ~
git clone git@github.com:bokoup/geyser-plugin-nats.git
cd ~/geyser-plugin-nats
cargo build
```

7. start the nats messaging server and local test validator

```
bash start_local_validator.sh
```

8. in a second terminal, build the indexer and start it

```
cd ~/bokoup-program-library/indexer
cargo build
cargo run
```

8. in a third terminal, build the transaction api and start it

```
cd ~/bokoup-program-library/api-tx
cargo build
cargo run
```

9. reset the localnet database

```
cd ~/bokoup-program-library/api-data
cargo run -- reset-schema
```

10. run program tests to populate database with test data

```
cd ~/bokoup-program-library
anchor test --skip-deploy --skip-local-validator
```

_checks_

- All program tests pass
- Logs in indexer worker terminal window show logs of accounts and transactions being inserted into the database
- Data can be queried at the [public graphql endpoint](https://cloud.hasura.io/public/graphiql?endpoint=https%3A%2F%2Fshining-sailfish-15.hasura.app%2Fv1%2Fgraphql%2F)

11. endpoints in merchant and customer apps can be updated to:

- data api: https://shining-sailfish-15.hasura.app/v1/graphql/
- transaction api: http://127.0.0.1:8080
- validator rpc: http://127.0.0.1:8899

## additional resources

- https://github.com/shiraz-edgevana/solana
- https://docs.solana.com/running-validator/validator-start
- https://github.com/agjell/sol-tutorials/blob/master/setting-up-a-solana-devnet-validator.md
- https://medium.com/coinmonks/running-a-solana-validator-on-aws-bb86162eaf29
- https://laine-sa.medium.com/running-a-solana-validator-lessons-tips-6e6d08c0c589
- https://chainstack.com/how-to-run-a-solana-node/
