use std::{
    fmt::Debug,
    ops::{AddAssign, SubAssign},
};
use time::Instant;

use fenwick_tree::FenwickTree;

use rand::{distributions::uniform::SampleUniform, prelude::*};
use rand_xoshiro::SplitMix64;

trait Increment {
    fn increment(self) -> Self;
}

impl Increment for isize {
    fn increment(self) -> Self {
        self + 1
    }
}

struct Person {
    capital: usize,
    active_capital: usize,
}

impl Person {
    pub fn new(initial_capital: usize) -> Self {
        Self {
            capital: initial_capital,
            active_capital: initial_capital,
        }
    }

    pub fn add_capital(&mut self, value: usize) {
        self.capital += value;
        self.active_capital += value;
    }

    pub fn make_dead(&mut self) {
        self.active_capital = 0;
    }
}

trait WeightedRandom<T> {
    fn weighted_index<R: RngCore>(&self, rng: &mut R, len: usize, total: T) -> usize;
}

impl<T> WeightedRandom<T> for FenwickTree<T>
where
    T: Default
        + Debug
        + Copy
        + AddAssign
        + SubAssign
        + SampleUniform
        + PartialOrd
        + std::ops::Add<Output = T>
        + Increment,
{
    fn weighted_index<R: RngCore>(&self, rng: &mut R, len: usize, total: T) -> usize {
        let mut need = rng.gen_range(T::default().increment()..=total);
        let mut a = 0;
        let mut b = len;

        while a != b - 1 {
            let half = (a + b) / 2;
            let sum = self.sum(a..half).unwrap();

            if sum < need {
                need -= sum;
                a = half;
            } else {
                b = half;
            }
        }

        a
    }
}

fn main() {
    const STEPS: usize = 1_000_000;
    const INITIAL_CAPITAL: usize = 1;
    const INCOME: usize = 1;
    const PROBABILITY_OF_DEATH: f64 = 0.1;

    let mut rng = SplitMix64::seed_from_u64(1);
    let mut population: Vec<Person> = Vec::with_capacity(STEPS as usize);
    let mut fenwick = FenwickTree::<isize>::with_len(STEPS as usize);

    // Add first person.
    population.push(Person::new(INITIAL_CAPITAL));
    fenwick.add(0, INITIAL_CAPITAL as isize).unwrap();

    let mut max = INITIAL_CAPITAL;

    let now = Instant::now();

    for i in 0..(STEPS - 1) {
        let total = INITIAL_CAPITAL + (i * 2 * INCOME);

        // Pick random human from the population.
        let human = fenwick.weighted_index(&mut rng, population.len(), total as isize) as usize;

        // Increase theirs capital.
        population[human].add_capital(INCOME);
        fenwick.add(human, INCOME as isize).unwrap();

        // Also add new human to the population.
        population.push(Person::new(INITIAL_CAPITAL));
        fenwick
            .add(population.len() - 1, INITIAL_CAPITAL as isize)
            .unwrap();

        // Someone died.
        let died = rng.gen_bool(PROBABILITY_OF_DEATH);
        if died {
            let dead_guy_index = rng.gen_range(0..population.len());
            let dead_guy = &mut population[dead_guy_index];

            let capital = dead_guy.active_capital as i32;

            dead_guy.make_dead();
            fenwick.add(dead_guy_index, -capital as isize).unwrap();
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
