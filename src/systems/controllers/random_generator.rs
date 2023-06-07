use std::ops::Range;

use rand::{rngs::StdRng, Rng, SeedableRng};

#[allow(dead_code)]
pub fn random_value_f32(seed: u64, range: Range<f32>) -> f32 {
    let mut rng = StdRng::seed_from_u64(seed);

    rng.gen_range(range.start..range.end)
}

#[allow(dead_code)]
pub fn generate_seed() -> u64 {
    let mut rng = rand::thread_rng();

    rng.gen_range(u64::MIN..u64::MAX)
}

#[cfg(test)]
mod random_generator_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1234, -20.0..20.0, -17.736874)]
    #[case(4321, -20.0..20.0, 8.739052)]
    #[case(4321, 0.0..100.0, 71.847626)]
    fn generate_a_random_value_f32(
        #[case] seed: u64,
        #[case] range: Range<f32>,
        #[case] expected: f32,
    ) {
        // When
        let actual = random_value_f32(seed, range);

        // Then
        assert_eq!(expected, actual);
    }
}
