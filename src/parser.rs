pub mod data;
pub mod rule_parser;

use once_cell::sync::OnceCell;
use pest::{Parser, Position};
use std::collections::HashMap;

use self::{data::*, rule_parser::parse_rule};

#[derive(Parser)]
#[grammar = "lmntal.pest"]
pub(crate) struct LMNParser;

pub type ParseRule = Rule;

static mut LINK_ID: LinkId = 0;
static mut RULE_ID: RuleId = 0;
pub(crate) static mut ENTITY_ID: MembraneId = 0;

pub static mut ATOMS: OnceCell<HashMap<AtomId, Atom>> = OnceCell::new();
pub static mut LINKS: OnceCell<HashMap<LinkId, Link>> = OnceCell::new();
pub static mut RULES: OnceCell<HashMap<RuleId, rule_parser::Rule>> = OnceCell::new();
pub static mut MEMS: OnceCell<HashMap<MembraneId, Membrane>> = OnceCell::new();

#[derive(Debug, Clone, Copy)]
pub struct Context {
    /// From which symbol this symbol is generated.
    from: Symbol,
    /// Valid only when `from` is `Some(Symbol::Atom)` or `Some(Symbol::Membrane)`.
    pos: Option<usize>,
    /// The membrane in which this symbol is generated.
    membrane: MembraneId,
}

pub fn parse_lmntal(file: &str) -> Result<Symbol, Box<pest::error::Error<Rule>>> {
    let pairs = LMNParser::parse(Rule::Program, file)?;
    let mut init_process = Vec::new();
    let id = unsafe { ENTITY_ID };
    unsafe { ENTITY_ID += 1 };
    let ctx = Context {
        from: Symbol::Membrane(id),
        pos: None,
        membrane: id,
    };
    for pair in pairs {
        match pair.as_rule() {
            Rule::Program => {
                for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::WorldProcessList => {
                            init_process.append(&mut parse_world_process_list(pair, ctx));
                        }
                        Rule::EOI => {}
                        _ => {
                            unreachable!("Unexpected rule: {:?}", pair.as_rule());
                        }
                    }
                }
            }
            _ => {
                unreachable!("Unexpected rule: {:?}", pair.as_rule());
            }
        }
    }

    unsafe {
        let links = LINKS.get_or_init(HashMap::new);
        for link in links.values() {
            if link.link2.is_none() {
                return Err(Box::new(pest::error::Error::new_from_pos(
                    pest::error::ErrorVariant::CustomError {
                        message: "Free link is not allowed here.".to_string(),
                    },
                    Position::new(file, link.pos1.unwrap()).unwrap(),
                )));
            }
        }
    }

    let mut rule_set = vec![];
    for symbol in init_process.iter() {
        if let Symbol::Rule(id) = symbol {
            rule_set.push(*id);
        }
    }
    init_process.retain(|symbol| !matches!(symbol, Symbol::Rule(_)));

    unsafe {
        MEMS.get_or_init(HashMap::new);
        MEMS.get_mut().unwrap().insert(
            id,
            Membrane {
                membrane: MembraneId::MAX,
                id,
                name: "init".to_string(),
                process: init_process,
                rule_set,
            },
        );
    }
    Ok(Symbol::Membrane(id))
}

fn parse_world_process_list(pair: pest::iterators::Pair<Rule>, ctx: Context) -> Vec<Symbol> {
    let mut list: Vec<Symbol> = Vec::new();
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::Rule => {
                list.push(parse_rule(pair, ctx));
            }
            Rule::DeclarationList => {
                list.append(&mut parse_declaration_list(pair, ctx));

                unsafe {
                    let atoms = ATOMS.get().unwrap();
                    for (id, atom) in atoms.iter() {
                        if atom.membrane == ctx.membrane && !list.contains(&Symbol::Atom(*id)) {
                            list.push(Symbol::Atom(*id));
                        }
                    }
                }
            }
            Rule::EOI => {}
            _ => {
                unreachable!("Unexpected rule: {:?}", pair.as_rule());
            }
        }
    }

    // sort the list
    list.sort_by(|a, b| a.compare(b));

    list
}

fn parse_declaration_list(pair: pest::iterators::Pair<Rule>, ctx: Context) -> Vec<Symbol> {
    let mut symbols = Vec::new();
    let mut counter = 0usize;
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::Declaration => match ctx.from {
                Symbol::Atom(_) => {
                    symbols.push(parse_declaration(
                        pair,
                        Context {
                            pos: Some(counter),
                            ..ctx
                        },
                    ));
                    counter += 1;
                }
                _ => {
                    symbols.push(parse_declaration(pair, ctx));
                }
            },
            _ => {
                unreachable!("Unexpected rule: {:?}", pair.as_rule());
            }
        }
    }
    symbols
}

