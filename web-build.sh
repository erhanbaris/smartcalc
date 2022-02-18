cd libsmartcalc
cargo build --target wasm32-unknown-unknown --release
wasm-pack build --out-dir ../assets/www/src/js/ --target web --no-typescript
wasm-gc ../assets/www/src/js/libsmartcalc_bg.wasm
cd ../assets/www/
python3 -m http.server
python -m SimpleHTTPServer
