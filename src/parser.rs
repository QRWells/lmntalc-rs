use std::collections::HashMap;

use lazy_static::lazy_static;
use pest::{error::Error, Parser};

use crate::data::{self, Atom, Link, Memebrane, Program};

#[derive(Parser)]
#[grammar = "lmntal.pest"]
struct LMNParser;

static mut ATOM_ID: usize = 0;
static mut LINK_ID: usize = 0;
static mut RULE_ID: usize = 0;
static mut MEM_ID: usize = 0;

lazy_static! {
    static ref ATOMS: HashMap<usize, Atom> = HashMap::new();
    static ref LINKS: HashMap<usize, Link> = HashMap::new();
    static ref RULES: HashMap<usize, data::Rule> = HashMap::new();
    static ref MEMS: HashMap<usize, Memebrane> = HashMap::new();
}

pub fn parse_lmntal(file: &str) -> Result<Program, Error<Rule>> {
    let pairs = LMNParser::parse(Rule::Program, file)?;
    for pair in pairs {
        match pair.as_rule() {
            Rule::Program => {
                for pair in pair.into_inner() {
                    match pair.as_rule() {
                        Rule::WorldProcessList => {
                            parse_world_process_list(pair);
                        }
                        Rule::EOI => {}
                        _ => {
                            println!("Unexpected rule: {:?}", pair.as_rule());
                        }
                    }
                }
            }
            _ => {
                println!("Unexpected rule: {:?}", pair.as_rule());
            }
        }
    }
    Ok(Program {})
}

fn parse_world_process_list(pair: pest::iterators::Pair<Rule>) {
    println!("Processing world process list");
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::Rule => {
                parse_rule(pair);
            }
            Rule::DeclarationList => {
                parse_declaration_list(pair);
            }
            Rule::EOI => {}
            _ => {
                println!("Unexpected rule: {:?}", pair.as_rule());
            }
        }
    }
}

fn parse_declaration_list(pair: pest::iterators::Pair<Rule>) {
    println!("Processing declaration list");
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::Declaration => {
                parse_declaration(pair);
            }
            _ => {
                println!("Unexpected rule: {:?}", pair.as_rule());
            }
        }
    }
}

fn parse_declaration(pair: pest::iterators::Pair<Rule>) {
    println!("Processing declaration");
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::UnitAtom => {
                parse_unit_atom(pair);
            }
            _ => {
                println!("Unexpected rule: {:?}", pair.as_rule());
            }
        }
    }
}

fn parse_unit_atom(pair: pest::iterators::Pair<Rule>) {
    println!("Processing unit atom");
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::Atom => {
                parse_atom(pair);
            }
            Rule::Membrane => {
                parse_membrane(pair);
            }
            Rule::Link => {
                parse_link(pair);
            }
            _ => {
                println!("Unexpected rule: {:?}", pair.as_rule());
            }
        }
    }
}

fn parse_link(pair: pest::iterators::Pair<Rule>) {
    println!("Processing link");
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::LinkName => {
                println!("Processing link name {}", pair.as_str());
            }
            _ => {
                println!("Unexpected rule: {:?}", pair.as_rule());
            }
        }
    }
}

fn parse_membrane(pair: pest::iterators::Pair<Rule>) {
    println!("Processing membrane");
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::AtomName => {
                println!("Processing membrane name {}", pair.as_str());
            }
            Rule::WorldProcessList => {
                parse_world_process_list(pair);
            }
            _ => {
                println!("Unexpected rule: {:?}", pair.as_rule());
            }
        }
    }
}

fn parse_atom(pair: pest::iterators::Pair<Rule>) {
    println!("Processing atom");
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::AtomName => {
                println!("Processing atom name {}", pair.as_str());
            }
            Rule::DeclarationList => {
                parse_declaration_list(pair);
            }
            _ => {
                println!("Unexpected rule: {:?}", pair.as_rule());
            }
        }
    }
}

fn parse_rule(pair: pest::iterators::Pair<Rule>) {
    println!("Processing rule");
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::RuleName => {
                println!("Processing rule name {}", pair.as_str());
            }
            Rule::DeclarationList => {
                parse_declaration_list(pair);
            }
            _ => {
                println!("Unexpected rule: {:?}", pair.as_rule());
            }
        }
    }
}
