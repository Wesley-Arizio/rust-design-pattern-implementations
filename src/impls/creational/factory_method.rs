/*
    Factory method:
        Usage of an abstract class to create objects that has similar functionalities (follow another abstract class or implements a interface)
        It resolves the problem of creating objects without specifying the concrete implementation.

    In this small application, a Person struct has the ability to log a content into the console.
    Using structs and traits, we separate the concrete implementation from the interfaces, from the product (Tool) and it's own creator (factory).
*/
pub mod person;
pub mod tools;
pub mod traits;
pub mod user;

use crate::impls::creational::factory_method::person::Person;
use crate::impls::creational::factory_method::tools::{CreateToolA, CreateToolB};
use crate::impls::creational::factory_method::user::User;

#[derive(Debug)]
pub enum Tools {
    ToolA,
    ToolB,
}

pub fn tools_factory_method() -> () {
    let tool_a = CreateToolA::new("red");
    let person = Person::new("wesley", 23u8, tool_a);

    person.log();
    // Output
    // The font color for this implementation should be 'red'
    // "name: wesley; age: 23"

    let tool_b = CreateToolB::new("green");
    let person = Person::new("enrique", 19, tool_b);

    person.log();
    // Output
    // The font color for this implementation should be 'green'
    // "name: enrique; age: 19"
}
