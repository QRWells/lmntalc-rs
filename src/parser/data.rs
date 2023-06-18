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

// Data structures for rules.

#[derive(Debug)]
pub struct PatternAtom {
    pub name: String,
    pub functors: usize,
}

#[derive(Debug)]
pub struct PatternMembrane {
    pub name: String,
    pub atoms: Vec<PatternAtom>,
    pub mems: Vec<PatternMembrane>,
}

#[derive(Debug)]
pub struct Pattern {
    pub atoms: Vec<PatternAtom>,
    pub mems: Vec<PatternMembrane>,
}

impl Pattern {
    pub fn new() -> Self {
        Pattern {
            atoms: Vec::new(),
            mems: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Rule {
    /// The membrane this rule belongs to.
    pub membrane: MembraneId,
    /// The name of this rule.
    ///
    /// Anonymouse rules are given a generated name.
    pub name: String,
    /// The pattern of this rule.
    pub pattern: Pattern,
    /// The guard of this rule.
    pub guard: Option<Vec<Symbol>>,
    /// The body of this rule.
    pub body: Vec<Symbol>,

    /// Counter for atoms.
    pub atom_id: AtomId,
}

impl Rule {
    pub fn new(membrane: MembraneId, name: String) -> Self {
        Rule {
            membrane,
            name,
            pattern: Pattern::new(),
            guard: None,
            body: Vec::new(),
            atom_id: 0,
        }
    }
}

// Data structures for atoms, links, and membranes.

#[derive(Debug)]
pub struct Atom {
    pub membrane: MembraneId,
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
    pub membrane: MembraneId,
    pub name: String,
    pub process: Vec<Symbol>,
    pub rule_set: Vec<RuleId>,
}
