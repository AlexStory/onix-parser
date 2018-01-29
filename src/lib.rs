pub mod worker;

/// Throws errors if either required argument is missing.
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

/// Returns arguments into a tuple for use in program.
///
/// (target, destination)
///
/// target - The xml file to process
///
/// destination - where to write the processed file to
pub fn collect_args(args: &Vec<String>) -> (String, String, String) {
    let first = &args[1];
    let second = &args[2];
    let third = &args[3];
    (first.to_owned(), second.to_owned(), third.to_owned())
}
