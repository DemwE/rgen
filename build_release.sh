echo "Updating rust targets"
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-unknown-linux-gnu
echo "Rust targets updated"

echo "Building release binaries"
cargo build --release --target x86_64-pc-windows-gnu
cargo build --release --target x86_64-unknown-linux-gnu
echo "Done!"