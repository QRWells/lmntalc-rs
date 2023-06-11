use std::collections::LinkedList;

pub enum SymbolType {
    Atom,
    Link,
    Rule,
    Membrane,
    Process,
}

pub struct SymbolId {
    symbol_type: SymbolType,
    id: usize,
}

pub struct  Process {
    name: String,
    symbol: SymbolId,
}

pub struct Program {}
pub struct Rule {
    name: String,
    head: LinkedList<Atom>,
}
pub struct Atom {
    name: String,
    process: Option<LinkedList<SymbolId>>,
}
pub struct Link {
    name: String,
}
pub struct Memebrane {}
