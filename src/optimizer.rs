pub mod rule_optimizer;

use std::{collections::BTreeSet, fmt::Debug};

use crate::codegen::ILGenerator;

pub trait Optimizer: Debug {
    /// Optimize the given IL.
    fn optimize(&self, il: &mut ILGenerator);

    /// Unique ID of this optimizer. Used to determine whether two optimizers are the same.
    fn uid(&self) -> u32 {
        rand::random()
    }

    /// The total number of times this optimizer is executed.
    ///
    /// For example, if this optimizer is executed in the first pass, return `1`.
    /// If this optimizer is executed in first two passes, return `2`.
    ///
    /// Here, if this optimizer is executed in every possible pass, return `0`.
    fn pass(&self) -> u8;

    /// Enable this optimizer only when the optimization level is greater than or equal to this value.
    fn level(&self) -> u8;

    /// The order in which this optimizer is executed.
    fn order(&self) -> i32;

    /// Set the order in which this optimizer is executed.
    fn set_order(&mut self, order: i32);
}

impl Eq for Box<dyn Optimizer> {}

impl PartialEq for Box<dyn Optimizer> {
    fn eq(&self, other: &Self) -> bool {
        self.uid() == other.uid()
    }
}

impl PartialOrd for Box<dyn Optimizer> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.order().partial_cmp(&other.order())
    }
}

impl Ord for Box<dyn Optimizer> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order().cmp(&other.order())
    }
}

#[derive(Debug)]
pub struct OptimizerManager {
    level: u8,
    pass: u8,
    optimizers: BTreeSet<Box<dyn Optimizer>>,
}

impl OptimizerManager {
    pub fn new(level: u8) -> Self {
        OptimizerManager {
            level,
            pass: 1,
            optimizers: BTreeSet::new(),
        }
    }

    pub fn optimize(&self, il: &mut ILGenerator) {
        for i in 1..=self.pass {
            for optimizer in self.optimizers.iter() {
                if optimizer.pass() >= i && optimizer.level() <= self.level {
                    optimizer.optimize(il);
                }
            }
        }
    }

    pub fn add_optimizer(&mut self, optimizer: Box<dyn Optimizer>) {
        self.pass = u8::max(self.pass, optimizer.pass());
        self.optimizers.insert(optimizer);
    }
}
