extern crate formulate_trait;
#[macro_use]
extern crate formulate_derive;

use formulate_trait::Formulate;

#[derive(Formulate)]
struct MyStruct {
    one: u8,
    two: u8,
    three: u8,
}

#[test]
fn it_works() {
    assert_eq!(MyStruct::formulate(), vec!["one", "two", "three"]);
}
