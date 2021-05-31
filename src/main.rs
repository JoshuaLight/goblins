mod model;
mod random;

use gnuplot::{Axes2D, Figure};
use time::Instant;

use rand::prelude::*;
use rand_xoshiro::SplitMix64;

use crate::model::{Model, ModelOptions, RandomStrategy};

fn main() {
    const STEPS: usize = 1_000_000;

    let mut fg = Figure::new();
    let axes = fg.axes2d();

    let model_a = Model::new(new_options::<SplitMix64>(
        STEPS,
        RandomStrategy::Uniform,
        RandomStrategy::Uniform,
    ));
    let model_b = Model::new(new_options::<SplitMix64>(
        STEPS,
        RandomStrategy::Weighted,
        RandomStrategy::Uniform,
    ));
    let model_c = Model::new(new_options::<SplitMix64>(
        STEPS,
        RandomStrategy::Uniform,
        RandomStrategy::Weighted,
    ));
    let model_d = Model::new(new_options::<SplitMix64>(
        STEPS,
        RandomStrategy::Weighted,
        RandomStrategy::Weighted,
    ));

    simulate(model_a, STEPS, axes, "A");
    simulate(model_b, STEPS, axes, "B");
    simulate(model_c, STEPS, axes, "C");
    simulate(model_d, STEPS, axes, "D");

    fg.show().unwrap();
}

fn new_options<R: RngCore + SeedableRng>(
    steps: usize,
    rnd_income: RandomStrategy,
    rnd_death: RandomStrategy,
) -> ModelOptions<R> {
    ModelOptions {
        max_steps: steps,

        initial_gold: 1,
        f_income: Box::new(|x| x + 1),

        rng: R::seed_from_u64(1),

        rnd_income,
        rnd_death,

        p_income: 1.0,
        p_birth: 1.0,
        p_death: 0.11,
    }
}

fn simulate<R: RngCore>(mut m: Model<R>, steps: usize, fg: &mut Axes2D, caption: &str) {
    let now = Instant::now();

    m.init();

    for _ in 1..(steps - 1) {
        m.sim();
    }

    let report = m.finish();
    let _ = now.elapsed();

    report.draw_to(fg, caption);
}
