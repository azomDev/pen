if [[ "$OSTYPE" != "darwin"* ]]; then
  echo "This script is only intended to run on macOS."
  exit 1
fi

cargo build --release
cp target/release/pen release_files/unix/macos/core
cd release_files/unix/macos
sha256sum core > core.sha256
