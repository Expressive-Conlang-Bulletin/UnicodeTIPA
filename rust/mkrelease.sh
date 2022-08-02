cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu
cp target/x86_64-unknown-linux-gnu/release/unicode-tipa target/unicode-tipa-x86_64-unknown-linux-gnu
cp target/x86_64-pc-windows-gnu/release/unicode-tipa.exe target/unicode-tipa-x86_64-pc-windows-gnu.exe