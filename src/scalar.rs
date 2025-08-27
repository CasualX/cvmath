/*!
Scalar functions.
*/

use super::*;

/// Linear interpolation between scalars.
///
/// - The `t` parameter is not clamped.
/// - Equivalent to `a + (b - a) * t`
///
/// <!--SCALAR_LERP--><svg width="420" height="80" font-family="monospace" xmlns="http://www.w3.org/2000/svg"><line x1="40" y1="40" x2="380" y2="40" stroke="white" /><circle cx="40" cy="40" r="2" fill="white" /><circle cx="380" cy="40" r="2" fill="white" /><text x="36" y="60" fill="white">a</text><text x="376" y="60" fill="white">b</text><circle cx="125" cy="40" r="2" fill="lime" /><text x="113" y="25" fill="lime">t = 0.25</text><circle cx="210" cy="40" r="2" fill="deepskyblue" /><text x="198" y="25" fill="deepskyblue">t = 0.5</text></svg>
#[inline]
pub fn lerp<T: Float>(a: T, b: T, t: T) -> T {
	a + (b - a) * t
}

/// Step function (Heaviside-like).
///
/// - Returns zero if `x < edge`, one otherwise (discontinuous at `edge`).
/// - Useful for thresholds and simple masks.
///
/// <!--SCALAR_STEP--><svg width="420" height="140" font-family="monospace" xmlns="http://www.w3.org/2000/svg"><text x="10" y="120" fill="white">0</text><text x="10" y="20" fill="white">1</text><line x1="40" y1="120" x2="200" y2="120" stroke="white" /><line x1="200" y1="120" x2="200" y2="20" stroke="grey" stroke-dasharray="5.0, 5.0" /><line x1="200" y1="20" x2="380" y2="20" stroke="white" /><text x="240" y="35" fill="white">x ≥ edge</text><text x="100" y="135" fill="white">x < edge</text></svg>
#[inline]
pub fn step<T: Scalar>(edge: T, x: T) -> T {
	if x < edge { T::ZERO } else { T::ONE }
}

/// Cubic smooth step between edges.
///
/// - Maps `x` in `[edge0, edge1]` to `t ∈ [0, 1]`, then applies `3t^2 − 2t^3`.
/// - Output is clamped to `[0, 1]` outside the edges; C1 continuous.
///
/// <!--SCALAR_SMOOTHSTEP--><svg width="420" height="140" font-family="monospace" xmlns="http://www.w3.org/2000/svg"><text x="10" y="120" fill="white">0</text><text x="10" y="20" fill="white">1</text><line x1="120" y1="120" x2="120" y2="20" stroke="grey" stroke-dasharray="5.0, 5.0" /><line x1="320" y1="120" x2="320" y2="20" stroke="grey" stroke-dasharray="5.0, 5.0" /><line x1="40" y1="120" x2="120" y2="120" stroke="white" /><polyline points="120.0 120.0 122.0 120.0 124.0 119.9 126.0 119.7 128.0 119.5 130.0 119.3 132.0 119.0 134.0 118.6 136.0 118.2 138.0 117.7 140.0 117.2 142.0 116.6 144.0 116.0 146.0 115.4 148.0 114.7 150.0 113.9 152.0 113.1 154.0 112.3 156.0 111.4 158.0 110.5 160.0 109.6 162.0 108.6 164.0 107.6 166.0 106.6 168.0 105.5 170.0 104.4 172.0 103.2 174.0 102.1 176.0 100.9 178.0 99.6 180.0 98.4 182.0 97.1 184.0 95.8 186.0 94.5 188.0 93.2 190.0 91.8 192.0 90.5 194.0 89.1 196.0 87.7 198.0 86.2 200.0 84.8 202.0 83.4 204.0 81.9 206.0 80.4 208.0 79.0 210.0 77.5 212.0 76.0 214.0 74.5 216.0 73.0 218.0 71.5 220.0 70.0 222.0 68.5 224.0 67.0 226.0 65.5 228.0 64.0 230.0 62.5 232.0 61.0 234.0 59.6 236.0 58.1 238.0 56.6 240.0 55.2 242.0 53.8 244.0 52.3 246.0 50.9 248.0 49.5 250.0 48.2 252.0 46.8 254.0 45.5 256.0 44.2 258.0 42.9 260.0 41.6 262.0 40.4 264.0 39.1 266.0 37.9 268.0 36.8 270.0 35.6 272.0 34.5 274.0 33.4 276.0 32.4 278.0 31.4 280.0 30.4 282.0 29.5 284.0 28.6 286.0 27.7 288.0 26.9 290.0 26.1 292.0 25.3 294.0 24.6 296.0 24.0 298.0 23.4 300.0 22.8 302.0 22.3 304.0 21.8 306.0 21.4 308.0 21.0 310.0 20.7 312.0 20.5 314.0 20.3 316.0 20.1 318.0 20.0 320.0 20.0" stroke="white" fill="none" stroke-width="1.5" /><line x1="320" y1="20" x2="380" y2="20" stroke="white" /><text x="100" y="135" fill="white">edge0</text><text x="300" y="135" fill="white">edge1</text></svg>
#[inline]
pub fn smoothstep<T: Float>(edge0: T, edge1: T, x: T) -> T {
	debug_assert!(edge0 < edge1, "smoothstep requires edge0 < edge1");

	let two = T::ONE + T::ONE;
	let three = two + T::ONE;

	let t = ((x - edge0) / (edge1 - edge0)).clamp(T::ZERO, T::ONE);

	t * t * (three - two * t)
}

