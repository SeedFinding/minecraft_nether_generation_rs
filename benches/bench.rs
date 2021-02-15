use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::io::{BufWriter, Write};
use std::fs::File;
use minecraft_nether_generation::NetherGen;

fn gen_end(seed: u64, x:i32, z:i32) -> EndBiomes {
    let mut gen: NetherGen = NetherGen::new(seed);
    gen.get_biome(x,0,z)
}

fn gen1million(seed: u64, offset_x:i32, offset_z:i32) {
    let mut gen: NetherGen = NetherGen::new(seed);
    let mut f = BufWriter::new(File::create("out.txt").unwrap());
    for x in 0..1000 {
        for z in 0..1000 {
            write!(f, "{} ", gen.get_biome(offset_x + x, 0,offset_z + z) as u8).expect("Failed to write file");
        }
        writeln!(f).expect("Failed to write newline to file");
        f.flush().expect("fail to flush");
    }
}
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("gen end", |b| b.iter(|| gen_end(black_box(500),black_box(500),black_box(500))));
    c.bench_function("gen million end", |b| b.iter(|| gen1million(black_box(500),black_box(500),black_box(500))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);