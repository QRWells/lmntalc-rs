use super::Optimizer;

#[derive(Debug, Default)]
struct RuleOptimizer {
    order: i32,
}

impl Optimizer for RuleOptimizer {
    fn optimize(&self, il: &mut crate::codegen::ILGenerator) {
        todo!()
    }

    fn pass(&self) -> u8 {
        0
    }

    fn level(&self) -> u8 {
        0
    }

    fn order(&self) -> i32 {
        self.order
    }

    fn set_order(&mut self, order: i32) {
        self.order = order;
    }
}
