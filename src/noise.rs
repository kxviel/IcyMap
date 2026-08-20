use noise::{NoiseFn, Perlin};

pub(crate) struct WorldNoise {
    pub(crate) height: Perlin,
    pub(crate) moisture: Perlin,
    pub(crate) flora_density: Perlin,
    pub(crate) flora_type: Perlin,
    pub(crate) terrain_detail: Perlin,
}

pub(crate) struct NoiseScales {
    pub(crate) height: f32,
    pub(crate) moisture: f32,
    pub(crate) flora_density: f32,
    pub(crate) flora_type: f32,
    pub(crate) terrain_detail: f32,
}

pub(crate) fn fbm_gen(
    noise: &Perlin,
    x: usize,
    y: usize,
    base_scale: f64,
    octaves: usize,
    persistence: f64,
    lacunarity: f64,
) -> f32 {
    debug_assert!(octaves > 0);
    debug_assert!(persistence > 0.0);
    debug_assert!(lacunarity > 0.0);

    let mut total = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    let mut max_value = 0.0;

    for _ in 0..octaves {
        let noise_x = x as f64 * base_scale * frequency;

        let noise_y = y as f64 * base_scale * frequency;

        total += noise.get([noise_x, noise_y]) * amplitude;
        max_value += amplitude;

        amplitude *= persistence;
        frequency *= lacunarity;
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
    pub(crate) fn new(seed: &str) -> Self {
        Self {
            height: Perlin::new(seed_to_u32(seed, 100)),
            moisture: Perlin::new(seed_to_u32(seed, 200)),
            flora_density: Perlin::new(seed_to_u32(seed, 300)),
            flora_type: Perlin::new(seed_to_u32(seed, 400)),
            terrain_detail: Perlin::new(seed_to_u32(seed, 500)),
        }
    }
}
