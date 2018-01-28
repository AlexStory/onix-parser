extern crate xml;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    xml::validate_args(&args);
    let (filename, destination) = xml::collect_args(&args);
    let mut contents = xml::worker::get_file_contents(filename);
    contents = xml::worker::replace_tags(contents);
    xml::worker::write_file(contents, destination);

    println!("Completed Successfully. file at {}", destination);
}
