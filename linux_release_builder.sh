cargo build --release
cp target/release/pen files/unix/linux/core
cd files/unix/linux/
sha256sum core > core.sha256
