cd libsmartcalc
cargo build --target wasm32-unknown-unknown --release
wasm-pack build --out-dir ../www/src/js/ --target web --no-typescript
wasm-gc ../www/src/js/libsmartcalc_bg.wasm
cd ../
cd www/
python3 -m http.server
python -m SimpleHTTPServer