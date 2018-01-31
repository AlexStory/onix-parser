extern crate xml_tag_parser as xml;
extern crate time;

use time::PreciseTime;
use std::env;

fn main() {
    let start = PreciseTime::now();
    let args: Vec<String> = env::args().collect();

    xml::validate_args(&args);
    let (filename, xml_tags, destination) = xml::collect_args(&args);
    let mut contents = xml::worker::get_file_contents(filename);
    contents = xml::worker::replace_tags(contents, xml_tags);

    xml::worker::write_file(contents, destination);
    let end = PreciseTime::now();

    println!("Completed Successfully. Ran in {}", start.to(end));
}
