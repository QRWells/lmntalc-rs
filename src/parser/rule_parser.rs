use std::str::FromStr;

use super::*;

pub fn parse_rule(pair: pest::iterators::Pair<ParseRule>, ctx: Context) -> Symbol {
    let id = unsafe { RULE_ID };
    let mut rule = Rule::new(pair.line_col());
    rule.parse(pair, ctx);

    unsafe {
        RULE_ID += 1;
        RULES.get_or_init(HashMap::new);
        RULES.get_mut().unwrap().insert(id, rule);
    }

    Symbol::Rule(id)
}

// Data structures for rules.

#[derive(Debug, Clone, Copy)]
pub enum GuardOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

#[derive(Debug, Clone, Copy)]
pub enum Functor {
    Int,
    Float,
    Unary,
    Uniq,
    Ground,
}

#[derive(Debug, Clone)]
pub enum GuardNode {
    Value(Symbol),
    Functor(Functor, Vec<Symbol>),
    IntValue(i64),
    FloatValue(f64),
    Operation(GuardOperator, Box<GuardNode>, Box<GuardNode>),
}

#[derive(Debug, Default)]
pub struct Rule {
    /// The line and column number of this rule in the source file.
    pub line_col: (usize, usize),

    /// The membrane this rule belongs to.
    pub membrane: MembraneId,

    /// The name of this rule.
    ///
    /// Anonymouse rules are given a generated name.
    pub name: String,

    /// The pattern of this rule.
    pub pattern: Membrane,

    /// The guard of this rule.
    pub guard: Option<GuardNode>,

    /// The body of this rule.
    pub body: Membrane,

    pub(crate) entity_id: usize,
    pub(crate) atoms: Vec<Atom>,
    pub(crate) links: HashMap<LinkId, Link>,
    pub(crate) mems: Vec<Membrane>,
}

impl Rule {
    pub fn new(pos: (usize, usize)) -> Self {
        Self {
            line_col: pos,
            ..Default::default()
        }
    }

    pub fn parse(&mut self, pair: pest::iterators::Pair<ParseRule>, ctx: Context) {
        for pair in pair.into_inner() {
            match pair.as_rule() {
                ParseRule::RuleName => {
                    self.name = pair.as_str().to_string();
                }
                ParseRule::Pattern => {
                    self.pattern = self.parse_root(pair, ctx);
                }
                ParseRule::Body => {
                    self.body = self.parse_root(pair, ctx);
                }
                ParseRule::Guard => {
                    self.guard = Some(self.parse_guard(pair));
                }
                _ => {
                    unreachable!("Unexpected rule: {:?}", pair.as_rule())
                }
            }
        }
    }

