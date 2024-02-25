
#cargo build --release

## below reduces our binary from 314kb to 302kb, but requires rust nightly
RUSTFLAGS="-Zlocation-detail=none" cargo +nightly build --release

## below doesn't reduce size in our case but complicates build because it requires specifying the correct target
#RUSTFLAGS="-Zlocation-detail=none" cargo +nightly build -Z build-std=std,panic_abort --target x86_64-unknown-linux-gnu --release
#RUSTFLAGS="-Zlocation-detail=none" cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu --release

ldd target/release/libjni_notifications.so
file target/release/libjni_notifications.so
ls -alih target/release/libjni_notifications.so