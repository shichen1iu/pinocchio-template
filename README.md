# Pinocchio Solana program template
This is a template for optimized Solana program development using the Pinocchio framework. This template was built by [Exo Technologies](https://exotechnologies.xyz).

## Running Tests

Run integration tests with the following script from project root
```
cargo-build-sbf && SBF_OUT_DIR=$(pwd)/target/sbf-solana-solana/release cargo test
```

## Generating IDL
This repository uses Shank for IDL generation.

Install the Shank CLI
```
cargo install shank-cli
```

Generate IDL
```
shank idl -r program -o idl
// OR
yarn generate-idl
```

## Generating Clients
*Ensure the IDL has been created or updated using the above IDL generation steps.*

Install dependencies
```
yarn install
```

Run client generation script
```
yarn generate-clients
```