/// Quintic smoother step between edges.
///
/// - Maps `x` in `[edge0, edge1]` to `t ∈ [0, 1]`, then applies `6t^5 − 15t^4 + 10t^3`.
/// - Output is clamped to `[0, 1]`; C2 continuous (flatter ends than [`smoothstep`]).
///
/// <!--SCALAR_SMOOTHERSTEP--><svg width="420" height="140" font-family="monospace" xmlns="http://www.w3.org/2000/svg"><text x="10" y="120" fill="white">0</text><text x="10" y="20" fill="white">1</text><line x1="120" y1="120" x2="120" y2="20" stroke="grey" stroke-dasharray="5.0, 5.0" /><line x1="320" y1="120" x2="320" y2="20" stroke="grey" stroke-dasharray="5.0, 5.0" /><line x1="40" y1="120" x2="120" y2="120" stroke="white" /><polyline points="120.0 120.0 122.0 120.0 124.0 120.0 126.0 120.0 128.0 119.9 130.0 119.9 132.0 119.8 134.0 119.7 136.0 119.5 138.0 119.4 140.0 119.1 142.0 118.9 144.0 118.6 146.0 118.2 148.0 117.8 150.0 117.3 152.0 116.8 154.0 116.3 156.0 115.6 158.0 114.9 160.0 114.2 162.0 113.4 164.0 112.6 166.0 111.6 168.0 110.7 170.0 109.6 172.0 108.6 174.0 107.4 176.0 106.2 178.0 105.0 180.0 103.7 182.0 102.3 184.0 100.9 186.0 99.5 188.0 98.0 190.0 96.5 192.0 94.9 194.0 93.3 196.0 91.7 198.0 90.0 200.0 88.3 202.0 86.5 204.0 84.7 206.0 83.0 208.0 81.1 210.0 79.3 212.0 77.5 214.0 75.6 216.0 73.7 218.0 71.9 220.0 70.0 222.0 68.1 224.0 66.3 226.0 64.4 228.0 62.5 230.0 60.7 232.0 58.9 234.0 57.0 236.0 55.3 238.0 53.5 240.0 51.7 242.0 50.0 244.0 48.3 246.0 46.7 248.0 45.1 250.0 43.5 252.0 42.0 254.0 40.5 256.0 39.1 258.0 37.7 260.0 36.3 262.0 35.0 264.0 33.8 266.0 32.6 268.0 31.4 270.0 30.4 272.0 29.3 274.0 28.4 276.0 27.4 278.0 26.6 280.0 25.8 282.0 25.1 284.0 24.4 286.0 23.7 288.0 23.2 290.0 22.7 292.0 22.2 294.0 21.8 296.0 21.4 298.0 21.1 300.0 20.9 302.0 20.6 304.0 20.5 306.0 20.3 308.0 20.2 310.0 20.1 312.0 20.1 314.0 20.0 316.0 20.0 318.0 20.0 320.0 20.0" stroke="white" fill="none" stroke-width="1.5" /><line x1="320" y1="20" x2="380" y2="20" stroke="white" /><text x="100" y="135" fill="white">edge0</text><text x="300" y="135" fill="white">edge1</text></svg>
#[inline]
pub fn smootherstep<T: Float>(edge0: T, edge1: T, x: T) -> T {
	debug_assert!(edge0 < edge1, "smootherstep requires edge0 < edge1");

	let two = T::ONE + T::ONE;
	let three = two + T::ONE;
	let five = two + three;
	let six = two * three;
	let ten = five * two;
	let fifteen = five * three;

	let t = ((x - edge0) / (edge1 - edge0)).clamp(T::ZERO, T::ONE);

	t * t * t * (t * (t * six - fifteen) + ten)
}
