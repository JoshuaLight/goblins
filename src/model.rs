use counter::Counter;
use gnuplot::*;
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

    pub p_income: f64,
    pub p_birth: f64,
    pub p_death: f64,
}

pub struct Model<R: RngCore> {
    options: ModelOptions<R>,

    gold: WeightVec<isize>,
    age: WeightVec<isize>,

    dead_count: usize,
}

impl<R: RngCore> Model<R> {
    pub fn new(options: ModelOptions<R>) -> Self {
        Model {
            gold: WeightVec::with_capacity(options.max_steps),
            age: WeightVec::with_capacity(options.max_steps),

            options,

            dead_count: 0,
        }
    }

    pub fn init(&mut self) {
        self.add_new_goblin();
    }

    pub fn sim(&mut self) {
        self.sim_income();
        self.sim_birth();
        self.sim_death();
        self.sim_ageing();
    }

    fn sim_income(&mut self) {
        let lucky = self.options.rng.gen_bool(self.options.p_income);
        if lucky {
            let goblin = self.gold.random_index(&mut self.options.rng);
            if let Some(goblin) = goblin {
                self.add_income(goblin);
            }
        }
    }

    fn sim_birth(&mut self) {
        let born = self.options.rng.gen_bool(self.options.p_birth);
        if born {
            self.add_new_goblin();
        }
    }

    fn sim_death(&mut self) {
        let died = self.options.rng.gen_bool(self.options.p_death);
        if died {
            let goblin = self.age.random_index(&mut self.options.rng);
            if let Some(goblin) = goblin {
                self.gold.reset(goblin);
                self.age.reset(goblin);

                self.dead_count += 1;
            }
        }
    }

    fn sim_ageing(&mut self) {
        for i in 0..self.age.vec.len() {
            self.age.add(i, 1);
        }
    }

    pub fn finish(self) -> Report {
        Report::from_model(self)
    }

    fn add_new_goblin(&mut self) {
        self.gold.push(self.options.initial_capital);
        self.age.push(1);
    }

    fn add_income(&mut self, goblin: usize) {
        self.gold.add(goblin, self.options.income);
    }
}

pub struct Report {
    alive_count: usize,
    dead_count: usize,

    max: isize,
    mean: isize,
    stdev: f64,

    gold: Vec<isize>,
}

impl Report {
    pub fn from_model<R: RngCore>(m: Model<R>) -> Self {
        let gold = &m.gold.vec;
        let n = gold.len();
        let mean = gold.iter().sum::<isize>() / n as isize;

        Self {
            alive_count: n - m.dead_count,
            dead_count: m.dead_count,

            max: *gold.iter().max().unwrap_or(&0),
            mean,
            stdev: (gold.iter().map(|x| (x - mean).pow(2)).sum::<isize>() as f64
                / ((n - 1) as f64))
                .sqrt(),

            gold: m.gold.vec,
        }
    }

    pub fn print(&self) {
        println!("Alive: {}", self.alive_count);
        println!("Dead: {}", self.dead_count);
        println!("Max capital: {}", self.max);
        println!("Mean: {}", self.mean);
        println!("Stdev: {}", self.stdev);
    }

    pub fn draw(&self) {
        let counter = self.gold.iter().collect::<Counter<_>>();
        let x: Vec<isize> = counter.keys().map(|x| **x).collect();
        let y: Vec<isize> = counter.values().map(|x| *x as isize).collect();

        let mut fg = Figure::new();

        fg.axes2d()
            .points(&x, &y, &[PointSymbol('O')])
            .set_size(0.5, 0.5)
            .set_pos(0.25, 0.25)
            .set_x_log(Some(10f64))
            .set_y_log(Some(10f64));
        fg.show().unwrap();
    }
}
