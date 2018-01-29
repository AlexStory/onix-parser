use std::fs::File;
use std::io::prelude::*;

/// Replace each tag in file with appropriate replacement.
pub fn replace_tags(mut file: String) -> String {
    for (old, new) in get_tags() {
        file = replace_tag(file, &old, &new);
    }
    file
}

/// Takes a path(filename) and returns the string contents of the file.
pub fn get_file_contents(filename: String) -> String {
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Failed to parse file");

    contents
}

/// Save [contents] to a file at [destination].
pub fn write_file(contents: String, destination: String) -> File {
    let mut new_file = File::create(destination).expect("Could not create file at destination");

    new_file
        .write_all(&contents.trim().as_bytes())
        .expect("Could not write to file");

    new_file
}

/// Replace all instances in the file , of old_tag withe a new_tag.
pub fn replace_tag(mut file: String, old_tag: &str, new_tag: &str) -> String {
    let old = format!("<{}>", old_tag);
    let close_old = format!("</{}>", old_tag);
    let new = format!("<{}>", new_tag);
    let close_new = format!("</{}>", new_tag);
    file = file.replace(&old, &new);
    file = file.replace(&close_old, &close_new);
    file
}


/// Returns a tuple of (shorthand, reference-name) for every row in the local csv file.
fn get_tags() -> Vec<(String, String)> {
    let mut res = Vec::new();
    let contents = get_file_contents("include/tags.csv".to_string());
    for row in contents.trim().split("\n") {
        let items: Vec<&str> = row.split(",").collect();
        res.push((String::from(items[0]), String::from(items[1])));
    }
    res
}
