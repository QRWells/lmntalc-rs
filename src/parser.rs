use once_cell::sync::OnceCell;
use pest::Parser;
use std::collections::HashMap;

use crate::data::{self, *};

#[derive(Parser)]
#[grammar = "lmntal.pest"]
struct LMNParser;

static mut ATOM_ID: AtomId = 0;
static mut LINK_ID: LinkId = 0;
static mut RULE_ID: RuleId = 0;
static mut MEM_ID: MembraneId = 0;

pub static mut ATOMS: OnceCell<HashMap<AtomId, Atom>> = OnceCell::new();
pub static mut LINKS: OnceCell<HashMap<LinkId, Link>> = OnceCell::new();
pub static mut RULES: OnceCell<HashMap<RuleId, data::Rule>> = OnceCell::new();
pub static mut MEMS: OnceCell<HashMap<MembraneId, Membrane>> = OnceCell::new();

pub fn parse_lmntal(file: &str) -> Result<Symbol, Box<pest::error::Error<Rule>>> {
    let pairs = LMNParser::parse(Rule::Program, file)?;
    let mut init_process = Vec::new();
    let id = unsafe { MEM_ID };
    for pair in pairs {
        match pair.as_rule() {
            Rule::Program => {
                for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::WorldProcessList => {
                            init_process.append(&mut parse_world_process_list(
                                pair,
                                Some(Symbol::Membrane(id)),
                            ));
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
        MEM_ID += 1;
        MEMS.get_or_init(HashMap::new);
        MEMS.get_mut().unwrap().insert(
            id,
            Membrane {
                name: "init".to_string(),
                process: init_process,
            },
        );
    }
    Ok(Symbol::Membrane(id))
}

fn parse_world_process_list(
    pair: pest::iterators::Pair<Rule>,
    from: Option<Symbol>,
) -> Vec<Symbol> {
    let mut list: Vec<Symbol> = Vec::new();
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::Rule => {
                list.push(parse_rule(pair));
            }
            Rule::DeclarationList => {
                list.append(&mut parse_declaration_list(pair, from));
            }
            Rule::EOI => {}
            _ => {
                unreachable!("Unexpected rule: {:?}", pair.as_rule());
            }
        }
    }
    list
}

fn parse_declaration_list(pair: pest::iterators::Pair<Rule>, from: Option<Symbol>) -> Vec<Symbol> {
    let mut symbols = Vec::new();
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::Declaration => {
                symbols.push(parse_declaration(pair, from));
            }
            _ => {
                unreachable!("Unexpected rule: {:?}", pair.as_rule());
            }
        }
    }
    symbols
}

fn parse_declaration(pair: pest::iterators::Pair<Rule>, from: Option<Symbol>) -> Symbol {
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::UnitAtom => {
                return parse_unit_atom(pair, from);
            }
            _ => {
                unreachable!("Unexpected rule: {:?}", pair.as_rule());
            }
        }
    }
    unreachable!();
}

fn parse_unit_atom(pair: pest::iterators::Pair<Rule>, from: Option<Symbol>) -> Symbol {
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::Atom => {
                return parse_atom(pair, from);
            }
            Rule::Membrane => {
                return parse_membrane(pair, from);
            }
            Rule::Link => {
                return parse_link(pair, from);
            }
            _ => {
                unreachable!();
            }
        }
    }
    unreachable!();
}

fn parse_link(pair: pest::iterators::Pair<Rule>, from: Option<Symbol>) -> Symbol {
    let mut name = "".to_string();
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
                v.link2 = from;
                return Symbol::Link(*k);
            }
        }
        let id = LINK_ID;
        LINK_ID += 1;
        let link = Link {
            name,
            link1: from,
            link2: None,
        };
        LINKS.get_mut().unwrap().insert(id, link);
        Symbol::Link(id)
    }
}

fn parse_membrane(pair: pest::iterators::Pair<Rule>, from: Option<Symbol>) -> Symbol {
    let mut name = "".to_string();
    let mut process: Vec<Symbol> = Vec::new();
    let id = unsafe { MEM_ID };
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::AtomName => {
                name = pair.as_str().to_string();
            }
            Rule::WorldProcessList => {
                process.append(&mut parse_world_process_list(
                    pair,
                    Some(Symbol::Membrane(id)),
                ));
            }
            _ => {
                unreachable!("Unexpected rule: {:?}", pair.as_rule());
            }
        }
    }
    let membrane = Membrane { name, process };
    unsafe {
        MEM_ID += 1;
        MEMS.get_or_init(HashMap::new);
        MEMS.get_mut().unwrap().insert(id, membrane);
    }
    Symbol::Membrane(id)
}

fn parse_atom(pair: pest::iterators::Pair<Rule>, from: Option<Symbol>) -> Symbol {
    let mut name: String = "".to_string();
    let mut process: Vec<Symbol> = Vec::new();
    let id = unsafe { ATOM_ID };
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::AtomName => {
                name = pair.as_str().to_string();
            }
            Rule::DeclarationList => {
                process.append(&mut parse_declaration_list(pair, Some(Symbol::Atom(id))));
            }
            _ => {
                unreachable!("Unexpected rule: {:?}", pair.as_rule());
            }
        }
    }

    let atom = if process.is_empty() {
        Atom {
            name,
            process: None,
        }
    } else {
        Atom {
            name,
            process: Some(process),
        }
    };
    unsafe {
        ATOM_ID += 1;
        _ = ATOMS.get_or_init(HashMap::new);
        ATOMS.get_mut().unwrap().insert(id, atom);
    }
    Symbol::Atom(id)
}

fn parse_rule(pair: pest::iterators::Pair<Rule>) -> Symbol {
    let mut stage = 0;
    let mut name = "".to_string();
    let mut pattern: Vec<Symbol> = Vec::new();
    let mut guard: Option<Vec<Symbol>> = None;
    let mut body: Vec<Symbol> = Vec::new();
    let id = unsafe { RULE_ID };
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::RuleName => {
                name = pair.as_str().to_string();
            }
            Rule::DeclarationList => match stage {
                0 => {
                    pattern.append(&mut parse_declaration_list(pair, Some(Symbol::Rule(id))));
                    stage += 1;
                }
                1 => {
                    body.append(&mut parse_declaration_list(pair, Some(Symbol::Rule(id))));
                    stage += 1;
                }
                _ => {
                    unreachable!("Unexpected stage: {}", stage)
                }
            },
            Rule::Guard => {
                guard = Some(parse_declaration_list(pair, Some(Symbol::Rule(id))));
            }
            _ => {
                unreachable!("Unexpected rule: {:?}", pair.as_rule())
            }
        }
    }
    let rule = data::Rule {
        name,
        pattern,
        guard,
        body,
    };
    unsafe {
        RULE_ID += 1;
        RULES.get_or_init(HashMap::new);
        RULES.get_mut().unwrap().insert(id, rule);
    }
    Symbol::Rule(id)
}
