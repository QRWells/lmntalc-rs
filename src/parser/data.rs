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

impl From<Symbol> for usize {
    fn from(val: Symbol) -> Self {
        match val {
            Symbol::Atom(id) => id,
            Symbol::Link(id) => id,
            Symbol::Rule(id) => id,
            Symbol::Membrane(id) => id,
            Symbol::ProcContext(id) => id,
        }
    }
}

impl Symbol {
    pub fn compare(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Symbol::Atom(id1), Symbol::Atom(id2)) => id1.cmp(id2),
            (Symbol::Atom(_), _) => std::cmp::Ordering::Less,
            (Symbol::Membrane(id1), Symbol::Membrane(id2)) => id1.cmp(id2),
            (Symbol::Membrane(_), Symbol::Atom(_)) => std::cmp::Ordering::Greater,
            (Symbol::Membrane(_), _) => std::cmp::Ordering::Less,
            (Symbol::Link(id1), Symbol::Link(id2)) => id1.cmp(id2),
            (Symbol::Link(_), Symbol::Atom(_)) => std::cmp::Ordering::Greater,
            (Symbol::Link(_), Symbol::Membrane(_)) => std::cmp::Ordering::Greater,
            (Symbol::Link(_), _) => std::cmp::Ordering::Less,
            (Symbol::Rule(id1), Symbol::Rule(id2)) => id1.cmp(id2),
            (Symbol::Rule(_), Symbol::Atom(_)) => std::cmp::Ordering::Greater,
            (Symbol::Rule(_), Symbol::Link(_)) => std::cmp::Ordering::Greater,
            (Symbol::Rule(_), Symbol::Membrane(_)) => std::cmp::Ordering::Greater,
            (Symbol::Rule(_), Symbol::ProcContext(_)) => std::cmp::Ordering::Less,
            (Symbol::ProcContext(id1), Symbol::ProcContext(id2)) => id1.cmp(id2),
            (Symbol::ProcContext(_), _) => std::cmp::Ordering::Greater,
        }
    }
}

// Data structures for atoms, links, and membranes.

#[derive(Debug)]
pub struct Atom {
    pub membrane: MembraneId,
    pub id: AtomId,
    pub name: String,
    pub links: Vec<Symbol>,
}

#[derive(Debug)]
pub struct Link {
    pub name: String,
    pub link1: Option<(Symbol, usize)>,
    pub link2: Option<(Symbol, usize)>,
    pub pos1: Option<usize>,
    pub pos2: Option<usize>,
}

#[derive(Debug, Default)]
pub struct Membrane {
    pub membrane: MembraneId,
    pub id: MembraneId,
    pub name: String,
    pub process: Vec<Symbol>,
    pub rule_set: Vec<RuleId>,
}
