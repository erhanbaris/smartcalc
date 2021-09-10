rustup default stable
rustup default nightly
cargo build -Z timings

# Execute tests
cargo test --all

cd libsmartcalc
wasm-pack test --node


# Desktop package creation
cd www
npm run gen

