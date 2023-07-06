use liblmntalc::codegen;
use liblmntalc::util::print_result;

mod options;

fn main() {
    match liblmntalc::parser::parse_lmntal("
    a(X), b(X), m{
        {t}; 
        s($p) |int($p)| -> ;
    }; 
    xx : a,b -> c, d;
    ") {
        Ok(s) => {
            // print_result(&s, 0);
            let mut gen = codegen::ILGenerator::default();
            gen.gen(s);
            print!("{}", gen);
        }
        Err(e) => println!("{}", e),
    }
}
