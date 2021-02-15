#![allow(dead_code)]

use core::fmt;

use intmap::IntMap;
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
            NetherBiomes::TheVoid => {NETHER_WASTES}
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
        let mut nether =NetherGen::new(1);
        assert_eq!(nether.get_biome(0,0,0),NetherBiomes::NetherWastes);
    }

    #[test]
    fn gen_1_million(){
        let mut nether =NetherGen::new(1);
        let bound:i32=40;
        let mut score=0;
        for x in 0..bound {
            for y in 0..100 {
                for z in 0..100 {
                    score+=nether.get_biome(x,y,z) as i32;
                }
            }
        }
        dbg!(score);
    }
}


#[derive(Clone)]
pub struct NetherGen {
    seed: u64,
    temperature: DoublePerlinNoise,
    humidity: DoublePerlinNoise,
    altitude: DoublePerlinNoise,
    weirdness: DoublePerlinNoise,
    voronoi: Voronoi,
    cache3d: HashMap<u128, NetherBiomes>,
}

impl NetherGen {
    // this implementation of the nether generation is only for 1.16+ as it is biomes only
    pub fn new(mut seed: u64) -> Self {
        seed = seed & mask(48);
        NetherGen {
            seed,
            temperature: DoublePerlinNoise::new(Random::with_seed(seed), create_range(-7, -6)),
            humidity: DoublePerlinNoise::new(Random::with_seed(seed + 1), create_range(-7, -6)),
            altitude: DoublePerlinNoise::new(Random::with_seed(seed + 2), create_range(-7, -6)),
            weirdness: DoublePerlinNoise::new(Random::with_seed(seed + 3), create_range(-7, -6)),
            voronoi: Voronoi::new(sha2long(seed) as i64),
            cache3d:HashMap::new(),
        }
    }
    fn _sample(&mut self, x: i32, y: i32, z: i32)-> NetherBiomes{
        let t=self.temperature.sample(x as f64, y as f64, z as f64) as f32;
        let biome_point: BiomePoint = BiomePoint {
            biome: NetherBiomes::NetherWastes,
            temperature:t ,
            humidity: self.humidity.sample(x as f64, y as f64, z as f64) as f32,
            altitude: self.altitude.sample(x as f64, y as f64, z as f64) as f32,
            weirdness: self.weirdness.sample(x as f64, y as f64, z as f64) as f32,
            offset: 0.0f32,
        };

        DEFAULT_BIOMES.to_vec().iter().min_by(|&a, &b|
            a.distance_to(&biome_point).partial_cmp(&b.distance_to(&biome_point)).expect("Not infinity"))
            .map(|x| x.biome)
            .unwrap_or(NetherBiomes::TheVoid)
    }
    pub fn get_biome(&mut self, x: i32, y: i32, z: i32) -> NetherBiomes {
        let key: u128 = ((x as u128) << 64 | (y as u128) << 32 | (z as u128)) as u128;
        let value = self.cache3d.get(&key);
        if let Some(res) = value {
            return *res;
        }
        let value = self._sample(x, y, z);
        self.cache3d.insert(key, value);
        return value;
    }
}