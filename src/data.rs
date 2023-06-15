use crate::{codegen::ILGenerator, il::IL};

pub type AtomId = usize;
pub type LinkId = usize;
pub type RuleId = usize;
pub type MembraneId = usize;

#[derive(Debug, Clone, Copy)]
pub enum Symbol {
    Atom(AtomId),
    Link(LinkId),
    Rule(RuleId),
    Membrane(MembraneId),
}

impl Symbol {
    pub fn gen_il(&self, generator: &mut ILGenerator) {
        match self {
            Symbol::Atom(_) => self.gen_atom(generator),
            Symbol::Link(_) => self.gen_link(generator),
            Symbol::Rule(_) => self.gen_rule(generator),
            Symbol::Membrane(_) => self.gen_mem(generator),
        }
    }

    fn gen_atom(&self, generator: &mut ILGenerator) {
        generator.emit(IL::NewAtom(0))
    }

    fn gen_rule(&self, generator: &mut ILGenerator) {}

    fn gen_link(&self, generator: &mut ILGenerator) {}

    fn gen_mem(&self, generator: &mut ILGenerator) {}
}

#[derive(Debug)]
pub struct Rule {
    pub name: String,
    pub pattern: Vec<Symbol>,
    pub guard: Option<Vec<Symbol>>,
    pub body: Vec<Symbol>,
}

#[derive(Debug)]
pub struct Atom {
    pub name: String,
    pub process: Option<Vec<Symbol>>,
}

#[derive(Debug)]
pub struct Link {
    pub name: String,
    pub link1: Option<Symbol>,
    pub link2: Option<Symbol>,
}

#[derive(Debug)]
pub struct Membrane {
    pub name: String,
    pub process: Vec<Symbol>,
}
