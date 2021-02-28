# Minecraft Nether Generation in Rust

You can run this in C, C++, Rust and Python.

First thing: Get rust: https://rustup.rs

Second thing: Run Cargo: `cargo build --release`

Third thing: Install python bindings (optional): `python3 setup.py install --user` 
You can then use it in python as usual:
```python
from minecraft_nether_gen_rs import NetherGen,create_new_nether,get_biome,NetherBiomes
from ctypes import *
nether_gen:POINTER(NetherGen)=create_new_nether(1)
assert get_biome(nether_gen,0,0,0)==NetherBiomes.NetherWastes
```

Fourth thing: Use Rust:
```rust
let mut nether = NetherGen::new(171171);
let biome = nether.get_final_biome(19, 19, 19);
```

Fifth thing: Use C/C++: You have a shared library (.so/.dll) in target/release and a header file in target, you have two functions, just use them like any other functions ;) see example.c
```c
struct NetherGen *create_new_nether(uint64_t seed);

NetherBiomes get_biome(struct NetherGen *nether_gen, int32_t x, int32_t y, int32_t z);
```

Six thing (you can also run that in go, ruby and much more, rust/cbindings are awesome !)

Don't forget that structure use chunkX and chunkZ and that get_biome_structure(gen,chunkX,chunkZ)/get_biome_decorator(gen,chunkX,chunkZ) exist.