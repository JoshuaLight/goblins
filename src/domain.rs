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

    money: WeightVec<isize>,
    alive: WeightVec<isize>,
}

impl<R: RngCore> Model<R> {
    pub fn new(options: ModelOptions<R>) -> Self {
        Model {
            money: WeightVec::with_capacity(options.max_steps),
            alive: WeightVec::with_capacity(options.max_steps),

            options,
        }
    }

    pub fn init(&mut self) {
        self.add_new_human();
    }

    pub fn sim(&mut self) {
        self.sim_income();
        self.sim_birth();
        self.sim_death();
    }

    fn sim_income(&mut self) {
        let lucky = self.options.rng.gen_bool(self.options.p_income);
        if lucky {
            let human = self.money.random_index(&mut self.options.rng);

            self.add_income(human);
        }
    }

    fn sim_birth(&mut self) {
        let born = self.options.rng.gen_bool(self.options.p_birth);
        if born {
            self.add_new_human();
        }
    }

    fn sim_death(&mut self) {
        let died = self.options.rng.gen_bool(self.options.p_death);
        if died {
            let human = self.money.random_index(&mut self.options.rng);

            self.money.reset(human);
            self.alive.reset(human);
        }
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
            alive_count: m.money.vec.iter().filter(|x| x > &&0).count(),
            dead_count: m.money.vec.iter().filter(|x| x == &&0).count(),

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

    pub fn draw(&self) {
        let counter = self.money.iter().collect::<Counter<_>>();
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
