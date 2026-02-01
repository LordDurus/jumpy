cls
rem cargo run  --no-default-features --features gba --level "../worlds/01/01.lvlb" --target thumbv4t-none-eabi
rem cargo run --no-default-features --features gba --target thumbv4t-none-eabi
cargo +nightly run -Z build-std=core,alloc --no-default-features --features gba --target thumbv4t-none-eabi