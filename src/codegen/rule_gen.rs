use clap::Id;

use crate::parser::{
    data::{self, AtomId, LinkId, MembraneId, ProcContextId, Symbol},
    rule_parser, RULES,
};

use super::{
    il::{Label, IL},
    ILGenerator,
};

struct RuleGenContext<'a> {
    rule: &'a rule_parser::Rule,
    id: usize,
    register: usize,
    remove_stack: Vec<(Symbol, usize)>,
}

impl ILGenerator {
    pub(crate) fn gen_rule(&mut self, id: usize) {
        self.emit(IL::Label(Label::Rule(id)));

        let rule = unsafe { RULES.get().unwrap().get(&id).unwrap() };

        self.emit(IL::Spec(1, rule.entity_id));

        let mut ctx = RuleGenContext {
            rule,
            id,
            register: 0,
            remove_stack: Vec::new(),
        };

        self.gen_pattern(&mut ctx);
        self.gen_body(&mut ctx);
    }

    fn gen_pattern(&mut self, ctx: &mut RuleGenContext) {
        let mem = &ctx.rule.pattern;
        let mut atom_counter = 0;
        let mut mem_counter = 0;
        for p in &mem.process {
            let reg = ctx.register;
            let rule = ctx.rule;
            match p {
                data::Symbol::Atom(_) => {
                    let atom = rule.atoms.get(atom_counter).unwrap();
                    atom_counter += 1;
                    self.emit(IL::FindAtom(
                        reg,
                        atom.membrane,
                        atom.name.clone(),
                        if let Some(p) = &atom.process {
                            p.len()
                        } else {
                            0
                        },
                    ));
                    ctx.remove_stack.push((Symbol::Atom(reg), atom.membrane));
                    ctx.register += 1;
                    // todo: gen process
                }
                data::Symbol::Membrane(_) => {
                    let mem = rule.mems.get(mem_counter).unwrap();
                    mem_counter += 1;
                    self.emit(IL::AnyMem(
                        reg,
                        rule.membrane,
                        0,
                        if !mem.name.is_empty() {
                            Some(mem.name.clone())
                        } else {
                            None
                        },
                    ));
                    ctx.remove_stack.push((Symbol::Membrane(reg), mem.membrane));
                    ctx.register += 1;
                    // todo: gen process
                }
                _ => {
                    unreachable!("Unexpected symbol: {:?}", p);
                }
            }
        }
        self.emit(IL::Commit(ctx.rule.name.clone(), ctx.rule.line_col.0));
    }

    fn gen_body(&mut self, ctx: &mut RuleGenContext) {
        // remove found atoms
        for (symbol, mem) in ctx.remove_stack.drain(..) {
            match symbol {
                Symbol::Atom(id) => {
                    self.emit(IL::RemoveAtom(id, mem));
                }
                Symbol::Membrane(id) => {
                    self.emit(IL::RemoveMem(id, mem));
                }
                _ => unreachable!(),
            }
        }

        let body = &ctx.rule.body;

        for process in &body.process {
            self.gen_rule_inner(ctx, *process);
        }
    }

    fn gen_rule_inner(&mut self, ctx: &mut RuleGenContext, symbol: Symbol) {
        match symbol {
            Symbol::Atom(id) => self.gen_rule_atom(ctx, id),
            Symbol::Link(id) => self.gen_rule_link(ctx, id),
            Symbol::Membrane(id) => self.gen_rule_membrane(ctx, id),
            Symbol::ProcContext(id) => self.gen_rule_proc_context(ctx, id),
            _ => {}
        }
    }

    fn gen_rule_atom(&mut self, ctx: &mut RuleGenContext, id: AtomId) {
        let atom = ctx.rule.atoms.get(id).unwrap();
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

    fn gen_rule_link(&mut self, ctx: &mut RuleGenContext, id: LinkId) {}

    fn gen_rule_membrane(&mut self, ctx: &mut RuleGenContext, id: MembraneId) {}

    fn gen_rule_proc_context(&mut self, ctx: &mut RuleGenContext, id: ProcContextId) {}
}
