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
        }
    }

    #[inline]
    pub fn init(&mut self) {
        self.add_new_human();
    }

    #[inline]
    pub fn sim(&mut self) {
        let human = self.fenwick.weighted_index(&mut self.options.rng);

        self.add_income(human);
        self.add_new_human();

        self.sim_death();
    }

    #[inline]
    pub fn finish(&mut self) {
        self.recalculate_max();
    }

    #[inline]
    fn add_income(&mut self, human: usize) {
        self.people.add_income(human, self.options.income);
        self.fenwick.add(human, self.options.income)
    }

    #[inline]
    fn add_new_human(&mut self) {
        self.people.add_new_human(self.options.initial_capital);
        self.fenwick.push(self.options.initial_capital);
    }

    #[inline]
    fn sim_death(&mut self) {
        let someone_died = self.options.rng.gen_bool(self.options.p_death);
        if someone_died {
            let human = self.options.rng.gen_range(0..self.people.count());

            self.people.money[human] = 0;
            self.died += 1;
        }
    }

    #[inline]
    fn recalculate_max(&mut self) {
        self.max_capital = *self.people.money.iter().max().unwrap_or(&0);
    }
}

pub struct People {
    money: Vec<Currency>,
}

impl People {
    #[inline]
    pub fn with_capacity(n: usize) -> Self {
        People {
            money: Vec::with_capacity(n),
        }
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.money.len()
    }

    #[inline]
    pub fn add_new_human(&mut self, capital: Currency) {
        self.money.push(capital);
    }

    #[inline]
    pub fn add_income(&mut self, human: usize, income: Currency) {
        self.money[human] += income;
    }
}
