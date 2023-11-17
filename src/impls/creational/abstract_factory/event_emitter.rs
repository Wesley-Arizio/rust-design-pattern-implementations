use std::collections::HashMap;

/*
    This is a small implementation of a EventEmitter to manage events.
    With this struct, it's possible to create channels and emit and receive events.
*/

type EventCallback<T> = Box<dyn Fn(T)>;

pub struct EventEmitter<T> {
    channels: HashMap<String, Vec<EventCallback<T>>>,
}

impl<T: Clone> EventEmitter<T> {
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
        }
    }

    pub fn create_channel(&mut self, channel_name: &str) {
        self.channels.insert(channel_name.to_string(), Vec::new());
    }

    pub fn delete_channel(&mut self, channel_name: &str) {
        self.channels.remove(channel_name);
    }

    pub fn on(&mut self, channel_name: &str, listener: EventCallback<T>) {
        if let Some(channel) = self.channels.get_mut(channel_name) {
            channel.push(listener);
        }
    }

    pub fn emit(&self, channel_name: &str, data: T) {
        if let Some(channel) = self.channels.get(channel_name) {
            for listener in channel {
                listener(data.clone());
            }
        }
    }
}
