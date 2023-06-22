use std::{collections::HashMap, fmt::Display};

use crate::parser::{
    data::{MembraneId, Symbol},
    ATOMS, LINKS, MEMS,
};

use self::il::{Label, IL};

pub mod il;
mod rule_gen;

pub enum Target {
    Text,
    Binary,
}

#[derive(Debug, Default)]
pub struct ILGenerator {
    il: Vec<IL>,
    rule_sets: HashMap<usize, Vec<IL>>,
    queue: Vec<Symbol>,
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

impl Display for ILGenerator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for il in &self.il {
            writeln!(f, "{}", il)?;
        }
        Ok(())
    }
}

impl ILGenerator {
    /// Entry point of code generation.
    pub fn gen(&mut self, symbol: Symbol) {
        match symbol {
            Symbol::Membrane(id) => self.gen_init(id),
            _ => unreachable!(),
        }
    }

    fn gen_inner(&mut self, symbol: Symbol) {
        match symbol {
            Symbol::Atom(id) => self.gen_atom(id),
            Symbol::Link(id) => self.gen_link(id, 0),
            Symbol::Rule(_) => {}
            Symbol::Membrane(id) => self.gen_membrane(id),
        }
    }

    fn gen_init(&mut self, id: MembraneId) {
        let mem = unsafe { MEMS.get().unwrap().get(&id).unwrap() };
        self.emit(IL::Spec(1, unsafe { super::parser::ENTITY_ID }));
        self.emit(IL::Commit("_init".to_owned(), 0));

        for process in &mem.process {
            self.gen_inner(*process);
        }

        while let Some(process) = self.queue.pop() {
            self.gen_inner(process);
        }

        self.emit(IL::Proceed());

        self.emit(IL::Label(Label::RuleSet(0)));
        for rule in &mem.rule_set {
            self.gen_rule(*rule);
        }
    }

    fn gen_membrane(&mut self, id: usize) {
        let mem = unsafe { MEMS.get().unwrap().get(&id).unwrap() };
        self.emit(IL::NewMem(id, 0));
        if !mem.name.is_empty() {
            self.emit(IL::SetMemName(id, mem.name.clone()));
        }
        for process in &mem.process {
            self.gen_inner(*process);
        }

        if mem.rule_set.is_empty() {
            return;
        }
        self.emit(IL::Label(Label::RuleSet(id)));
        for rule in &mem.rule_set {
            self.gen_rule(*rule);
        }
    }

    fn gen_atom(&mut self, id: usize) {
        let atom = unsafe { ATOMS.get().unwrap().get(&id).unwrap() };
        let functors = if let Some(p) = &atom.process {
            p.len()
        } else {
            0
        };
        let name = format!("'{}'_{}", atom.name, functors);
        self.emit(IL::NewAtom(id, atom.membrane, name));
        if let Some(p) = &atom.process {
            for process in p {
                if !self.queue.contains(process) {
                    self.queue.push(*process);
                }
            }
        }
    }

    fn gen_link(&mut self, id: usize, mem: MembraneId) {
        let link = unsafe { LINKS.get().unwrap().get(&id).unwrap() };
        let (id1, pos1) = if let Some(link1) = &link.link1 {
            match link1 {
                (Symbol::Atom(id), pos) => (*id, *pos),
                (Symbol::Membrane(_), _) => todo!(),
                _ => unreachable!(),
            }
        } else {
            panic!("There is a free link: {:?}", link);
        };
        let (id2, pos2) = if let Some(link2) = &link.link2 {
            match link2 {
                (Symbol::Atom(id), pos) => (*id, *pos),
                (Symbol::Membrane(_), _) => todo!(),
                _ => unreachable!(),
            }
        } else {
            panic!("There is a free link: {:?}", link);
        };
        self.emit(IL::NewLink(id1, pos1, id2, pos2, mem));
    }
}
