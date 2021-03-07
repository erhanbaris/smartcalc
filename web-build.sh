cd libsmartcalc
cargo build --target wasm32-unknown-unknown --release
wasm-pack build --out-dir ../www/pkg/ --target web --no-typescript
wasm-gc ../www/pkg/libsmartcalc_bg.wasm
cd ../
cd www/
python3 -m http.server