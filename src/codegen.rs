use std::{collections::HashMap, fmt::Display};

use colored::Colorize;

use crate::parser::{
    data::{Membrane, Symbol},
    rule_parser::Rule,
    MEMS, RULES,
};

use self::{il::IL, rule_gen::RuleIL};

pub mod il;
mod rule_gen;

pub enum Target {
    Text,
    Binary,
}

#[derive(Debug, Default)]
pub struct ILGenerator {
    init_rule: Vec<IL>,
    /// Membrane ID -> Rule Set
    /// Rule set contains rules in the membrane.
    rule_sets: HashMap<usize, Vec<RuleIL>>,
}

impl Display for ILGenerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", "Init".magenta())?;
        for il in &self.init_rule {
            writeln!(f, "{}", il)?;
        }
        writeln!(f)?;
        for (mem_id, rule_set) in &self.rule_sets {
            writeln!(f, "{} {}", "RuleSet".blue(), mem_id)?;
            for rule in rule_set {
                writeln!(f, "{}", rule)?;
            }
        }
        Ok(())
    }
}

impl ILGenerator {
    /// Entry point of code generation.
    pub fn gen(&mut self, symbol: Symbol) {
        match symbol {
            Symbol::Membrane(id) => {
                let mem = unsafe { MEMS.get().unwrap().get(&id).unwrap() };
                self.gen_init_mem(mem);
            }
            _ => unreachable!(),
        }
    }

    fn gen_init_mem(&mut self, mem: &Membrane) {
        for process in &mem.process {
            let mut unit = self.gen_unit(*process);
            self.init_rule.append(&mut unit);
        }

        // while let Some(process) = self.queue.pop() {
        //     self.gen_unit(process);
        // }

        for rule_id in &mem.rule_set {
            let rule = unsafe { RULES.get().unwrap().get(rule_id).unwrap() };
            let rule_il = gen_rule(rule);
            self.emit_rule(mem.id, rule_il);
        }
    }

    fn gen_unit(&mut self, symbol: Symbol) -> Vec<IL> {
        match symbol {
            Symbol::Atom(id) => {
                let atom = unsafe { crate::parser::ATOMS.get().unwrap().get(&id).unwrap() };
                ILGenerator::gen_atom(atom)
            }
            Symbol::Membrane(id) => {
                let mem = unsafe { crate::parser::MEMS.get().unwrap().get(&id).unwrap() };
                self.gen_mem(mem)
            }
            _ => {
                unreachable!()
            }
        }
    }

    fn gen_atom(atom: &crate::parser::data::Atom) -> Vec<IL> {
        let mut il = vec![IL::NewAtom(
            atom.id,
            atom.membrane,
            atom.name.clone(),
            atom.links.len(),
        )];

        for link in &atom.links {
            if let Symbol::Link(id) = link {
                let link = unsafe { crate::parser::LINKS.get().unwrap().get(id).unwrap() };
                if let Some(link2) = link.link2 {
                    if Into::<usize>::into(link2.0) == atom.id {
                        il.push(IL::new_link(link, 0));
                    }
                }
            }
        }

        il
    }

    fn gen_mem(&mut self, mem: &Membrane) -> Vec<IL> {
        let mut il = Vec::new();
        il.push(IL::NewMem(mem.id, mem.membrane));
        if !mem.name.is_empty() {
            il.push(IL::SetMemName(mem.id, mem.name.clone()));
        }
        for process in &mem.process {
            let mut unit = self.gen_unit(*process);
            il.append(&mut unit);
        }
        for rule_id in &mem.rule_set {
            let rule = unsafe { RULES.get().unwrap().get(rule_id).unwrap() };
            let rule_il = gen_rule(rule);
            self.emit_rule(mem.id, rule_il);
        }
        il
    }
}

fn gen_rule(rule: &Rule) -> RuleIL {
    let mut rule_gen = rule_gen::RuleGenerator::new(rule);
    rule_gen.gen();
    rule_gen.il
}
