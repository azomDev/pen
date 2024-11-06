if [[ "$OSTYPE" != "darwin"* ]]; then
  echo "This script is only intended to run on macOS."
  exit 1
fi

cargo build --release
cp target/release/pen release_files/unix/macos/core
sha256sum release_files/unix/macos/core > release_files/unix/macos/core.sha256
