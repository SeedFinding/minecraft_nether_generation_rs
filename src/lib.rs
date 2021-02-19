#![allow(dead_code)]

use core::fmt;

use java_random::{mask, Random};
use noise_rs::create_range;
use noise_rs::double_perlin_noise::DoublePerlinNoise;
use noise_rs::math::sha2long;
use noise_rs::voronoi::Voronoi;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BiomePoint {
    biome: NetherBiomes,
    temperature: f32,
    humidity: f32,
    altitude: f32,
    weirdness: f32,
    offset: f32,
}

impl BiomePoint {
    pub fn new(biome: NetherBiomes, temperature: f32, humidity: f32, altitude: f32, weirdness: f32, offset: f32) -> Self {
        BiomePoint { biome, temperature, humidity, altitude, weirdness, offset }
    }
    pub fn get_biome(&self) -> NetherBiomes {
        self.biome
    }
    pub fn distance_to(&self, other: &Self) -> f32 {
        ((self.temperature - other.temperature) * (self.temperature - other.temperature) +
            (self.humidity - other.humidity) * (self.humidity - other.humidity) +
            (self.altitude - other.altitude) * (self.altitude - other.altitude) +
            (self.weirdness - other.weirdness) * (self.weirdness - other.weirdness) +
            (self.offset - other.offset) * (self.offset - other.offset)) as f32
    }
}


impl From<NetherBiomes> for BiomePoint {
    fn from(nether_biome: NetherBiomes) -> Self {
        match nether_biome {
            NetherBiomes::NetherWastes => { NETHER_WASTES }
            NetherBiomes::SoulSandValley => { SOUL_SAND_VALLEY }
            NetherBiomes::CrimsonForest => { CRIMSON_FOREST }
            NetherBiomes::WarpedForest => { WARPED_FOREST }
            NetherBiomes::BasaltDeltas => { BASALT_DELTAS }
            NetherBiomes::TheVoid => { NETHER_WASTES }
        }
    }
}

impl fmt::Display for BiomePoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

const NETHER_WASTES: BiomePoint = BiomePoint {
    biome: NetherBiomes::NetherWastes,
    temperature: 0.0,
    humidity: 0.0,
    altitude: 0.0,
    weirdness: 0.0,
    offset: 0.0,
};

const SOUL_SAND_VALLEY: BiomePoint = BiomePoint {
    biome: NetherBiomes::SoulSandValley,
    temperature: 0.0,
    humidity: -0.5,
    altitude: 0.0,
    weirdness: 0.0,
    offset: 0.0,
};

const CRIMSON_FOREST: BiomePoint = BiomePoint {
    biome: NetherBiomes::CrimsonForest,
    temperature: 0.4,
    humidity: 0.0,
    altitude: 0.0,
    weirdness: 0.0,
    offset: 0.0,
};

const WARPED_FOREST: BiomePoint = BiomePoint {
    biome: NetherBiomes::WarpedForest,
    temperature: 0.0,
    humidity: 0.5,
    altitude: 0.0,
    weirdness: 0.0,
    offset: 0.375,
};
const BASALT_DELTAS: BiomePoint = BiomePoint {
    biome: NetherBiomes::BasaltDeltas,
    temperature: -0.5,
    humidity: 0.0,
    altitude: 0.0,
    weirdness: 0.0,
    offset: 0.175,
};

const DEFAULT_BIOMES: [BiomePoint; 5] = [NETHER_WASTES, SOUL_SAND_VALLEY, CRIMSON_FOREST, WARPED_FOREST, BASALT_DELTAS];

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NetherBiomes {
    NetherWastes = 8,
    TheVoid = 127,
    SoulSandValley = 170,
    CrimsonForest = 171,
    WarpedForest = 172,
    BasaltDeltas = 173,
}


impl From<BiomePoint> for NetherBiomes {
    fn from(biome_point: BiomePoint) -> Self {
        biome_point.biome
    }
}


impl fmt::Display for NetherBiomes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen1() {
        let mut nether = NetherGen::new(1);
        assert_eq!(nether.get_final_biome(0, 0, 0), NetherBiomes::NetherWastes);
        assert_eq!(nether.get_final_biome(100, 100, 100), NetherBiomes::SoulSandValley);
    }

    #[test]
    fn gen2() {
        let mut nether = NetherGen::new(171171);
        let biome = nether.get_final_biome(19, 19, 19);
        assert_eq!(biome, NetherBiomes::CrimsonForest);
    }

    #[test]
    fn gen3() {
        let mut nether = NetherGen::new(171171);
        let biome = nether.get_final_biome(92, 94, 0);
        assert_eq!(biome, NetherBiomes::NetherWastes);
    }

    #[test]
    fn gen_1_million() {
        let mut nether = NetherGen::new(171171);
        let bound: i32 = 100;
        let low = 0;
        let mut score = 0;
        for x in low..bound {
            for y in low..bound {
                for z in low..bound {
                    score += nether.get_final_biome(x, y, z) as i32;
                }
            }
        }
        assert_eq!(score, 113015032)
    }
}

