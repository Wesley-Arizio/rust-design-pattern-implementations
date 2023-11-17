// Trait to print information in the console.
pub trait Tool {
    fn print<T: ToString>(&self, content: &T) -> ();
}

// Trait to initialize the logger tool
pub trait CreateTool<T>
where
    T: Tool,
{
    fn create_tool(&self) -> Box<T>;
}
