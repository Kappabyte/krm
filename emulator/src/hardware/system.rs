use crate::hardware::bus::Bus;
use crate::hardware::component::Component;

pub struct System {
    bus: Bus
}

impl System {
    pub fn new() -> Self {
        return System {
            bus: Bus::new()
        }
    }

    pub fn add_component(&mut self, component: Box<dyn Component>) {
        self.bus.add_component(component);
    }

    pub fn start(&mut self) {
        loop {
            self.bus.tick();
        }
    }
}
