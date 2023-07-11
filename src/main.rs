use liblmntalc::codegen;

mod options;

fn main() {
    match liblmntalc::parser::parse_lmntal(
        r"
    a(X,c), b(X), m{
        {t}; 
        s($p) when int($p); with T := 2; then ;
    }; 
    xx : a,b,e{f} then c, d;
    ",
    ) {
        Ok(s) => {
            // print_result(&s, 0);
            let mut gen = codegen::ILGenerator::default();
            gen.gen(s);
            print!("{}", gen);
        }
        Err(e) => println!("{}", e),
    }
}
