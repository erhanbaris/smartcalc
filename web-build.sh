cd libsmartcalc
cargo build --target wasm32-unknown-unknown --release
wasm-pack build --out-dir ../pkg/
cd ../
cd www/
npm install
npm run start
