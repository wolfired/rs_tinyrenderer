//! 噪声
//!
//! * 程序噪声
//!   * 基于晶格的方法（lattice based）
//!     * 梯度噪声（gradient noise）
//!       * perlin noise
//!       * simplex noise
//!       * wavelet noise
//!     * value noise
//!   * 基于点的方法（point based）
//!     * worley noise
//!
//!
//!

use crate::la::Dot;
use crate::la::Vector2;

const Hash2XPredefineA: [f32; 2] = [127.1, 311.7];
const Hash2XPredefineB: [f32; 2] = [269.5, 183.3];
const Hash2XPredefineC: f32 = 43758.5453123;

fn hash21_inner(p: Vector2<f32>, a: Option<Vector2<f32>>) -> f32 {
    let r = a.unwrap_or(Hash2XPredefineA.into());
    let v = p.dot(r);
    -1.0 + 2.0 * (v.sin() * Hash2XPredefineC).fract()
}

///
/// ```glsl
///  float hash21(vec2 p) {
///      float h = dot(p, vec2(127.1, 311.7));
///      return -1.0 + 2.0 * fract(sin(h) * 43758.5453123);
///  }
/// ```
///
/// return [-1, 1]
pub fn hash21(p: Vector2<f32>) -> f32 {
    hash21_inner(p, Some(Hash2XPredefineA.into()))
}

///
/// ```glsl
///  vec2 hash22(vec2 p) {
///      p = vec2(dot(p, vec2(127.1, 311.7)), dot(p, vec2(269.5, 183.3)));
///      return -1.0 + 2.0 * fract(sin(p) * 43758.5453123);
///  }
/// ```
///
/// return [-1, 1]
pub fn hash22(p: Vector2<f32>) -> Vector2<f32> {
    [hash21_inner(p, Some(Hash2XPredefineA.into())), hash21_inner(p, Some(Hash2XPredefineB.into()))].into()
}

/// https://libnoise.sourceforge.net/noisegen/index.html
/// return [-1, 1]
pub fn integer_noise_1d(mut x: i32) -> f32 {
    x = (x >> 13) ^ x;
    x = (x * (x * x * 60493 + 19990303) + 1376312589) & 0x7fffffff;
    return 1.0 - (x as f32 / 1073741824.0);
}

pub fn lerp_s(a: f32, b: f32, t: f32) -> f32 {
    (1.0 - t) * a + t * b
}

pub fn fade(t: f32) -> f32 {
    t * t * t * (t * (6.0 * t - 15.0) + 10.0)
}

pub fn perlin_noise_1d(x: f32) -> f32 {
    let x0 = x.floor();
    let x1 = x0 + 1.0;

    let g0 = integer_noise_1d(x0 as i32);
    let g1 = integer_noise_1d(x1 as i32);

    let v0 = x - x0;
    let v1 = x - x1;

    let x = x.fract();

    lerp_s(g0 * v0, g1 * v1, x)
}

// frequency 频率
// amplitude 振幅
// persistence 持续度
// octaves 倍频

// 分形噪声（）
//   频率翻倍
//   振幅减半

pub fn fbm(x: f32, octaves: i8) -> f32 {
    let mut frequency = 1.0;
    let mut amplitude = 1.0;
    let mut amplitude_sum = 0.0;

    let mut result = 0.0;

    for _ in 0..octaves {
        result += perlin_noise_1d(x * frequency) * amplitude;
        amplitude_sum += amplitude;

        frequency *= 2.0;
        amplitude /= 2.0;
    }

    result / amplitude_sum
}
