/*
    This is a video-game implementation using creational design pattern abstract factory.
    Different implementations can create a new console and controllers, using controllers to emit events and consoles to receive events.

    Abstract factory is a creational design pattern to create family of objects.
*/

use std::sync::{Arc, Mutex};
use crate::impls::creational::abstract_factory::event_emitter::EventEmitter;
use crate::impls::creational::abstract_factory::traits::{Console, ConsoleStatus, Controller, ControllerStatus};

struct PlayStation5Controller {
    status: ControllerStatus,
    console_id: String,
    event_emitter: Arc<Mutex<EventEmitter<String>>>
}

impl PlayStation5Controller {
    pub fn new(console_id: &str, event_emitter: Arc<Mutex<EventEmitter<String>>>) -> Self {
        Self {
            status: ControllerStatus::CONNECTED,
            console_id: console_id.to_string(),
            event_emitter
        }
    }
}

impl Controller<String> for PlayStation5Controller {
    fn disconnect(&mut self) -> () {
        self.status = ControllerStatus::DISCONNECTED;
    }

    fn press(&self, command: String) -> () {
        match self.status {
            ControllerStatus::DISCONNECTED => println!("Controller is not connected to the console."),
            ControllerStatus::CONNECTED => {
                self.event_emitter.lock().unwrap().emit(&self.console_id, command);
            }
        }
    }
}

struct PlayStation5 {
    status: ConsoleStatus,
    channel_name: String,
    event_emitter: Arc<Mutex<EventEmitter<String>>>,
    max_controllers: u8,
    controllers_count: u8
}

impl PlayStation5 {
    pub fn new(channel_name: String, event_emitter: Arc<Mutex<EventEmitter<String>>>) -> Self {
        let mut p = Self {
            channel_name,
            status: ConsoleStatus::OFF,
            event_emitter,
            max_controllers: 4,
            controllers_count: 0,
        };

        p.create_channel();

        p
    }

    fn create_channel(&mut self) -> () {
        match self.status {
            ConsoleStatus::OFF => {
                self.status = ConsoleStatus::ON;
                self.event_emitter.lock().unwrap().create_channel(&self.channel_name);
                self.event_emitter.lock().unwrap().on(&self.channel_name, Box::new(
                    |data| {
                        println!("[PlayStation5]: {}", data);
                }));

            },
            ConsoleStatus::ON => println!("Console is already on"),
        }
    }

    fn delete_channel(&mut self) -> () {
        match self.status {
            ConsoleStatus::OFF => println!("Console is already off"),
            ConsoleStatus::ON => {
                self.status = ConsoleStatus::OFF;
                self.event_emitter.lock().unwrap().delete_channel(&self.channel_name);
            }
        }
    }
}

impl Console<String> for PlayStation5 {
    fn power(&mut self) -> () {
        match self.status {
            ConsoleStatus::OFF => self.create_channel(), // Turn on,
            ConsoleStatus::ON => self.delete_channel() // Turn off
        }
    }

    fn status(&self) -> &ConsoleStatus {
        &self.status
    }

    fn create_controller(&mut self) -> Option<Box<dyn Controller<String>>> {
        if self.controllers_count < self.max_controllers {
            self.controllers_count += 1;
            Some(Box::new(PlayStation5Controller::new(&self.channel_name, self.event_emitter.clone())))
        } else {
            None
        }
    }
}


pub fn abstract_factory() -> () {
    let event = Arc::new(Mutex::new(EventEmitter::<String>::new()));

    let mut p1 = PlayStation5::new("channel1".to_string(), event);

    let c1 = p1.create_controller().unwrap();
    let c2 = p1.create_controller().unwrap();

    c1.press("UP".to_string());
    c2.press("DOWN".to_string());
    p1.power();
}