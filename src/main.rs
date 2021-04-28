use plotters::prelude::*;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use time::Instant;

trait WeightedRandom {
    fn index(&mut self, items: &Vec<u32>) -> usize;
}

struct RandWeighted {
    rng: StdRng,
}

impl RandWeighted {
    pub fn new() -> Self {
        Self {
            rng: StdRng::seed_from_u64(32),
        }
    }
}

impl WeightedRandom for RandWeighted {
    fn index(&mut self, items: &Vec<u32>) -> usize {
        let dist = WeightedIndex::new(items).unwrap();

        dist.sample(&mut self.rng)
    }
}

fn main() {
    const STEPS: u32 = 50_000;
    const INITIAL_CAPITAL: u32 = 1;
    const INCOME: u32 = 1;

    let mut random = RandWeighted::new();
    let mut population: Vec<u32> = Vec::with_capacity(STEPS as usize);

    // Add first person.
    population.push(INITIAL_CAPITAL);

    let mut max = INITIAL_CAPITAL;

    let now = Instant::now();

    for _ in 1..=STEPS {
        // Pick random human from the population.
        let human = random.index(&population);

        // Increase theirs capital.
        population[human] += INCOME;

        // Recalculate the maximum.
        if population[human] > max {
            max = population[human];
        }

        // Also add new human to the population.
        population.push(INITIAL_CAPITAL);
    }

    let elapsed = now.elapsed();

    println!("Duration: {:#?} s.", elapsed.whole_seconds());
    println!("Max: {:#?}", max);
}
