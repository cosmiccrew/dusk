`Cosmic Crew: Dusk`
==================

Cosmic Crew: Dusk ~~is~~ will be a conceptual, 3d interplanitary base building game. The main inspiration comes from the "Space Kit" free assets found at [kenny.nl/assets/space-kit](https://kenney.nl/assets/space-kit). **NOTE: This project is in an extraordinarily early stage, and the idea, concept and any related content is subject to drastic change and modification.**

-------

## Download & play

To run and play Cosmic Crew: Dusk, there are a few options:


#### Releases (PLANNED)

1. ~~By downloading the game from the github release page~~
2. ~~Extracting the archive and running the executable~~


#### From source

1. Install rust at [rustup.rs](https://rustup.rs)
2. Clone the repo `git clone https://github.com/cosmiccrew/dusk.git`
3. `cd dusk`
4. `cargo run` or (for increased runtime performance, but longer compile times) `cargo run --release`

-------

## Contributing

Any and all contributions are welcome! Pull requests are checked for `cargo test`, `cargo clippy` and `cargo +nightly fmt`. Note this project uses unstable cargo fmt settings, and requires installing and running cargo fmt on the nighlty edition.

Before submitting a PR or issue, please run the following commands and follow their instructions:
1. `cargo clippy`
2. `cargo +nightly fmt`

#### Dev builds

The development build by default has some **runtime performance** improvements enabled - however, to **speed up compile times** (namely using bevy's internal dynamic linking feature), a simple feature flag can be enabled:
```bash
cargo run --profile dynamic_linking
```
please note: this will decrease some runtime performance.

-------

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.