run: fmt
  cargo run --features=dynamic_linking

build: fmt
  cargo build --features=dynamic_linking

#correctness/testing

fmt:
  cargo +nightly fmt

check:
  cargo check

test:
  cargo test --all --features=dynamic_linking

actions: fmt check test


#tools

kenny_gltf_fixer:
  cargo run -p kenny_gltf_fixer