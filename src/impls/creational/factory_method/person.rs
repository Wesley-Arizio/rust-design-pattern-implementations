use std::fmt::Debug;
use crate::impls::creational::factory_method::traits::{CreateTool, Tool};
use crate::impls::creational::factory_method::user::User;


#[derive(Debug)]
pub struct Person<T> where T: Tool {
    pub name: String,
    pub age: u8,
    pub tool: Box<T>,
}

impl<T> ToString for Person<T> where T: Tool {
    fn to_string(&self) -> String {
        format!("name: {}; age: {}", self.name, self.age)
    }
}

impl<T> Person<T> where T: Tool {
    pub fn new(name: &str, age: u8, factory: Box<dyn CreateTool<T>>) -> Self {
        Self { name: name.into(), age, tool: factory.create_tool() }
    }
}

impl<T> User for Person<T> where T: Tool {
    fn log(&self) -> () {
        self.tool.print::<Self>(&self);
    }
}