fn parse_declaration(pair: pest::iterators::Pair<Rule>, ctx: Context) -> Symbol {
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::UnitAtom => {
                return parse_unit_atom(pair, ctx);
            }
            Rule::Context => {
                panic!(
                    "{:?}, Context can only be declared in rule",
                    pair.line_col()
                );
            }
            _ => {
                unreachable!("Unexpected rule: {:?}", pair.as_rule());
            }
        }
    }
    unreachable!();
}

fn parse_unit_atom(pair: pest::iterators::Pair<Rule>, ctx: Context) -> Symbol {
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::Atom => {
                return parse_atom(pair, ctx);
            }
            Rule::Membrane => {
                return parse_membrane(pair, ctx);
            }
            Rule::Link => {
                return parse_link(pair, ctx);
            }
            _ => {
                unreachable!();
            }
        }
    }
    unreachable!();
}

fn parse_link(pair: pest::iterators::Pair<Rule>, ctx: Context) -> Symbol {
    let mut name = "".to_string();
    let pos = pair.as_span().start_pos().pos();
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::LinkName => {
                name = pair.as_str().to_string();
            }
            _ => {
                unreachable!("Unexpected rule: {:?}", pair.as_rule());
            }
        }
    }

    unsafe {
        LINKS.get_or_init(HashMap::new);
        // find if there is a link with the same name
        for (k, v) in LINKS.get_mut().unwrap().iter_mut() {
            if v.name == name {
                v.link2 = Some((ctx.from, ctx.pos.unwrap()));
                v.pos2 = Some(pos);
                return Symbol::Link(*k);
            }
        }
        let id = LINK_ID;
        LINK_ID += 1;
        let link = Link {
            name,
            link1: Some((ctx.from, ctx.pos.unwrap())),
            link2: None,
            pos1: Some(pos),
            pos2: None,
        };
        LINKS.get_mut().unwrap().insert(id, link);
        Symbol::Link(id)
    }
}

fn parse_membrane(pair: pest::iterators::Pair<Rule>, ctx: Context) -> Symbol {
    let mut name = "".to_string();
    let mut process: Vec<Symbol> = Vec::new();
    let parent = ctx.membrane;
    let id = unsafe { ENTITY_ID };
    unsafe { ENTITY_ID += 1 };
    let ctx = Context {
        from: Symbol::Membrane(id),
        pos: None,
        membrane: id,
    };
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::AtomName => {
                name = pair.as_str().to_string();
            }
            Rule::WorldProcessList => {
                process.append(&mut parse_world_process_list(pair, ctx));
            }
            _ => {
                unreachable!("Unexpected rule: {:?}", pair.as_rule());
            }
        }
    }

    let mut rule_set = vec![];
    for symbol in process.iter() {
        if let Symbol::Rule(id) = symbol {
            rule_set.push(*id);
        }
    }
    process.retain(|symbol| !matches!(symbol, Symbol::Rule(_)));

    let membrane = Membrane {
        membrane: parent,
        id,
        name,
        process,
        rule_set,
    };

    unsafe {
        MEMS.get_or_init(HashMap::new);
        MEMS.get_mut().unwrap().insert(id, membrane);
    }
    Symbol::Membrane(id)
}

fn parse_atom(pair: pest::iterators::Pair<Rule>, ctx: Context) -> Symbol {
    let mut name: String = "".to_string();
    let mut process: Vec<Symbol> = Vec::new();
    let id = unsafe { ENTITY_ID };
    unsafe { ENTITY_ID += 1 };
    let mut pos = 0;
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::AtomName => {
                name = pair.as_str().to_string();
            }
            Rule::DeclarationList => {
                process.append(&mut parse_declaration_list(
                    pair,
                    Context {
                        from: Symbol::Atom(id),
                        pos: {
                            pos += 1;
                            Some(pos - 1)
                        },
                        ..ctx
                    },
                ));
            }
            _ => {
                unreachable!("Unexpected rule: {:?}", pair.as_rule());
            }
        }
    }

    let mut atom = Atom {
        membrane: ctx.membrane,
        id,
        name,
        links: process,
    };

    let res = match ctx.from {
        Symbol::Atom(from_id) => {
            let link = Link {
                name: String::new(),
                link1: Some((Symbol::Atom(from_id), ctx.pos.unwrap())),
                link2: Some((Symbol::Atom(id), atom.links.len())),
                pos1: None,
                pos2: None,
            };
            unsafe {
                let id = LINK_ID;
                LINK_ID += 1;

                LINKS.get_or_init(HashMap::new);
                LINKS.get_mut().unwrap().insert(id, link);
                atom.links = vec![Symbol::Link(id)];
                Symbol::Link(id)
            }
        }
        _ => Symbol::Atom(id),
    };

    unsafe {
        _ = ATOMS.get_or_init(HashMap::new);
        ATOMS.get_mut().unwrap().insert(id, atom);
    }

    res
}
