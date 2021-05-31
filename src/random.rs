use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Neg, SubAssign},
};

use rand::{distributions::uniform::SampleUniform, prelude::*};

use fenwick_tree::FenwickTree;

pub trait Weight:
    Default
    + Debug
    + Copy
    + AddAssign
    + SubAssign
    + Add<Output = Self>
    + Neg<Output = Self>
    + PartialOrd
    + SampleUniform
{
    fn one() -> Self;
}

pub trait WeightedRandom<W: Weight> {
    fn weighted_index<R: RngCore>(&self, rng: &mut R) -> usize;
}

pub struct RngFenwickTree<W: Weight> {
    pub tree: FenwickTree<W>,
    len: usize,
}

impl<W: Weight> RngFenwickTree<W> {
    pub fn with_capacity(n: usize) -> Self {
        Self {
            tree: FenwickTree::with_len(n),
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

pub struct WeightVec<W: Weight> {
    pub vec: Vec<W>,
    tree: RngFenwickTree<W>,
}

impl<W: Weight> WeightVec<W> {
    pub fn with_capacity(n: usize) -> Self {
        Self {
            vec: Vec::with_capacity(n),
            tree: RngFenwickTree::with_capacity(n),
        }
    }

    pub fn random_index<R: RngCore>(&self, rng: &mut R) -> usize {
        self.tree.weighted_index(rng)
    }

    pub fn push(&mut self, x: W) {
        self.vec.push(x);
        self.tree.push(x);
    }

    pub fn add(&mut self, i: usize, x: W) {
        self.vec[i] += x;
        self.tree.add(i, x);
    }

    pub fn reset(&mut self, i: usize) {
        let x = self.vec[i];

        // Don't want to lost the value in vec.
        //self.vec[i] = W::default();
        self.tree.add(i, -x);
    }
}
