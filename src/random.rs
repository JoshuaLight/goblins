use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Neg, SubAssign},
};

use rand::{distributions::uniform::SampleUniform, prelude::*};

use fenwick_tree::FenwickTree;

/// A weight in a weighted random algorithm.
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
    /// Returns an identity weight that can be used as a default "working" weight,
    /// considering that `W::default()` returns zero.
    fn identity() -> Self;
}

/// Implements the `Weight` trait for a given type.
#[macro_export]
macro_rules! impl_weight {
    ($t:ty) => {
        impl Weight for $t {
            fn identity() -> Self {
                1
            }
        }
    };
}

/// A collection of weights that is able to sample a random weighted index.
pub trait Weights<W: Weight> {
    /// Samples a weighted index from the collection using the random generator `rng`.
    fn weighted_index<R: RngCore>(&self, rng: &mut R) -> Option<usize>;
}

/// A Fenwick tree with a separate `len` field (the tree's one acts like capacity).
///
/// Note: this tree is suitable for implementing the `Weights` trait.
pub struct RngFenwickTree<W: Weight> {
    tree: FenwickTree<W>,
    len: usize,
}

impl<W: Weight> RngFenwickTree<W> {
    /// Constructs a new instance of `RngFenwickTree` with the capacity `n`.
    pub fn with_capacity(n: usize) -> Self {
        Self {
            tree: FenwickTree::with_len(n),
            len: 0,
        }
    }

    /// Pushes a new weight `x` to the tree.
    pub fn push(&mut self, x: W) {
        self.tree.add(self.len, x).unwrap();
        self.len += 1;
    }

    /// Adds the delta `x` at the index `i` in the tree.
    pub fn add(&mut self, i: usize, x: W) {
        self.tree.add(i, x).unwrap();
    }
}

impl<W: Weight> Weights<W> for RngFenwickTree<W> {
    fn weighted_index<R: RngCore>(&self, rng: &mut R) -> Option<usize> {
        let total = self.tree.sum(0..self.len).unwrap();
        if total == W::default() {
            return None;
        }

        let mut sum = rng.gen_range(W::identity()..=total);
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

        Some(a)
    }
}

/// A data structure that combines regular vector of values with Fenwick tree
/// that is used to sample random indices from this vector.
pub struct WeightVec<W: Weight> {
    pub vec: Vec<W>,
    tree: RngFenwickTree<W>,
}

impl<W: Weight> WeightVec<W> {
    /// Constructs a new instance of `WeightVec` with the capacity `n`.
    pub fn with_capacity(n: usize) -> Self {
        Self {
            vec: Vec::with_capacity(n),
            tree: RngFenwickTree::with_capacity(n),
        }
    }

    /// Samples a random index using the random generator `rng`.
    pub fn random_index<R: RngCore>(&self, rng: &mut R) -> Option<usize> {
        self.tree.weighted_index(rng)
    }

    /// Pushes a new value `x`.
    pub fn push(&mut self, x: W) {
        self.vec.push(x);
        self.tree.push(x);
    }

    /// Adds the delta `x` at the index `i`.
    pub fn add(&mut self, i: usize, x: W) {
        self.vec[i] += x;
        self.tree.add(i, x);
    }

    /// Sets the value at the index `i` to zero,
    /// making it imossible to retrieve an item with this index from the `random_index`.
    pub fn reset(&mut self, i: usize) {
        self.tree.add(i, -self.vec[i]);
    }
}
