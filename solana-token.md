# [Solana Token Program features](https://github.com/solana-labs/solana-program-library/tree/master/token)

First, for using the Solana programs, we need to install and configure the `Solana Toolchain`.
For that, once the tool chain installed, you can use the command `solana-keygen new` for creating a keypair.
We can also set the RPC URL to `http://api.testnet.solana.com` to connect to the testnet.

## spl-token command-line utility

All these functionalities are also available with a JavaScript binding.

  - Creating fungible token - `spl-token create-token`
  - Show supply of a fungible  - `spl-token supply <token-unique-identifier>`
  - Create Account - `spl-token create-account <token-unique-identifier>`
  - Show balance - `spl-token balance <token-unique-identifier>`
  - Show all tokens owned by an account - `spl-token accounts`
  - Mint tokens - `spl-token mint <token-unique-identifier> <supply-to-mint>`
  - Wrap SOL in a Token - `spl-token wrap <number-of-SOL-to-wrap>`
  - Unwrap token back to SOL - `spl-token unwrap <recipient-adress>`
  - Transfer tokens to another user - `spl-token transfer <token-unique-identifier> <amount> <recipient-adress>`
  - Transfer tokens to another user, with sender-funding - `spl-token transfer --fund-recipient <token-unique-identifier> <amount> <recipient-adress>`
  - Create Non-Fungible Token - `spl-token create-token --decimals 0`
  - Mint tokens - `spl-token mint <token-unique-identifier> 1 <recipient-token-account>`
  - Disable future mining - `spl-token authorize <token-unique-identifier> mint --disable`
  - Get Account infos - `spl-token account-info <token-unique-identifier>`
  - Create Multisig account - `spl-token create-multisig <minimum-number-of-signers> <public-keys-of-all-keypairs-allowed>`
  - Set mint account's minting authority to multisig account - `spl-token authorize <token-unique-identifier> mint <multisig accounts>`
  - Burn tokens - `spl-token burn <SOURCE_TOKEN_ACCOUNT_ADDRESS> <TOKEN_AMOUNT>`
  - Freeze Token Account - `spl-token freeze <TOKEN_ACCOUNT_ADDRESS>`
  - Thaw Token Account - `spl-token thaw <TOKEN_ACCOUNT_ADDRESS>`
  - Close Token Account - `spl-token close <TOKEN_ADDRESS>`

## [JSON RPC methods](https://docs.solana.com/developing/clients/jsonrpc-api)

- [`getTokenAccountBalance`](https://docs.solana.com/developing/clients/jsonrpc-api#gettokenaccountbalance) - Returns the token balance of an SPL Token account.
- [`getTokenAccountsByDelegate`](https://docs.solana.com/developing/clients/jsonrpc-api#gettokenaccountsbydelegate) - Returns all SPL Token accounts by approved Delegate.
- [`getTokenAccountsByOwner`](https://docs.solana.com/developing/clients/jsonrpc-api#gettokenaccountsbyowner) - Returns all SPL Token accounts by token owner.
- [`getTokenLargestAccounts`](https://docs.solana.com/developing/clients/jsonrpc-api#gettokenlargestaccounts) - Returns the 20 largest accounts of a particular SPL Token type.
- [`getProgramAccounts`](https://docs.solana.com/developing/clients/jsonrpc-api#getprogramaccounts) - fetch SPL Token accounts of interest.

## Address used for the tests

Address : BeghKTb3A2pBxGqwDt4MB5PzzNGDqdyLaMoNtnYg7zSM

Token :  5ZAifycb78GgqvW2dzBQTwysSEZYaJvdVvhTGWXmJmex

Token Account : DE2Q31HCvqBZvZTkZgukLvDTHYDDekkRPM67wEDzFjmT

NFT : 2Ywo82j5uJKyKDQWyHhXAaEHEycWVy4uPWpu1XbcyNu8


## Rust program structure 

### Solana depencies

The two programs (`cli` and `client`) have the fallowing dependecy in commun :
- `solana-sdk`
- `spl-token` (the `client` use the 2022 version))
- `spl-associated-token-account`
- `solana-client`
- `spl-memo`

The `client` library have `solana-program-test` as an additional dependency.

The `cli` program have as additional dependencies :
- `solana-account-decoder`
- `solana-clap-utils`
- `solana-cli-config`
- `solana-cli-output`
- `solana-logger`
- `solana-remote-wallet`
- `solana-transaction-status` 



### Structures

- `cli` - Command-line utility interface program
  - `main.rs` 
    - Import dependencies and modules of the program. 
    - Define contants of the commands and arguments taken by the cli.
    - Functions relatives to the differents commands inside the `main` function of the program.
    - Format Output.
  - `bench.rs` 
    - The `bench` subcommand and process command.
    - Get Token address and token validation functions.
    - Create and close accounts commands.
    - Deposit into or withdraw from commands.
    - Send message function.
  - `config.rs` - Configurations management with default behaviors.
  - `output.rs` - Various Display types for command outputs.
  - `rpc_client_utils.rs` - Spinner and progress bar functions.
  - `sort.rs` - Sort and parse token accounts.
- `client` - RPC Client.
  - `lib.rs` - List the modules of the librairy.
  - `client.rs` - Transactions and accounts operations executed by the RPC client.
  - `token.rs` - Tokens operation executed by the RPC client.
