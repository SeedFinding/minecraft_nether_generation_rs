use java_random::Random;
use std::time::{SystemTime};
use minecraft_nether_gen_rs::NetherGen;

fn main() {
    let seed: u64 = 1551515151585454;
    let offset_x: i32 = 10000;
    let offset_z: i32 = 10000;
    let now = SystemTime::now();
    run_offset(seed, offset_x, offset_z);
    println!("{}", now.elapsed().expect("error").as_secs_f64());
    let now = SystemTime::now();
    run_random(seed);
    println!("{}", now.elapsed().expect("error").as_secs_f64())
}

fn run_offset(seed: u64, offset_x: i32, offset_z: i32) {
    let now = SystemTime::now();
    let mut gen: NetherGen = unsafe { NetherGen::new(seed) };
    println!("{}", now.elapsed().expect("error").as_secs_f64());
    let mut som: i32 = 0;

    for x in 0..1000 {
        for z in 0..1000 {
            unsafe { som = som.wrapping_add(gen.get_final_biome(x + offset_x, 0, z + offset_z) as i32); }
        }
    }
    println!("{}", som);
}

fn run_random(seed: u64) {
    let mut gen: NetherGen = unsafe { NetherGen::new(seed) };
    let mut som: i32 = 0;
    let mut r = Random::with_seed(1);

    for _ in 0..100 {
        for _ in 0..100 {
            unsafe { som = som.wrapping_add(gen.get_final_biome(r.next_int_n(1000), 0, r.next_int_n(1000)) as i32); }
        }
    }
    println!("{}", som);
}