use std::env;
use std::process;
use std::io;

mod ijvm;
use ijvm::Ijvm;

fn print_help() {
    println!("Usage: ijvm binary");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        process::exit(1);
    }

    let binary_path = &args[1];

    // Initialize IJVM with stdin/stdout (equivalent to init_ijvm_std)
    let machine_result = Ijvm::new(
        binary_path, 
        Box::new(io::stdin()), 
        Box::new(io::stdout())
    );

    match machine_result {
        Ok(mut machine) => {
            machine.run();
            // Rust automatically calls "destroy" (Drop trait) when 'machine' goes out of scope
        },
        Err(e) => {
            eprintln!("Couldn't load binary {}: {}", binary_path, e);
            process::exit(1);
        }
    }
}
