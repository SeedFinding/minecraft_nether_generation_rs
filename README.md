# Minecraft Nether Generation in Rust

You can run this in C, C++, Rust and Python.

First thing: Get rust: https://rustup.rs

Second thing: Run Cargo: `cargo build --release`

Third thing: Install python bindings (optional): `python3 setup.py install --user`

Fourth thing: Use Rust:
```rust
let mut nether = NetherGen::new(171171);
let biome = unsafe { nether.get_final_biome(19, 19, 19) };
```
( I am so sorry about the unsafe, I promise I will fix it, for now it's due to the C stuff on top)

Fifth thing: Use C/C++: You have a shared library (.so/.dll) in target/release and a header file in target, you have two functions, just use them like any other functions ;) see example.c
```c
struct NetherGen *create_new_nether(uint64_t seed);

NetherBiomes get_biome(struct NetherGen *nether_gen, int32_t x, int32_t y, int32_t z);
```

Six thing (you can also run that in go, ruby and much more, rust/cbindings are awesome !)


