extern crate formulate_trait;
#[macro_use]
extern crate formulate_derive;

use formulate_trait::Formulate;

#[derive(Formulate)]
struct Pancakes;

#[test]
fn it_works() {
    assert_eq!(Pancakes::formulate(), "Hello, Macro! My name is Pancakes");
}
