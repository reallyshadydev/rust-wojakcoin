# Rust Wojakcoin

Rust library for [Wojakcoin (WJK)](https://wojakcoin.cash): de/serialization, parsing, and handling of data structures and network messages.

Fork of [rust-pepecoin](https://github.com/Nintondo/rust-pepecoin) (from [rust-bitcoin](https://github.com/rust-bitcoin/rust-bitcoin)).

## Features

- De/serialization of Wojakcoin protocol network messages
- De/serialization of blocks and transactions
- Script de/serialization
- Private keys and address creation, de/serialization and validation (BIP32)
- PSBT v0 de/serialization (use rust-miniscript to finalize)

Network parameters match [wojakcore](https://github.com/wojakcoin/wojakcore) chainparams: mainnet magic `0x6f8da579`, address prefix 0x49 (W), script 0x05, WIF 0xc9.

## Building

```bash
git clone https://github.com/reallyshadydev/rust-wojakcoin.git
cd rust-wojakcoin
cargo build
cargo test --package nintondo-wojakcoin
```

## Usage

```toml
[dependencies]
nintondo-wojakcoin = { path = "nintondo-wojakcoin", features = ["rand-std"] }
```

```rust
use nintondo_wojakcoin::{Address, Network, PublicKey};
use nintondo_wojakcoin::secp256k1::{rand, Secp256k1};

let secp = Secp256k1::new();
let (_, public_key) = secp.generate_keypair(&mut rand::thread_rng());
let pubkey = PublicKey::new(public_key);
let address = Address::p2pkh(pubkey, Network::Wojakcoin);
```

## License

CC0-1.0 (see [LICENSE](LICENSE)).