    fn parse_root(&mut self, pair: pest::iterators::Pair<ParseRule>, ctx: Context) -> Membrane {
        let mut process = Vec::new();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                ParseRule::DeclarationList => {
                    // pattern actually is a membrane.
                    process.append(&mut self.parse_declaration_list(pair, ctx));
                }
                _ => {
                    unreachable!("Unexpected rule: {:?}", pair.as_rule())
                }
            }
        }

        Membrane {
            name: "".to_string(),
            membrane: ctx.membrane,
            id: ctx.membrane,
            process,
            rule_set: vec![],
        }
    }

    // Guard parsing

    /// Parse a guard function constraint.
    fn parse_guard(&mut self, pair: pest::iterators::Pair<ParseRule>) -> GuardNode {
        for pair in pair.into_inner() {
            match pair.as_rule() {
                ParseRule::OrExpr => {
                    return self.parse_expr(pair);
                }
                _ => {
                    unreachable!("Unexpected rule: {:?}", pair.as_rule());
                }
            }
        }
        unreachable!()
    }

    fn parse_expr(&mut self, pair: pest::iterators::Pair<ParseRule>) -> GuardNode {
        let lhs_parsed = false;
        let mut lhs: GuardNode = GuardNode::IntValue(0);
        let mut op: rule_parser::GuardOperator = rule_parser::GuardOperator::Or;
        for pair in pair.into_inner() {
            match pair.as_rule() {
                ParseRule::GuardFuncConstraint => {
                    return self.parse_guard_func(pair);
                }
                ParseRule::Float => {
                    return GuardNode::FloatValue(pair.as_str().parse().unwrap());
                }
                ParseRule::Int => {
                    return GuardNode::IntValue(pair.as_str().parse().unwrap());
                }
                ParseRule::GuardFunctor => {
                    return GuardNode::Value(self.get_symbol(pair.as_str()));
                }
                ParseRule::OrExpr
                | ParseRule::AndExpr
                | ParseRule::RelExpr
                | ParseRule::AddSubExpr
                | ParseRule::MulDivExpr => {
                    lhs = if !lhs_parsed {
                        self.parse_expr(pair)
                    } else {
                        let rhs = self.parse_expr(pair);
                        GuardNode::Operation(op, Box::new(lhs), Box::new(rhs))
                    };
                }
                _ => {
                    op = op_map(pair.as_rule());
                }
            }
        }
        lhs
    }

    fn parse_guard_func(&mut self, pair: pest::iterators::Pair<ParseRule>) -> GuardNode {
        let mut functor: Functor = Functor::Ground;
        let mut args: Vec<Symbol> = Vec::new();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                ParseRule::GuardInt => {
                    functor = Functor::Int;
                }
                ParseRule::GuardFloat => {
                    functor = Functor::Float;
                }
                ParseRule::GuardUnary => {
                    functor = Functor::Unary;
                }
                ParseRule::GuardUniq => {
                    functor = Functor::Uniq;
                }
                ParseRule::GuardGround => {
                    functor = Functor::Ground;
                }
                ParseRule::GuardFunctorList => {
                    for pair in pair.into_inner() {
                        match pair.as_rule() {
                            ParseRule::GuardFunctor => {
                                args.push(self.get_symbol(pair.as_str()));
                            }
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

        GuardNode::Functor(functor, args)
    }

    fn get_symbol(&mut self, name: &str) -> Symbol {
        for (i, link) in self.links.iter() {
            if link.name == name {
                return Symbol::Link(*i);
            }
        }
        panic!("Symbol not found: {}", name);
    }

    // Parsing pattern and body

    fn parse_declaration_list(
        &mut self,
        pair: pest::iterators::Pair<ParseRule>,
        ctx: Context,
    ) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                ParseRule::Declaration => {
                    symbols.push(self.parse_declaration(pair, ctx));
                }
                _ => {
                    unreachable!("Unexpected rule: {:?}", pair.as_rule());
                }
            }
        }
        symbols
    }

    fn parse_declaration(
        &mut self,
        pair: pest::iterators::Pair<ParseRule>,
        ctx: Context,
    ) -> Symbol {
        for pair in pair.into_inner() {
            match pair.as_rule() {
                ParseRule::UnitAtom => {
                    return self.parse_unit_atom(pair, ctx);
                }
                _ => {
                    unreachable!("Unexpected rule: {:?}", pair.as_rule());
                }
            }
        }
        unreachable!();
    }

    fn parse_unit_atom(&mut self, pair: pest::iterators::Pair<ParseRule>, ctx: Context) -> Symbol {
        for pair in pair.into_inner() {
            match pair.as_rule() {
                ParseRule::Atom => {
                    return self.parse_atom(pair, ctx);
                }
                ParseRule::Membrane => {
                    return self.parse_membrane(pair, ctx);
                }
                ParseRule::Link => match ctx.from {
                    Symbol::Rule(id) | Symbol::Membrane(id) => {
                        panic!("Top-level link is not allowed in rule {}", id);
                    }
                    _ => {
                        return self.parse_link(pair, ctx);
                    }
                },
                _ => {
                    unreachable!();
                }
            }
        }
        unreachable!();
    }

    fn parse_link(&mut self, pair: pest::iterators::Pair<ParseRule>, ctx: Context) -> Symbol {
        let mut name = "".to_string();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                ParseRule::LinkName => {
                    name = pair.as_str().to_string();
                }
                _ => {
                    unreachable!("Unexpected rule: {:?}", pair.as_rule());
                }
            }
        }

        // find if there is a link with the same name
        for (k, v) in self.links.iter_mut() {
            if v.name == name {
                v.link2 = Some((ctx.from, ctx.pos.unwrap()));
                return Symbol::Link(*k);
            }
        }
        let id = self.links.len();
        let link = Link {
            name,
            link1: Some((ctx.from, ctx.pos.unwrap())),
            link2: None,
        };
        self.links.insert(id, link);
        Symbol::Link(id)
    }

    fn parse_membrane(&mut self, pair: pest::iterators::Pair<ParseRule>, ctx: Context) -> Symbol {
        let mut name = "".to_string();
        let mut process: Vec<Symbol> = Vec::new();
        let id = self.entity_id;
        self.entity_id += 1;
        for pair in pair.into_inner() {
            match pair.as_rule() {
                ParseRule::AtomName => {
                    name = pair.as_str().to_string();
                }
                ParseRule::WorldProcessList => {
                    process.append(&mut parse_world_process_list(
                        pair,
                        Context {
                            from: Symbol::Membrane(id),
                            ..ctx
                        },
                    ));
                }
                _ => {
                    unreachable!("Unexpected rule: {:?}", pair.as_rule());
                }
            }
        }
        let membrane = Membrane {
            membrane: ctx.membrane,
            id,
            name,
            process,
            rule_set: vec![],
        };
        self.mems.push(membrane);
        Symbol::Membrane(id)
    }

    fn parse_atom(&mut self, pair: pest::iterators::Pair<ParseRule>, ctx: Context) -> Symbol {
        let mut name: String = "".to_string();
        let mut process: Vec<Symbol> = Vec::new();
        let id = self.entity_id;
        self.entity_id += 1;
        let mut pos = 0;
        for pair in pair.into_inner() {
            match pair.as_rule() {
                ParseRule::AtomName => {
                    name = pair.as_str().to_string();
                }
                ParseRule::DeclarationList => {
                    process.append(&mut self.parse_declaration_list(
                        pair,
                        Context {
                            from: Symbol::Atom(id),
                            pos: Some({
                                pos += 1;
                                pos - 1
                            }),
                            ..ctx
                        },
                    ));
                }
                _ => {
                    unreachable!("Unexpected rule: {:?}", pair.as_rule());
                }
            }
        }

        let atom = if process.is_empty() {
            Atom {
                membrane: ctx.membrane,
                id,
                name,
                process: None,
            }
        } else {
            Atom {
                membrane: ctx.membrane,
                id,
                name,
                process: Some(process),
            }
        };
        self.atoms.push(atom);
        Symbol::Atom(id)
    }
}

fn op_map(rule: ParseRule) -> GuardOperator {
    match rule {
        ParseRule::OR => GuardOperator::Or,
        ParseRule::AND => GuardOperator::And,
        ParseRule::EQ => GuardOperator::Eq,
        ParseRule::NE => GuardOperator::Neq,
        ParseRule::LT => GuardOperator::Lt,
        ParseRule::LE => GuardOperator::Le,
        ParseRule::GT => GuardOperator::Gt,
        ParseRule::GE => GuardOperator::Ge,
        ParseRule::ADD => GuardOperator::Add,
        ParseRule::SUB => GuardOperator::Sub,
        ParseRule::MUL => GuardOperator::Mul,
        ParseRule::DIV => GuardOperator::Div,
        ParseRule::MOD => GuardOperator::Mod,
        _ => {
            unreachable!("Unexpected rule: {:?}", rule);
        }
    }
}
