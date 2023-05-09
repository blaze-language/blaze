use blaze::compiler::Compiler;
use blaze::error::BlazeError;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    if args.len() == 0 {
        println!("No input files!");
        return;
    }

    let mut compiler: Compiler = Compiler::new();
    compiler.add_file("runtime/prelude.bl".to_string());
    for arg in args {
        compiler.add_file(arg);
    }
    let result: Result<(), BlazeError> = compiler.compile();

    if let Err(error) = result {
        println!("{}", error.to_string());
    }

    if compiler.errors.len() > 0 {
        println!("{} errors found!", compiler.errors.len());
        for error in compiler.errors {
            println!("{}", error.to_string());
        }
    }

    println!("Done!");
}