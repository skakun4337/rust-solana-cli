# Rust Solana CLI program

This program is of Proof of Concept for creating a Solana Token (SPL Token), minting 10_000_000 of it, disable theminting and show the balance for this token with the command `rust-solana-cli create-token --name=<token-name>`.

The `rust-solana-cli show-accounts` command list all the tokens of the current account;

The `rust-solana-cli connect` command create a libp2p connection that can ping another libp2p connection if run in two separates terminals.

The program need you to have : 
- the `Solana Toolchain` installed
- the `spl-token cli` installed
- a keypair generated (`solana-keygen new`)
- the RPC URL set to the solana testnet (`http://api.testnet.solana.com`) 
- at least some SOL air dropped on your account.

### Setting commands
```
$ solana-keygen new
$ solana config set --url https://api.testnet.solana.com

if needed :
$ solana config set --keypair ${HOME}/new-keypair.json

$ solana airdrop 1
```

## Technologies used

- [Rust](https://www.rust-lang.org)
- [libp2p](https://docs.rs/libp2p/latest/libp2p/index.html)
- [Solana Token Program](https://github.com/solana-labs/solana-program-library/tree/master/token)

## Example Output

```
$ target/debug/rust-solana-cli create-token -n=JOE

Token name : "JOE"

Created Token Hash : 7HrENEytzV32yNygpsJS7kT2UWvHrzawcbXKCPNUSu5H
Created Account Hash :  BEHLC6Agk1bmdRjYqapD1NdkC7goRShd1yLKMoiXSFig

Minting 10000000 tokens
  Token: 7HrENEytzV32yNygpsJS7kT2UWvHrzawcbXKCPNUSu5H
  Recipient: BEHLC6Agk1bmdRjYqapD1NdkC7goRShd1yLKMoiXSFig

Signature: 3SQrXH9L9jAMByDxbJkrFtwQ3qek2zQNZfp33X4MJ8EBz7pUJikqB99xSkSCYmyqWwzxJ4gZDCBKz5j62jrD7FN3


Updating 7HrENEytzV32yNygpsJS7kT2UWvHrzawcbXKCPNUSu5H
  Current mint authority: BeghKTb3A2pBxGqwDt4MB5PzzNGDqdyLaMoNtnYg7zSM
  New mint authority: disabled

Signature: 65iKup4ydzDq4jfXaP5rkwN2dEMwN84Bs3GCxgCFDYBKcYQdZz5Fhe9uBjq4hJuav4MnjXPx1XZYamwTExQ2C4UL


Balance : 10000000
```
## Chat app

A basic chat application demonstrating libp2p and the mDNS and floodsub protocols.

Using two terminal windows, start two instances. If you local network allows mDNS,
they will automatically connect. Type a message in either terminal and hit return: the
message is sent and printed in the other terminal. Close with Ctrl-c.

You can of course open more terminal windows and add more participants.
Dialing any of the other peers will propagate the new participant to all
chat members and everyone will receive all messages.

### If they don't automatically connect

If the nodes don't automatically connect, take note of the listening addresses of the first
instance and start the second with one of the addresses as the first argument. In the first
terminal window, run:

```sh
rust-solana-cli chat
```

It will print the PeerId and the listening addresses, e.g. `Listening on
"/ip4/0.0.0.0/tcp/24915"`

In the second terminal window, start a new instance of the example with:

```sh
rust-solana-cli chat --address=/ip4/127.0.0.1/tcp/24915
```

The two nodes then connect.
