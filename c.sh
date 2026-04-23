cargo build --target wasm32-unknown-unknown --release

cp ../target/wasm32-unknown-unknown/release/*.wasm game.wasm

python3 -m http.server 8080
