set -e

npm run build
cd rust_project
cargo build --release
cd ..
cp rust_project/target/wasm32-unknown-unknown/release/rust_project.wasm dist/rust_project.wasm
devserver --path dist