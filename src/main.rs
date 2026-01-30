use vvm::{Program, VM, parse};

const USAGE: &str = "Usage: vvm <run|build|disasm|optimise> <infile> [outfile]";

fn run(file: &String) {
    let data = std::fs::read(file).expect("Failed to read input file");
    let prog = Program::from_bytecode(data).expect("Failed to parse bytecode");
    let mut vm = VM::new(prog);

    vm.register_native_handler("debug", |vm| {
        vm.debug();
    });

    vm.register_native_handler("print", |vm| {
        let value = vm.pop();
        match value {
            vvm::Value::Int(i) => print!("{}", i),
            vvm::Value::String(s) => print!("{}", s),
            vvm::Value::Bool(b) => print!("{}", b),
            vvm::Value::Float(f) => print!("{}", f),
            vvm::Value::UInt(f) => print!("{}", f),
            _ => println!("<complex value>"),
        }
    });

    vm.run();
}

fn build(file: &String, outfile: &String) {
    let source = std::fs::read_to_string(file).expect("Failed to read input file");
    let bytecode = parse(&source).expect("Failed to parse source representation");
    std::fs::write(outfile, bytecode).expect("Failed to write output file");
}

fn disasm(file: &String, outfile: &String) {
    let data = std::fs::read(file).expect("Failed to read input file");
    let prog = Program::from_bytecode(data).expect("Failed to parse bytecode");
    let disassembled = vvm::disasm(prog);
    std::fs::write(outfile, disassembled).expect("Failed to write output file");
}

fn optimise(file: &String, outfile: &String) {
    let data = std::fs::read(file).expect("Failed to read input file");
    let prog = Program::from_bytecode(data).expect("Failed to parse bytecode");
    let optimised_prog = vvm::optimise(&prog);
    let disassembled = vvm::disasm(optimised_prog);
    let bytecode = parse(&disassembled).expect("Failed to parse source representation");
    std::fs::write(outfile, bytecode).expect("Failed to write output file");
}

fn main() {
    let cmd = match std::env::args().nth(1) {
        Some(c) => c,
        None => {
            eprintln!("{}", USAGE);
            return;
        }
    };
    let infile = match std::env::args().nth(2) {
        Some(f) => f,
        None => {
            eprintln!("{}", USAGE);
            return;
        }
    };

    match cmd.as_str() {
        "run" => run(&infile),
        "build" => {
            let outfile = match std::env::args().nth(3) {
                Some(f) => f,
                None => {
                    eprintln!("{}", USAGE);
                    return;
                }
            };
            build(&infile, &outfile);
        }
        "disasm" => {
            let outfile = match std::env::args().nth(3) {
                Some(f) => f,
                None => {
                    eprintln!("{}", USAGE);
                    return;
                }
            };
            disasm(&infile, &outfile);
        }
        "optimise" => {
            let outfile = match std::env::args().nth(3) {
                Some(f) => f,
                None => {
                    eprintln!("{}", USAGE);
                    return;
                }
            };
            optimise(&infile, &outfile);
        }
        _ => {
            eprintln!("{}", USAGE);
            return;
        }
    }
}
