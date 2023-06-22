use colored::Colorize;

use crate::parser::{self, data::Symbol};

pub fn print_indent(indent: usize) {
    for _ in 0..indent {
        print!("  ");
    }
}

pub fn print_result(s: &Symbol, indent: usize) {
    match s {
        Symbol::Atom(a) => unsafe {
            let atom = parser::ATOMS.get().unwrap().get(a).unwrap();
            print_indent(indent);
            println!("{} id:{} name:{}", "Atom".bold().blue(), atom.id, atom.name);
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
            println!("{} name:{}", "Rule".bold().magenta(), rule.name);
            print_indent(indent + 1);
            println!("{}", "pattern".bright_magenta());
            println!("{:?}", rule.pattern);
            if let Some(g) = &rule.guard {
                print_indent(indent + 1);
                println!("guard");
                println!("{:?}", g);
            }
            print_indent(indent + 1);
            println!("body");
            println!("{:?}", rule.body);
        },
        Symbol::Membrane(m) => unsafe {
            let mem = parser::MEMS.get().unwrap().get(m).unwrap();
            print_indent(indent);
            println!("{} id:{} name:{}","Membrane".bold().green(), mem.id, mem.name);
            for s in &mem.process {
                print_result(s, indent + 1);
            }
            for r in &mem.rule_set {
                print_result(&Symbol::Rule(*r), indent + 1);
            }
        },
    }
}
