cargo build --release
cp target/release/pen release_files/unix/linux/core
sha256sum release_files/unix/linux/core > release_files/unix/linux/core.sha256
