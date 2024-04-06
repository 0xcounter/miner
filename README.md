# Modified Ore CLI

A custom command line interface for the Ore program.

## Prerequisites

1. Make sure you install rust first. Just google how for your OS.
2. Also you will need to generate some keypairs. This would require you to install solana-cli.

For convention you wanna do something like this and number it id0.json, id1.json, id2.json... and so forth

```sh
solana-keygen new -o "id0.json"
```

Alternatively you can use this script to generate a lot of keys at once. this will create 20 keypairs.

```sh
chmod +x keygen.sh
./keygen.sh 0 20
```

3. You should next populate each of these wallets with 0.05 SOL. You can gather all the addresses by running this:
   You need to make sure you have Node installed.

```
npm install
node printAddresses.js
```

Get these addresses and use something like FFF airdrop tool to distribute the sol.

# Start mining

Run these commands to setup once.

```sh
cargo build
chmod +x run.sh
```

Step 2. Run this in the terminal one for each address (Max or Linux)

Where 0 replace with the keypair number.

```
./run.sh 0
```

Step 3. You can claim your ORE later by using this command:
Replace the brackets along with the inside of it with the API key and the id of the keypair

```sh
cargo run -- --rpc 'https://rpc.hellomoon.io/<HELLO MOON RPC>' --keypair id<ID OF KEYPAIR>.json --priority-fee 100000 claim
```

example:

```sh
cargo run -- --rpc 'https://rpc.hellomoon.io/a51d4e2f-b447-45f7-8a4a-78baf90bac12' --keypair id3.json --priority-fee 100000 claim
```

Open a terminal for each miner and change the integer.

Good luck

Optional:
Get a free hellomoon RPC endpoint:

you can edit the run.sh script:
replacing this line:

```sh
cargo run -- --keypair "id${x}.json" --priority-fee 100000 mine
```

to this line:

```sh
cargo run -- --rpc 'https://rpc.hellomoon.io/<HELLO MOON RPC>' --keypair "id${x}.json" --priority-fee 100000 mine
```
