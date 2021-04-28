use plotters::prelude::*;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand_xoshiro::SplitMix64;
use time::Instant;

struct RandWeighted {
    rng: SplitMix64,
}

impl RandWeighted {
    pub fn new() -> Self {
        Self {
            rng: SplitMix64::seed_from_u64(1),
        }
    }

    pub fn weighted_index(&mut self, items: &Vec<u32>) -> usize {
        let dist = WeightedIndex::new(items).unwrap();

        dist.sample(&mut self.rng)
    }

    pub fn naive_weighted_index(&mut self, items: &Vec<u32>, total: u32) -> usize {
        let need = self.rng.gen_range(1..=total);

        let mut current = 0;

        for i in 0..items.len() {
            current += items[i];

            if current >= need {
                return i;
            }
        }

        0
    }

    pub fn halved_naive_weighted_index(
        &mut self,
        items: &Vec<u32>,
        total: u32,
        half_total: u32,
    ) -> usize {
        let mut need = self.rng.gen_range(1..=total);

        let half = (items.len() - 1) / 2;
        let in_lesser_half = need <= half_total;

        let from = if in_lesser_half { 0 } else { half + 1 };
        let to = if in_lesser_half {
            half
        } else {
            items.len() - 1
        };

        if !in_lesser_half {
            need -= half_total;
        }

        // println!("Need: {:?}, looking in: {:?}", need, range);

        let mut current = 0;
        let mut i = from;

        while i <= to {
            current += items[i];

            if current >= need {
                return i;
            }

            i += 1;
        }

        0
    }
}

fn main() {
    const STEPS: u32 = 1_000_000;
    const INITIAL_CAPITAL: u32 = 1;
    const INCOME: u32 = 1;

    let mut random = RandWeighted::new();
    let mut population: Vec<u32> = Vec::with_capacity(STEPS as usize);

    // Add first person.
    population.push(INITIAL_CAPITAL);

    let mut max = INITIAL_CAPITAL;
    let mut total = INITIAL_CAPITAL;
    let mut half_total = INITIAL_CAPITAL;

    let mut half_cursor = 0;
    let mut half_shifted = false;

    let now = Instant::now();

    for i in 0..(STEPS - 1) {
        total = INITIAL_CAPITAL + i * 2 * INCOME;

        let half = (i / 2) as usize;

        // println!("Population: {:?}", population);
        // println!("Total: {:?}", total);
        // println!("Half total: {:?}", half_total);

        // Pick random human from the population.
        let human = random.halved_naive_weighted_index(&population, total, half_total);

        // Increase theirs capital.
        population[human] += INCOME;

        // Selected human from the first half.
        if human <= half {
            half_total += INCOME;
        }

        // Recalculate the maximum.
        if population[human] > max {
            max = population[human];
        }

        // Also add new human to the population.
        population.push(INITIAL_CAPITAL);

        // Check if new guy added new item to the first half.
        if half_shifted {
            half_cursor += 1;
            half_total += population[half_cursor];
        }

        half_shifted = !half_shifted;
    }

    let elapsed = now.elapsed();

    println!("Duration: {:#?} ms.", elapsed.whole_milliseconds());
    println!("Max: {:#?}", max);
}
