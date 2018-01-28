pub mod worker;

pub fn validate_args(args: &Vec<String>) {
    if args.len() == 1 {
        eprintln!("No filename or destination provided");
        std::process::exit(1);
    }
    if args.len() == 2 {
        eprintln!("No destination provided");
        std::process::exit(2);
    }
}

pub fn collect_args(args: &Vec<String>) -> (&String, &String) {
    let first = &args[1];
    let second = &args[2];
    (first, second)
}
