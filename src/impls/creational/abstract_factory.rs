pub mod event_emitter;
mod ps5;
mod traits;

/*
    This is a video-game implementation using creational design pattern abstract factory.
    Different implementations can create a new console and controllers, using controllers to emit events and consoles to receive events.

    Abstract factory is a creational design pattern to create family of objects.
*/

use super::abstract_factory::{event_emitter::EventEmitter, ps5::PlayStation5, traits::Console};
use std::sync::{Arc, Mutex};

pub fn abstract_factory() -> () {
    let event = Arc::new(Mutex::new(EventEmitter::<String>::new()));

    let mut p1 = PlayStation5::new("channel1".to_string(), event);

    let c1 = p1.create_controller().unwrap();
    let c2 = p1.create_controller().unwrap();

    c1.press("UP".to_string());
    c2.press("DOWN".to_string());
    p1.power();
}
