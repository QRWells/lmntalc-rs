pub type AtomId = usize;
pub type LinkId = usize;
pub type RuleId = usize;
pub type MembraneId = usize;
pub type ProcContextId = usize;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Symbol {
    Atom(AtomId),
    Link(LinkId),
    Rule(RuleId),
    Membrane(MembraneId),
    ProcContext(ProcContextId),
}

// Data structures for atoms, links, and membranes.

#[derive(Debug)]
pub struct Atom {
    pub membrane: MembraneId,
    pub id: AtomId,
    pub name: String,
    pub process: Option<Vec<Symbol>>,
}

#[derive(Debug)]
pub struct Link {
    pub name: String,
    pub link1: Option<(Symbol, usize)>,
    pub link2: Option<(Symbol, usize)>,
}

#[derive(Debug, Default)]
pub struct Membrane {
    pub membrane: MembraneId,
    pub id: MembraneId,
    pub name: String,
    pub process: Vec<Symbol>,
    pub rule_set: Vec<RuleId>,
}
