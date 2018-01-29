extern crate xml;

use xml::worker::*;

#[test]
fn replace_tag_test() {
    let my_file = "<a>apple</a>".to_string();
    assert_eq!(replace_tag(my_file, "a", "fruit"), "<fruit>apple</fruit>");
}

#[test]
fn collect_arg_test() {
    let fake_args = vec!["rust".to_owned(), "foo".to_owned(), "bar".to_owned()];
    println!("{:?}", fake_args);

    assert_eq!(xml::collect_args(&fake_args),
               ("foo".to_owned(), "bar".to_owned()));
}
