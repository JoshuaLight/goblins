use counter::Counter;
use gnuplot::*;
use rand::{Rng, RngCore};

use crate::impl_weight;
use crate::random::{Weight, WeightVec};

impl_weight!(isize);

/// Type of random strategy that is used to choose a goblin.
#[derive(Clone, Copy)]
pub enum RandomStrategy {
    /// A goblin is chosen uniformly from all alive ones.
    Uniform,

    /// A goblin is chosen with its gold as weight from all alive ones.
    Weighted,
}

/// Options of the simulation model.
pub struct ModelOptions<R: RngCore> {
    /// Maximum number of steps the model can simulate.
    /// This is needed for memory pre-allocation.
    pub max_steps: usize,

    /// How much gold a fresh goblin has.
    pub initial_gold: isize,

    /// Function that adds an income to the gold of a goblin.
    pub f_income: Box<dyn Fn(isize) -> isize>,

    /// Random generator.
    pub rng: R,

    /// Random strategy for choosing a lucky goblin that'll receive gold.
    pub rnd_income: RandomStrategy,

    /// Random strategy for choosing a dead goblin.
    pub rnd_death: RandomStrategy,

    /// Probability of a goblin receiving an income.
    pub p_income: f64,

    /// Probability of a goblin becoming alive.
    pub p_birth: f64,

    /// Probability of a goblin dying.
    pub p_death: f64,
}

/// Simulation model of goblins making money.
pub struct Model<R: RngCore> {
    options: ModelOptions<R>,

    alive: WeightVec<isize>,
    gold: WeightVec<isize>,

    dead_count: usize,
}

impl<R: RngCore> Model<R> {
    /// Constructs a new instance of the simulation model using specified `options`.
    pub fn new(options: ModelOptions<R>) -> Self {
        Model {
            alive: WeightVec::with_capacity(options.max_steps),
            gold: WeightVec::with_capacity(options.max_steps),

            options,

            dead_count: 0,
        }
    }

    /// Initializes the model. Gives a life for a new goblin.
    pub fn init(&mut self) {
        self.give_life();
    }

    /// Advances the model by one step of simulation.
    pub fn sim(&mut self) {
        self.sim_income();
        self.sim_birth();
        self.sim_death();
    }

    /// Finishes the simulation and constructs a report.
    pub fn finish(self) -> Report {
        Report::from_model(self)
    }

    /// A random alive goblin receives gold.
    fn sim_income(&mut self) {
        let lucky = self.options.rng.gen_bool(self.options.p_income);
        if lucky {
            let goblin = self.random_goblin(self.options.rnd_income);
            if let Some(goblin) = goblin {
                self.add_income(goblin);
            }
        }
    }

    /// A new goblin is born.
    fn sim_birth(&mut self) {
        let born = self.options.rng.gen_bool(self.options.p_birth);
        if born {
            self.give_life();
        }
    }

    /// A random alive goblin dies.
    fn sim_death(&mut self) {
        let died = self.options.rng.gen_bool(self.options.p_death);
        if died {
            let goblin = self.random_goblin(self.options.rnd_death);
            if let Some(goblin) = goblin {
                self.kill(goblin);
            }
        }
    }

    /// Gives life for a goblin.
    fn give_life(&mut self) {
        self.alive.push(1);
        self.gold.push(self.options.initial_gold);
    }

    /// Adds income to the `goblin`.
    fn add_income(&mut self, goblin: usize) {
        let current = self.gold.vec[goblin];
        let next = (self.options.f_income)(current);

        self.gold.add(goblin, next - current);
    }

    /// Kills the `goblin`.
    fn kill(&mut self, goblin: usize) {
        self.alive.reset(goblin);
        self.gold.reset(goblin);

        self.dead_count += 1;
    }

    /// A random goblin chosen using a random type `t`.
    fn random_goblin(&mut self, t: RandomStrategy) -> Option<usize> {
        match t {
            RandomStrategy::Uniform => self.alive.random_index(&mut self.options.rng),
            RandomStrategy::Weighted => self.gold.random_index(&mut self.options.rng),
        }
    }
}

/// Report with the results of simulating the model.
pub struct Report {
    /// How many goblins are still alive.
    alive_count: usize,

    /// How many goblins are dead.
    dead_count: usize,

    /// Gold of all goblins in the population.
    /// Note: there are no zeroes even for dead goblins.
    gold: Vec<isize>,
}

impl Report {
    /// Constructs a new simulation report from the model `m`.
    pub fn from_model<R: RngCore>(m: Model<R>) -> Self {
        let gold = m.gold.vec;

        Self {
            alive_count: gold.len() - m.dead_count,
            dead_count: m.dead_count,

            gold,
        }
    }

    /// Prints the report into the console.
    pub fn print(&self) {
        println!("Alive: {}", self.alive_count);
        println!("Dead: {}", self.dead_count);
    }

    /// Draws the report into the `gnuplot` axes.
    pub fn draw_to(&self, fg: &mut Axes2D, caption: &str) {
        let counter = self.gold.iter().collect::<Counter<_>>();
        let x: Vec<isize> = counter.keys().map(|x| **x).collect();
        let y: Vec<isize> = counter.values().map(|x| *x as isize).collect();

        fg.points(&x, &y, &[PointSymbol('O'), Caption(caption)])
            .set_x_log(Some(10f64))
            .set_y_log(Some(10f64));
    }
}
