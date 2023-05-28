use crate::il::IL;

pub enum Target {
    Text,
    Binary,
}

struct ILGenerator {
    il: Vec<IL>,
}

impl ILGenerator {
    pub fn emit(&mut self, il: IL) {
        self.il.push(il);
    }

    pub fn write_to(&self, path: &str, target: Target) {
        todo!("ILGenerator::write_to")
    }
}
