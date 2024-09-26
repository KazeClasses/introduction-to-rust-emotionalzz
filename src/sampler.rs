use rand::{prelude::Distribution, rngs::StdRng, Rng, SeedableRng};
use statrs::distribution::MultivariateNormal;

pub struct State<const N_DIM: usize> {
    // Fill this
    rng: StdRng,
    pub arr: [f64; N_DIM],
    proposal_distribution: MultivariateNormal,
}

fn log_likelihood<const N_DIM: usize>(arr: &[f64]) -> f64 {
    // Fill this
}

impl<const N_DIM: usize> State<N_DIM> {
    pub fn new(seed: u64) -> Self {
    // Fill this
    }

    pub fn take_step(&mut self) {
    // Fill this
    }
}
