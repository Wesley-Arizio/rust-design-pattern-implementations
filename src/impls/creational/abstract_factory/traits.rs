#[derive(Debug)]
pub enum ConsoleStatus {
    ON,
    OFF,
}

#[derive(Debug)]
pub enum ControllerStatus {
    CONNECTED,
    DISCONNECTED,
}

pub trait Controller<C> {
    fn disconnect(&mut self) -> ();
    fn press(&self, command: C) -> ();
}

pub trait Console<C> {
    fn power(&mut self) -> ();
    fn status(&self) -> &ConsoleStatus;
    fn create_controller(&mut self) -> Option<Box<dyn Controller<C>>>;
}
