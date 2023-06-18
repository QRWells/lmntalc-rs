use super::*;

pub(crate) fn parse_rule(pair: pest::iterators::Pair<ParserRule>, membrane: MembraneId) -> Symbol {
    let mut name = "".to_string();
    let mut pattern: Pattern = Pattern::new();
    let mut guard: Option<Vec<Symbol>> = None;
    let mut body: Vec<Symbol> = Vec::new();
    let id = unsafe { RULE_ID };

    for pair in pair.into_inner() {
        match pair.as_rule() {
            ParserRule::RuleName => {
                name = pair.as_str().to_string();
            }
            ParserRule::Pattern => {
                pattern = parse_pattern(pair, id);
            }
            ParserRule::Body => {
                body = parse_body(pair, id);
            }
            ParserRule::Guard => {
                guard = Some(parse_guard(pair, id));
            }
            _ => {
                unreachable!("Unexpected rule: {:?}", pair.as_rule())
            }
        }
    }

    let mut rule = data::Rule::new(membrane, name);
    rule.pattern = pattern;
    rule.guard = guard;
    rule.body = body;

    unsafe {
        RULE_ID += 1;
        RULES.get_or_init(HashMap::new);
        RULES.get_mut().unwrap().insert(id, rule);
    }

    Symbol::Rule(id)
}

fn parse_pattern(pair: pest::iterators::Pair<ParserRule>, rule: RuleId) -> Pattern {
    let mut pattern = Pattern::new();
    let pair = pair.into_inner().next().unwrap().into_inner();
    for pair in pair {
        match pair.as_rule() {
            ParserRule::Declaration => {
                todo!()
            }
            _ => {
                unreachable!("Unexpected rule: {:?}", pair.as_rule())
            }
        }
    }
    pattern
}

fn parse_body(pair: pest::iterators::Pair<ParserRule>, rule: RuleId) -> Vec<Symbol> {
    todo!()
}

fn parse_guard(pair: pest::iterators::Pair<ParserRule>, rule: RuleId) -> Vec<Symbol> {
    let mut guard: Vec<Symbol> = Vec::new();
    for pair in pair.into_inner() {
        match pair.as_rule() {
            ParserRule::LogicalOperand => {
                guard.append(&mut parse_declaration_list(pair, None, 0));
            }
            ParserRule::LogicalOperator => {}
            _ => {
                unreachable!("Unexpected rule: {:?}", pair.as_rule())
            }
        }
    }
    guard
}

// Functions for parsing other rules

fn parse_declaration(pair: pest::iterators::Pair<ParserRule>, rule: RuleId) -> Symbol {
    for pair in pair.into_inner() {
        match pair.as_rule() {
            ParserRule::UnitAtom => {
                return parse_unit_atom(pair, rule);
            }
            _ => {
                unreachable!("Unexpected rule: {:?}", pair.as_rule());
            }
        }
    }
    unreachable!();
}

fn parse_unit_atom(pair: pest::iterators::Pair<ParserRule>, rule: RuleId) -> Symbol {
    todo!()
}

fn parse_link(pair: pest::iterators::Pair<ParserRule>, rule: RuleId) -> Symbol {
    todo!()
}

fn parse_membrane(pair: pest::iterators::Pair<ParserRule>, rule: RuleId) -> Symbol {
    todo!()
}

fn parse_atom(pair: pest::iterators::Pair<ParserRule>, rule: RuleId) -> Symbol {
    todo!()
}
