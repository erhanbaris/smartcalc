cd libsmartcalc
cargo build --target wasm32-unknown-unknown --release
wasm-pack build --out-dir ../pkg/
cd ../
cd www/
npm install
npm run start


set CARGO_INCREMENTAL=0
set RUSTFLAGS=-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort
set RUSTDOCFLAGS=-Cpanic=abort
cargo +nightly test --all --target x86_64-pc-windows-msvc
grcov ./target/x86_64-pc-windows-msvc/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/
grcov ../ -s ../ --binary-path ../ -t html  --ignore-not-existing -o ./target/debug/coverage/