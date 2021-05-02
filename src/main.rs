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

    pub fn fenwick_weighted_index(
        &mut self,
        items: &Vec<u32>,
        total: u32,
        fenwick: &FenwickTree,
    ) -> usize {
        let need = self.rng.gen_range(1..=total);

        self.fenwick_weighted_index_range(items, 0, items.len() - 1, need, fenwick)
    }

    pub fn fenwick_weighted_index_range(
        &mut self,
        items: &Vec<u32>,
        from: usize,
        to: usize,
        need: u32,
        fenwick: &FenwickTree,
    ) -> usize {
        let count = to - from + 1;

        // [x] case: `to` == `from`.
        if count == 1 {
            return from; // or `to`.
        }

        // [x, y] case: `y` if `need` is greater than `x`, `x` otherwise.
        if count == 2 {
            if need > items[from] {
                return to;
            } else {
                return from;
            }
        }

        // `[x, ..., z]` case: divide vector by half.
        let half = from + (to - from) / 2;
        let half_sum = fenwick.sum(from, half);

        if need <= half_sum {
            self.fenwick_weighted_index_range(items, from, half, need, fenwick)
        } else {
            self.fenwick_weighted_index_range(items, half, to, need, fenwick)
        }
    }
}

struct FenwickTree {
    tree: Vec<u32>,
}

impl FenwickTree {
    pub fn new(size: usize) -> Self {
        Self {
            tree: vec![0; size],
        }
    }

    pub fn increase(&mut self, index: usize, delta: u32) {
        let mut i = index;

        while i < self.tree.len() {
            self.tree[i] += delta;
            i |= i + 1;
        }
    }

    pub fn sum(&self, from: usize, to: usize) -> u32 {
        let sum_from = if from == 0 { 0 } else { self.sum_at(from - 1) };
        let sum_to = self.sum_at(to);

        sum_to - sum_from
    }

    fn sum_at(&self, index: usize) -> u32 {
        let mut sum = 0;
        let mut i = index as i32;

        while i >= 0 {
            sum += self.tree[i as usize];

            i &= i + 1;
            i -= 1;
        }

        sum
    }
}

fn main() {
    const STEPS: u32 = 1_000_000;
    const INITIAL_CAPITAL: u32 = 1;
    const INCOME: u32 = 1;

    let mut random = RandWeighted::new();
    let mut population: Vec<u32> = Vec::with_capacity(STEPS as usize);
    let mut fenwick = FenwickTree::new(STEPS as usize);

    // Add first person.
    population.push(INITIAL_CAPITAL);
    fenwick.increase(0, INITIAL_CAPITAL);

    let mut max = INITIAL_CAPITAL;

    let now = Instant::now();

    for i in 0..(STEPS - 1) {
        let total = INITIAL_CAPITAL + i * 2 * INCOME;

        // Pick random human from the population.
        // 1. Naive.
        // let human = random.naive_weighted_index(&population, total);

        // 2. Fenwick.
        let human = random.fenwick_weighted_index(&population, total, &fenwick);

        // Increase theirs capital.
        population[human] += INCOME;
        fenwick.increase(human, INCOME);

        // Recalculate the maximum.
        if population[human] > max {
            max = population[human];
        }

        // Also add new human to the population.
        population.push(INITIAL_CAPITAL);
        fenwick.increase(population.len() - 1, INITIAL_CAPITAL);
    }

    let elapsed = now.elapsed();

    println!("Duration: {:#?} ms.", elapsed.whole_milliseconds());
    println!("Max: {:#?}", max);
}
