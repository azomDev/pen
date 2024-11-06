if [[ "$OSTYPE" != "linux-gnu" ]]; then
  echo "This script is only intended to run on Linux."
  exit 1
fi

cargo build --release
cp target/release/pen release_files/unix/linux/core
sha256sum release_files/unix/linux/core > release_files/unix/linux/core.sha256
