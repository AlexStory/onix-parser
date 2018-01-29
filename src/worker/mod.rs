use std::fs::File;
use std::io::prelude::*;

pub fn replace_tags(file: String) -> String {
    let mut new_file = file;

    for tag in get_tags() {
        new_file = replace_tag(&new_file, &tag[0], &tag[1]);
    }
    new_file
}

pub fn get_file_contents(filename: &String) -> String {
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Failed to parse file");

    contents
}

pub fn write_file(contents: String, destination: &String) -> File {
    let mut new_file = File::create(destination).expect("Could not create file at destination");

    new_file
        .write_all(&contents.trim().as_bytes())
        .expect("Could not write to file");

    new_file
}

fn replace_tag(z: &str, x: &str, y: &str) -> String {
    let old = format!("<{}>", x);
    let close_old = format!("</{}>", x);
    let new = format!("<{}>", y);
    let close_new = format!("</{}>", y);
    let t = z.replace(&old, &new);
    let t = t.replace(&close_old, &close_new);
    t
}

fn get_tags() -> Vec<Vec<String>> {
    let mut res = Vec::new();
    let contents = get_file_contents(&String::from("include/tags.csv"));
    for row in contents.trim().split("\n") {
        let mut t_row = Vec::new();
        if row != "" {
            for item in row.split(",") {
                t_row.push(String::from(item));
            }
        }
        res.push(t_row);
    }
    res
}
