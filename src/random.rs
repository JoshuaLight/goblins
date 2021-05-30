use std::{
    fmt::Debug,
    ops::{Add, AddAssign, SubAssign},
};

use rand::{distributions::uniform::SampleUniform, prelude::*};

use fenwick_tree::FenwickTree;

pub trait Weight:
    Default + Debug + Copy + AddAssign + SubAssign + Add<Output = Self> + PartialOrd + SampleUniform
{
    fn one() -> Self;
}

pub trait WeightedRandom<W: Weight> {
    fn weighted_index<R: RngCore>(&self, rng: &mut R) -> usize;
}

pub struct RngFenwickTree<W: Weight> {
    tree: FenwickTree<W>,
    len: usize,
}

impl<W: Weight> RngFenwickTree<W> {
    pub fn with_capacity(l: usize) -> Self {
        Self {
            tree: FenwickTree::with_len(l),
            len: 0,
        }
    }

    pub fn push(&mut self, x: W) {
        self.tree.add(self.len, x).unwrap();
        self.len += 1;
    }

    pub fn add(&mut self, i: usize, x: W) {
        self.tree.add(i, x).unwrap();
    }
}

impl<W: Weight> WeightedRandom<W> for RngFenwickTree<W> {
    fn weighted_index<R: RngCore>(&self, rng: &mut R) -> usize {
        let total = self.tree.sum(0..self.len).unwrap();

        let mut sum = rng.gen_range(W::one()..=total);
        let mut a = 0;
        let mut b = self.len;

        while a < b - 1 {
            let half = (a + b) / 2;
            let s = self.tree.sum(a..half).unwrap();

            if s < sum {
                sum -= s;
                a = half;
            } else {
                b = half;
            }
        }

        a
    }
}
