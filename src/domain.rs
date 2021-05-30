use rand::RngCore;

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
}

impl<R: RngCore> Model<R, Currency> {
    pub fn new(options: ModelOptions<R>) -> Self {
        let max_steps = options.max_steps;

        Model {
            options,

            people: People::with_capacity(max_steps),
            fenwick: RngFenwickTree::with_capacity(max_steps),

            max_capital: 1,
        }
    }

    pub fn initialize(&mut self) {
        self.add_new_human();
    }

    pub fn simulate(&mut self) {
        let human = self.fenwick.weighted_index(&mut self.options.rng);

        self.add_income(human);

        if self.people.money[human] > self.max_capital {
            self.max_capital = self.people.money[human];
        }

        self.add_new_human();
    }

    fn add_income(&mut self, human: usize) {
        self.people.add_income(human, self.options.income);
        self.fenwick.add(human, self.options.income)
    }

    fn add_new_human(&mut self) {
        self.people.add_new_human(self.options.initial_capital);
        self.fenwick.push(self.options.initial_capital);
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

    pub fn add_new_human(&mut self, capital: Currency) {
        self.money.push(capital);
    }

    pub fn add_income(&mut self, human: usize, income: Currency) {
        self.money[human] += income;
    }
}
