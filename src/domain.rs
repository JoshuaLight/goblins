use rand::{Rng, RngCore};

use crate::random::{RngFenwickTree, Weight, WeightedRandom};

pub type Currency = isize;

impl Weight for Currency {
    fn one() -> Self {
        1
    }
}

pub struct ModelOptions<R: RngCore> {
    pub max_steps: usize,

    pub initial_capital: Currency,
    pub income: Currency,

    pub rng: R,
    pub p_death: f64,
}

pub struct Model<R: RngCore, W: Weight> {
    options: ModelOptions<R>,

    people: People,
    fenwick: RngFenwickTree<W>,

    pub max_capital: Currency,
    pub died: usize,
    pub mean: Currency,
    pub stdev: f64,
}

impl<R: RngCore> Model<R, Currency> {
    pub fn new(options: ModelOptions<R>) -> Self {
        let max_steps = options.max_steps;

        Model {
            options,

            people: People::with_capacity(max_steps),
            fenwick: RngFenwickTree::with_capacity(max_steps),

            max_capital: 1,
            died: 0,
            mean: 0,
            stdev: 0.0,
        }
    }

    pub fn init(&mut self) {
        self.add_new_human();
    }

    pub fn sim(&mut self) {
        let human = self.fenwick.weighted_index(&mut self.options.rng);

        self.add_income(human);
        self.add_new_human();

        self.sim_death();
    }

    pub fn finish(self) -> Report {
        Report::from_model(self)
    }

    fn add_income(&mut self, human: usize) {
        self.people.add_income(human, self.options.income);
        self.fenwick.add(human, self.options.income)
    }

    fn add_new_human(&mut self) {
        self.people.add_new_human(self.options.initial_capital);
        self.fenwick.push(self.options.initial_capital);
    }

    fn sim_death(&mut self) {
        let someone_died = self.options.rng.gen_bool(self.options.p_death);
        if someone_died {
            let human = self.options.rng.gen_range(0..self.people.count());

            self.kill(human);
            self.died += 1;
        }
    }

    fn kill(&mut self, human: usize) {
        let capital = self.people.money[human];

        self.people.money[human] = 0;
        self.fenwick.add(human, -capital);
    }

    // For debugging.
    fn naive_weighted_index(&mut self) -> usize {
        let total = self.people.money.iter().sum::<Currency>();
        let need = self.options.rng.gen_range(1..=total);

        let mut current = 0;

        for i in 0..self.people.count() {
            current += self.people.money[i];

            if current >= need {
                return i;
            }
        }

        0
    }
}

pub struct People {
    money: Vec<Currency>,
}

impl People {
    pub fn with_capacity(n: usize) -> Self {
        People {
            money: Vec::with_capacity(n),
        }
    }

    pub fn count(&self) -> usize {
        self.money.len()
    }

    pub fn add_new_human(&mut self, capital: Currency) {
        self.money.push(capital);
    }

    pub fn add_income(&mut self, human: usize, income: Currency) {
        self.money[human] += income;
    }
}

pub struct Report {
    alive_count: usize,
    dead_count: usize,

    max: Currency,
    mean: Currency,
    stdev: f64,

    money: Vec<Currency>,
}

impl Report {
    pub fn from_model<R: RngCore, W: Weight>(m: Model<R, W>) -> Self {
        let n = m.people.count();
        let money = &m.people.money;
        let mean = money.iter().sum::<Currency>() / n as Currency;

        Self {
            alive_count: money.iter().map(|x| x > &0).count(),
            dead_count: money.iter().map(|x| x == &0).count(),

            max: *money.iter().max().unwrap_or(&0),
            mean,
            stdev: (money.iter().map(|x| (x - mean).pow(2)).sum::<Currency>() as f64
                / ((n - 1) as f64))
                .sqrt(),

            money: m.people.money,
        }
    }

    pub fn print_verbose(&self) {
        println!("Money: {:?}", self.money);

        self.print();
    }

    pub fn print(&self) {
        println!("Alive: {}", self.alive_count);
        println!("Dead: {}", self.dead_count);
        println!("Max capital: {}", self.max);
        println!("Mean: {}", self.mean);
        println!("Stdev: {}", self.stdev);
    }
}
