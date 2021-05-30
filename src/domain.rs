use rand::{Rng, RngCore};

use crate::random::{Weight, WeightVec};

impl Weight for isize {
    fn one() -> Self {
        1
    }
}

pub struct ModelOptions<R: RngCore> {
    pub max_steps: usize,

    pub initial_capital: isize,
    pub income: isize,

    pub rng: R,
    pub p_death: f64,
}

pub struct Model<R: RngCore> {
    options: ModelOptions<R>,

    money: WeightVec<isize>,
    alive: WeightVec<isize>,
}

impl<R: RngCore> Model<R> {
    pub fn new(options: ModelOptions<R>) -> Self {
        let max_steps = options.max_steps;

        Model {
            options,

            money: WeightVec::with_capacity(max_steps),
            alive: WeightVec::with_capacity(max_steps),
        }
    }

    pub fn init(&mut self) {
        self.add_new_human();
    }

    pub fn sim(&mut self) {
        let human = self.money.random_index(&mut self.options.rng);

        self.add_income(human);
        self.add_new_human();

        self.sim_death();
    }

    pub fn finish(self) -> Report {
        Report::from_model(self)
    }

    fn add_new_human(&mut self) {
        self.money.push(self.options.initial_capital);
        self.alive.push(1);
    }

    fn add_income(&mut self, human: usize) {
        self.money.add(human, self.options.income);
    }

    fn sim_death(&mut self) {
        let someone_died = self.options.rng.gen_bool(self.options.p_death);
        if someone_died {
            let human = self.alive.random_index(&mut self.options.rng);

            self.money.reset(human);
            self.alive.reset(human);
        }
    }
}

pub struct Report {
    alive_count: usize,
    dead_count: usize,

    max: isize,
    mean: isize,
    stdev: f64,

    money: Vec<isize>,
}

impl Report {
    pub fn from_model<R: RngCore>(m: Model<R>) -> Self {
        let money = &m.money.vec;
        let n = money.len();
        let mean = money.iter().sum::<isize>() / n as isize;

        Self {
            alive_count: money.iter().filter(|x| x > &&0).count(),
            dead_count: money.iter().filter(|x| x == &&0).count(),

            max: *money.iter().max().unwrap_or(&0),
            mean,
            stdev: (money.iter().map(|x| (x - mean).pow(2)).sum::<isize>() as f64
                / ((n - 1) as f64))
                .sqrt(),

            money: m.money.vec,
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
