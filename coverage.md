# MacOsX
cargo install grcov 
rustup default nightly  
cargo clean
env RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort" RUSTDOCFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests
-Cpanic=abort" CARGO_INCREMENTAL=0 RUSTC_WRAPPER='' cargo +nightly test --all

grcov ./target/debug -s ./ --binary-path ./target/debug/ -t html  --ignore-not-existing -o ./target/debug/coverage/

# Windows
set CARGO_INCREMENTAL=0
set RUSTFLAGS=-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort
set RUSTDOCFLAGS=-Cpanic=abort

export CARGO_INCREMENTAL=0
export RUSTFLAGS=-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort
export RUSTDOCFLAGS=-Cpanic=abort
cargo +nightly test --all --target x86_64-pc-windows-msvc
grcov ./target/x86_64-pc-windows-msvc/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/
grcov ../ -s ../ --binary-path ../ -t html  --ignore-not-existing -o ./target/debug/coverage/