/// <div rustbindgen hide></div>
#[derive(Clone)]
struct Noise {
    temperature: DoublePerlinNoise,
    humidity: DoublePerlinNoise,
    altitude: DoublePerlinNoise,
    weirdness: DoublePerlinNoise,
    voronoi: Voronoi,
    cache3d: HashMap<u128, NetherBiomes>,
}

#[repr(C)]
#[derive(Clone)]
pub struct NetherGen {
    seed: u64,
    _noise: Box<Noise>, // this because in C we can n
    is_3d: bool,
}

#[no_mangle]
pub extern "C" fn create_new_nether(seed: u64) -> Box<NetherGen> {
    Box::new(NetherGen::new(seed))
}

#[no_mangle]
pub unsafe extern "C" fn delete(nether_gen: &mut NetherGen) -> () {
    std::mem::drop(Box::from_raw(nether_gen));
}

#[no_mangle]
pub extern "C" fn get_biome(nether_gen: &mut NetherGen, x: i32, y: i32, z: i32) -> NetherBiomes {
    nether_gen.get_final_biome(x, y, z)
}


impl NetherGen {
    // this implementation of the nether generation is only for 1.16+ as it is biomes only
    // TODO fix the boxed into raw hiding (opaque pointer) to not use unsafe everywhere
    pub fn new(mut seed: u64) -> Self {
        let hashed_seed:i64=sha2long(seed) as i64;
        seed = seed & mask(48);
        let noise = Noise {
            temperature: DoublePerlinNoise::new(&mut Random::with_seed(seed), create_range(-7, -6)),
            humidity: DoublePerlinNoise::new(&mut Random::with_seed(seed + 1), create_range(-7, -6)),
            altitude: DoublePerlinNoise::new(&mut Random::with_seed(seed + 2), create_range(-7, -6)),
            weirdness: DoublePerlinNoise::new(&mut Random::with_seed(seed + 3), create_range(-7, -6)),
            voronoi: Voronoi::new(hashed_seed),
            cache3d: HashMap::new(),
        };
        let boxed: Box<Noise> = Box::new(noise);
        NetherGen {
            seed,
            _noise: boxed,
            is_3d: false,
        }
    }
    fn _sample(&mut self, x: i32, mut y: i32, z: i32) -> NetherBiomes {
        y = if self.is_3d { y } else { 0 };
        let biome_point: BiomePoint = BiomePoint {
            biome: NetherBiomes::NetherWastes,
            temperature:  (*self._noise).temperature.sample(x as f64, y as f64, z as f64) as f32 ,
            humidity:   (*self._noise).humidity.sample(x as f64, y as f64, z as f64) as f32 ,
            altitude:   (*self._noise).altitude.sample(x as f64, y as f64, z as f64) as f32 ,
            weirdness:   (*self._noise).weirdness.sample(x as f64, y as f64, z as f64) as f32 ,
            offset: 0.0f32,
        };
        DEFAULT_BIOMES.to_vec().iter().min_by(|&a, &b|
            a.distance_to(&biome_point).partial_cmp(&b.distance_to(&biome_point)).expect("Not infinity"))
            .map(|x| x.biome)
            .unwrap_or(NetherBiomes::TheVoid)
    }

    pub fn get_final_biome(&mut self, x: i32, y: i32, z: i32) -> NetherBiomes {
        let (xx, yy, zz): (i32, i32, i32) =  self._noise.voronoi.get_fuzzy_positions(x, y, z) ;
        return self.get_biome(xx, yy, zz);
    }

    pub fn get_biome(&mut self, x: i32, y: i32, z: i32) -> NetherBiomes {
        // WARNING this is not the final method, use the one with voronoi
        let key: u128 = (((x as u32) as u128) << 64 | ((y as u32) as u128) << 32 | ((z as u32) as u128)) as u128;
        let value =  (*self._noise).cache3d.get(&key) ;
        if let Some(res) = value {
            return *res;
        }
        let value = self._sample(x, y, z);
        (*self._noise).cache3d.insert(key, value) ;
        return value;
    }
}