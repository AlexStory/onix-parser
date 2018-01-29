use std;
use std::fs::File;
use std::io::prelude::*;

/// Replace each tag in file with appropriate replacement.
pub fn replace_tags(mut file: String, xml_tags: String) -> String {
    for (old, new) in get_tags(xml_tags) {
        file = replace_tag(file, &old, &new);
    }
    file
}

/// Takes a path(filename) and returns the string contents of the file.
pub fn get_file_contents(filename: String) -> String {
    let mut file = match File::open(filename) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Could not find target file.");
            std::process::exit(3);
        }
    };
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect(
        "Failed to parse file",
    );

    contents
}

/// Save [contents] to a file at [destination].
pub fn write_file(contents: String, destination: String) -> File {
    let mut new_file = File::create(destination).expect("Could not create file at destination");

    new_file.write_all(&contents.trim().as_bytes()).expect(
        "Could not write to file",
    );

    new_file
}

/// Replace all instances in the file , of old_tag withe a new_tag.
pub fn replace_tag(file: String, old_tag: &str, new_tag: &str) -> String {
    let old = format!("<{}>", old_tag);
    let close_old = format!("</{}>", old_tag);
    let new = format!("<{}>", new_tag);
    let close_new = format!("</{}>", new_tag);
    let solo_old = format!("<{} />", old_tag);
    let solo_new = format!("<{} />", new_tag);

    file.replace(&old, &new)
        .replace(&close_old, &close_new)
        .replace(&solo_old, &solo_new)
        .replace("ONIXmessage", "ONIXMessage")
}


/// Returns a tuple of (shorthand, reference-name) for every row in the local csv file.
fn get_tags(location: String) -> Vec<(String, String)> {
    let mut res = Vec::new();
    let contents = get_file_contents(location);
    for row in contents.trim().lines() {
        let items: Vec<&str> = row.split(",").collect();
        res.push((String::from(items[0]), String::from(items[1])));
    }
    res
}
