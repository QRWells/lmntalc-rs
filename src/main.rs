use liblmntalc::data::Symbol;
use liblmntalc::parser;

pub mod rewriter;

fn main() {
    match liblmntalc::parser::parse_lmntal("a(X), b(X). xx @@ a,b :- c, d.") {
        Ok(s) => {
            print_result(&s, 0);
        }
        Err(e) => println!("{}", e),
    }
}

fn print_indent(indent: usize) {
    for _ in 0..indent {
        print!("  ");
    }
}

fn print_result(s: &Symbol, indent: usize) {
    match s {
        Symbol::Atom(a) => unsafe {
            let atom = parser::ATOMS.get().unwrap().get(a).unwrap();
            print_indent(indent);
            println!("{:?}", atom);
            if let Some(p) = &atom.process {
                for s in p {
                    print_result(s, indent + 1);
                }
            }
        },
        Symbol::Link(l) => unsafe {
            print_indent(indent);
            println!("{:?}", parser::LINKS.get().unwrap().get(l).unwrap());
        },
        Symbol::Rule(r) => unsafe {
            let rule = parser::RULES.get().unwrap().get(r).unwrap();
            print_indent(indent);
            println!("{:?}", rule);
            print_indent(indent + 1);
            println!("pattern");
            for s in &rule.pattern {
                print_result(s, indent + 2);
            }
            if let Some(g) = &rule.guard {
                print_indent(indent + 1);
                println!("guard");
                for s in g {
                    print_result(s, indent + 2);
                }
            }
            print_indent(indent + 1);
            println!("body");
            for s in &rule.body {
                print_result(s, indent + 2);
            }
        },
        Symbol::Membrane(m) => unsafe {
            let mem = parser::MEMS.get().unwrap().get(m).unwrap();
            print_indent(indent);
            println!("{:?}", mem);
            for s in &mem.process {
                print_result(s, indent + 1);
            }
        },
    }
}
