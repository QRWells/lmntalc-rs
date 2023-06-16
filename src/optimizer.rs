use std::fmt::Debug;

use crate::codegen::ILGenerator;

pub trait Optimizer: Debug {
    fn optimize(&self, il: &mut ILGenerator);

    fn level(&self) -> u8;
    fn set_level(&mut self, level: u8);

    fn order(&self) -> i32;
    fn set_order(&mut self, order: i32);
}

#[derive(Debug, Default)]
pub struct OptimizerManager {
    level: u8,
    optimizers: Vec<Box<dyn Optimizer>>,
}

impl OptimizerManager {
    pub fn optimize(&self, il: &mut ILGenerator) {
        
    }

    pub fn add_optimizer(&mut self, optimizer: Box<dyn Optimizer>) {
        self.optimizers.push(optimizer);
        // sort by order
    }
}
