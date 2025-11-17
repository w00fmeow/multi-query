curl -L https://github.com/roblabla/MacOSX-SDKs/releases/download/macosx14.5/MacOSX14.5.sdk.tar.xz | tar xJ
export SDKROOT=$(pwd)/MacOSX14.5.sdk/
export PATH=$PATH:~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/
export CARGO_TARGET_X86_64_APPLE_DARWIN_LINKER=rust-lld
rustup target add x86_64-apple-darwin
cargo build --release --target x86_64-apple-darwin