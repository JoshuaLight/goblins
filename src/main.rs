mod domain;
mod random;

use time::Instant;

use rand::prelude::*;
use rand_xoshiro::SplitMix64;

use crate::domain::{Model, ModelOptions};

fn main() {
    const STEPS: usize = 1_000_000;

    let mut model = Model::new(ModelOptions {
        max_steps: STEPS,

        initial_capital: 1,
        income: 1,

        rng: SplitMix64::seed_from_u64(1),
        p_death: 0.1,
    });

    let now = Instant::now();

    model.initialize();

    for _ in 0..(STEPS - 1) {
        model.simulate();
    }

    let elapsed = now.elapsed();

    println!("---");
    println!("Population size: {}", STEPS);
    println!("Max capital: {}", model.max_capital);
    println!("---");
    println!("Duration: {:#?} ms.", elapsed.whole_milliseconds());
}
