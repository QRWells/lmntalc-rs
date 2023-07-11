use crate::parser::{
    data::{self, Membrane, Symbol},
    rule_parser,
};

use super::{il::IL, RuleIL};

#[derive(Debug)]
pub(crate) struct RuleGenerator<'a> {
    rule: &'a rule_parser::Rule,
    register: usize,
    remove_stack: Vec<(Symbol, usize)>,
    pub(crate) il: RuleIL,
}

impl<'a> RuleGenerator<'a> {
    pub fn new(rule: &'a rule_parser::Rule) -> Self {
        Self {
            rule,
            register: 0,
            remove_stack: Vec::new(),
            il: RuleIL::default(),
        }
    }

    pub(crate) fn gen(&mut self) {
        self.il.name = self.rule.name.clone();
        self.gen_pattern();
        self.gen_guard();
        self.gen_body();
    }

    fn gen_pattern(&mut self) {
        let mem = &self.rule.pattern;
        let mut atom_counter = 0;
        let mut mem_counter = 0;
        for p in &mem.process {
            let reg = self.register;
            let rule = self.rule;
            match p {
                data::Symbol::Atom(_) => {
                    let atom = rule.atoms.get(atom_counter).unwrap();
                    atom_counter += 1;
                    self.il.pattern.push(IL::FindAtom(
                        reg,
                        atom.membrane,
                        atom.name.clone(),
                        atom.links.len(),
                    ));
                    self.remove_stack.push((Symbol::Atom(reg), atom.membrane));
                    self.register += 1;
                    // todo: gen process
                }
                data::Symbol::Membrane(_) => {
                    let mem = rule.mems.get(mem_counter).unwrap();
                    mem_counter += 1;
                    self.il.pattern.push(IL::AnyMem(
                        reg,
                        rule.membrane,
                        0,
                        if !mem.name.is_empty() {
                            Some(mem.name.clone())
                        } else {
                            None
                        },
                    ));
                    self.remove_stack
                        .push((Symbol::Membrane(reg), mem.membrane));
                    self.register += 1;

                    self.il.pattern.push(IL::NAtoms(reg, mem.process.len()))
                }
                _ => {
                    unreachable!("Unexpected symbol: {:?}", p);
                }
            }
        }
    }

    fn gen_guard(&mut self) {}

    fn gen_body(&mut self) {
        for (symbol, mem) in self.remove_stack.iter().rev() {
            match symbol {
                Symbol::Atom(id) => {
                    self.il.body.push(IL::RemoveAtom(*id, *mem));
                }
                Symbol::Membrane(id) => {
                    self.il.body.push(IL::RemoveMem(*id, *mem));
                }
                _ => {
                    unreachable!()
                }
            }
        }
        let mut il = Vec::new();
        let mem = &self.rule.body;
        for process in &mem.process {
            let mut unit = self.gen_unit(*process);
            il.append(&mut unit);
        }
        self.il.body.append(&mut il);
    }

    fn gen_unit(&mut self, symbol: Symbol) -> Vec<IL> {
        match symbol {
            Symbol::Atom(id) => {
                for atom in &self.rule.atoms {
                    if atom.id == id {
                        return self.gen_atom(atom);
                    }
                }
                unreachable!()
            }
            Symbol::Membrane(id) => {
                for mem in &self.rule.mems {
                    if mem.id == id {
                        return self.gen_mem(mem);
                    }
                }
                unreachable!()
            }
            _ => {
                unreachable!()
            }
        }
    }

    fn gen_atom(&mut self, atom: &crate::parser::data::Atom) -> Vec<IL> {
        let mut il = vec![IL::NewAtom(
            atom.id,
            atom.membrane,
            atom.name.clone(),
            atom.links.len(),
        )];

        for link in &atom.links {
            if let Symbol::Link(id) = link {
                let link = self.rule.links.get(id).unwrap();
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
        // for rule_id in &mem.rule_set {
        //     let rule = unsafe { RULES.get().unwrap().get(rule_id).unwrap() };
        //     let rule_il = gen_rule(rule);
        //     self.emit_rule(mem.id, rule_il);
        // }
        il
    }
}
