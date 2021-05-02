use plotters::prelude::*;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand_xoshiro::SplitMix64;
use time::Instant;

trait Weight {
    fn weight(&self) -> u32;
}

struct Person {
    capital: u32,
    active_capital: u32,
}

impl Person {
    pub fn new(initial_capital: u32) -> Self {
        Self {
            capital: initial_capital,
            active_capital: initial_capital,
        }
    }

    pub fn add_capital(&mut self, value: u32) {
        self.capital += value;
        self.active_capital += value;
    }

    pub fn make_dead(&mut self) {
        self.active_capital = 0;
    }
}

impl Weight for Person {
    fn weight(&self) -> u32 {
        self.active_capital
    }
}

struct RandWeighted {
    rng: SplitMix64,
}

impl RandWeighted {
    pub fn new() -> Self {
        Self {
            rng: SplitMix64::seed_from_u64(1),
        }
    }

    pub fn true_with(&mut self, p: f64) -> bool {
        self.rng.gen_bool(p)
    }

    pub fn uniform_index<T>(&mut self, items: &Vec<T>) -> usize {
        self.rng.gen_range(0..items.len())
    }

    pub fn weighted_index<W: Weight>(&mut self, items: &Vec<W>) -> usize {
        let weights = items.iter().map(|x| x.weight());
        let dist = WeightedIndex::new(weights).unwrap();

        dist.sample(&mut self.rng)
    }

    pub fn naive_weighted_index<W: Weight>(&mut self, items: &Vec<W>, total: u32) -> usize {
        let need = self.rng.gen_range(1..=total);

        let mut current = 0;

        for i in 0..items.len() {
            current += items[i].weight();

            if current >= need {
                return i;
            }
        }

        0
    }

    pub fn fenwick_weighted_index<W: Weight>(
        &mut self,
        items: &Vec<W>,
        total: u32,
        fenwick: &FenwickTree,
    ) -> usize {
        let need = self.rng.gen_range(1..=total);

        self.fenwick_weighted_index_range(items, 0, items.len() - 1, need, fenwick)
    }

    pub fn fenwick_weighted_index_range<W: Weight>(
        &mut self,
        items: &Vec<W>,
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
            // TODO: In order for items to be removed, here we need to check weight.
            if need > items[from].weight() {
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

    pub fn increase(&mut self, index: usize, delta: i32) {
        let mut i = index;

        while i < self.tree.len() {
            let current = self.tree[i] as i32;
            let current = current + delta;

            self.tree[i] = current as u32;
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
    const PROBABILITY_OF_DEATH: f64 = 0.1;

    let mut random = RandWeighted::new();
    let mut population: Vec<Person> = Vec::with_capacity(STEPS as usize);
    let mut fenwick = FenwickTree::new(STEPS as usize);

    // Add first person.
    population.push(Person::new(INITIAL_CAPITAL));
    fenwick.increase(0, INITIAL_CAPITAL as i32);

    let mut max = INITIAL_CAPITAL;

    let now = Instant::now();

    for i in 0..(STEPS - 1) {
        let total = INITIAL_CAPITAL + i * 2 * INCOME;

        // Pick random human from the population.
        // 1. Naive (O(n^2)).
        // let human = random.naive_weighted_index(&population, total);

        // 2. Fenwick (O(n logn)).
        let human = random.fenwick_weighted_index(&population, total, &fenwick);

        // Increase theirs capital.
        population[human].add_capital(INCOME);
        fenwick.increase(human, INCOME as i32);

        // Also add new human to the population.
        population.push(Person::new(INITIAL_CAPITAL));
        fenwick.increase(population.len() - 1, INITIAL_CAPITAL as i32);

        // Someone died.
        let died = random.true_with(PROBABILITY_OF_DEATH);
        if died {
            let dead_guy_index = random.uniform_index(&population);
            let dead_guy = &mut population[dead_guy_index];

            let capital = dead_guy.active_capital as i32;

            dead_guy.make_dead();
            fenwick.increase(dead_guy_index, -capital);
        }

        // Recalculate the maximum.
        if population[human].active_capital > max {
            max = population[human].active_capital;
        }
    }

    let dead = population.iter().filter(|x| x.active_capital == 0);

    let elapsed = now.elapsed();

    println!("---");
    println!("Population size: {:?}", population.len());
    println!("Died: {:?}", dead.count());
    println!("Max capital: {:#?}", max);
    println!("---");
    println!("Duration: {:#?} ms.", elapsed.whole_milliseconds());
}
