use noise::{NoiseFn, Perlin};

pub struct WorldNoise {
    pub height: Perlin,
    pub moisture: Perlin,
    pub flora_density: Perlin,
    pub flora_type: Perlin,
    pub terrain_detail: Perlin,
}

pub struct NoiseScales {
    pub height: f32,
    pub moisture: f32,
    pub flora_density: f32,
    pub flora_type: f32,
    pub terrain_detail: f32,
}

impl Default for NoiseScales {
    fn default() -> Self {
        Self {
            height: 0.021,
            moisture: 0.014,
            flora_density: 0.063,
            flora_type: 0.140,
            terrain_detail: 0.140,
        }
    }
}

pub fn fbm(noise: &Perlin, x: usize, y: usize, scale: f32, octaves: usize) -> f32 {
    debug_assert!(octaves > 0);

    let mut total = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    let mut max_value = 0.0;

    for _ in 0..octaves {
        let noise_x = x as f64 * scale as f64 * frequency;
        let noise_y = y as f64 * scale as f64 * frequency;

        total += noise.get([noise_x, noise_y]) * amplitude;
        max_value += amplitude;

        amplitude *= 0.5;
        frequency *= 2.0;
    }

    let normalized = (total / max_value + 1.0) / 2.0;

    normalized.clamp(0.0, 1.0) as f32
}

fn seed_to_u32(seed: &str, salt: u32) -> u32 {
    let mut hash: u32 = 2_166_136_261;

    for byte in seed.as_bytes() {
        hash ^= *byte as u32;
        hash = hash.wrapping_mul(16_777_619);
    }

    hash.wrapping_add(salt)
}

impl WorldNoise {
    pub fn new(seed: &str) -> Self {
        Self {
            height: Perlin::new(seed_to_u32(seed, 100)),
            moisture: Perlin::new(seed_to_u32(seed, 200)),
            flora_density: Perlin::new(seed_to_u32(seed, 300)),
            flora_type: Perlin::new(seed_to_u32(seed, 400)),
            terrain_detail: Perlin::new(seed_to_u32(seed, 500)),
        }
    }
}
