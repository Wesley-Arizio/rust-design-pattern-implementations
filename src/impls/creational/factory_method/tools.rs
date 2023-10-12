use crate::impls::creational::factory_method::traits::{CreateTool, Tool};

pub struct ToolA {
    color: String,
}

impl ToolA {
    fn new(color: &str) -> Self {
        Self {
            color: color.into()
        }
    }
}

// Concrete implementation of a product (a tool for logging using technology A)
impl Tool for ToolA {
    fn print<T: ToString>(&self, content: &T) -> () {
        // TODO - use any third party library to style the logs
        println!("The font color for this implementation should be '{}'", self.color);
        println!("{:#?}", content.to_string());
    }
}

pub struct CreateToolA {
    color: String,
}

impl CreateToolA {
    pub fn new(color: &str) -> Box<Self> {
        Box::new(Self {
            color: color.into()
        })
    }
}

// Concrete implementation of the factory that creates a Product (a new tool)
impl CreateTool<ToolA> for CreateToolA {
    fn create_tool(&self) -> Box<ToolA> {
        Box::new(ToolA::new(&self.color))
    }
}

pub struct ToolB {
    color: String,
}
impl ToolB {
    fn new(color: &str) -> Self {
        Self {
            color: color.into()
        }
    }
}

// Another concrete implementation of a product (a tool for logging using technology B)
impl Tool for ToolB {
    fn print<T: ToString>(&self, content: &T) -> () {
        // TODO - use any third party library to style the logs
        println!("The font color for this implementation should be '{}'", self.color);
        println!("{:#?}", content.to_string());
    }
}

pub struct CreateToolB {
    color: String,
}

impl CreateToolB {
    pub fn new(color: &str) -> Box<Self> {
        Box::new(Self {
            color: color.into()
        })
    }
}

// Another concrete implementation of the factory that creates a new product (a tool for logging)
impl CreateTool<ToolB> for CreateToolB {
    fn create_tool(&self) -> Box<ToolB> {
        Box::new(ToolB::new(&self.color))
    }
}
