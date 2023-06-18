use self::il::IL;

pub mod il;
mod rule_gen;

pub enum Target {
    Text,
    Binary,
}

#[derive(Debug, Default)]
pub struct ILGenerator {
    il: Vec<IL>,
}

impl ILGenerator {
    pub fn emit(&mut self, il: IL) {
        self.il.push(il);
    }

    pub fn write_to(&self, path: &str, target: Target) {
        todo!("ILGenerator::write_to")
    }

    fn binary_header(&self) -> Vec<u8> {
        todo!("ILGenerator::binary_header")
    }
}
