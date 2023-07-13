use liblmntalc::codegen;

mod options;

fn main() {
    match liblmntalc::parser::parse_lmntal(
        r"
a(10), b(20);
case_rule:
a(X), b(Y)
when int(X) && int(Y);
    with Z := X + Y;
    then c(Z);
when float(X) && float(Y);
    with W := X - Y;
    then c(W);
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
