# [Solana Name Service Program](https://spl.solana.com/name-service)

The Solana Name Service allowe to resolve `.sol` domaine name.

Developers can interact with it mainly with a JavaScript binding.

JavaScript binding allow to interact with a spl program for issuing and managing
ownership of:

- domain names,
- Solana Pubkeys,
- URLs,
- twitter handles,
- arweave ids,
- metadata,
- etc..

The registry stores information about the domain name. It is made of two things:

- The header
- The data

Users can register a domain name with platforms like [Bonfida](https://naming.bonfida.org/#/auctions)



## Rust program structure

The programm use the `Solana-program-test`, `solana-sdk` and `solana-program` as main dependency.

- `lib.rs` - List the diffenrent modules of the librairy and export current sdk types for downstream users building with a different sdk version.
- `entrypoint.rs` - Declare the program entry point and set up global handlers.
- `error.rs` - Specific Name Service Result and Error for the librairy.
- `instruction.rs` - CRUD-like Instructions supported by the generic Name Registry program.
- `processor.rs` - Process funtions that operate the generic Name Registry program instructions.
- `state.rs` - Implementation of the differents states of the Name Record Header and get_seeds_and_key function.

## Rust Features

- Create an empty name record
- Update the data in a name record
- Transfer ownership of a name record
- Delete a name record.


## JavaScript binding features

### Resolve a domain name

In order to get the information of a domain name you need to:

- Hash the domain name
- Derive a PDA from the hash
- Retrieve the account info

### Reverse look up

This can be used to resolve the domain name from its public key.

### Subdomain look up

In order to resolve a subdomain you need to:

- Get the parent domain key
- Get the subdomain key
- Retrieve the account info

### Find all the domain names owned by a public key

You can retrieve all the domains owned by a public key using a `MemcmpFilter` filter.

### Favorite domain

Users have the possibility to select a domain name as their favorite one.

### Twitter handles

Users can register a Twitter handle on [Bonfida](https://naming.bonfida.org/#/twitter-registration). 

And then he can :
- Handle TLD
- Resolve an Twitter handle
- Reverse look up
