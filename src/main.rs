use std::path;

mod purge;

fn usage(program: &str) {
    eprintln!("usage: {} <folder>", program);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        usage(&args[0]);
        std::process::exit(1);
    }

    let current_path = path::Path::new(&args[1]);
    purge::purge(current_path).unwrap();

    println!(
        "Removed All node_modules from directory {}",
        current_path.canonicalize().unwrap().display()
    );
}
