use crate::hardware::component::Component;

pub struct BusLines {
    address: u16,
    data: u16,
    read_enable: bool,
    write_enable: bool
}

pub struct Bus {
    bus_lines: BusLines,
    connected_components: Vec<Box<dyn Component>>
}

impl BusLines {
    pub fn write_address(&mut self, address: u16) {
        self.address = address;
    }
    pub fn write_data(&mut self, data: u16) {
        self.data = data;
    }
    pub fn read(&mut self) {
        self.write_enable = false;
        self.read_enable = true;
    }
    pub fn write(&mut self) {
        self.read_enable = false;
        self.write_enable = true;
    }
    pub fn disable(&mut self) {
        self.read_enable = false;
        self.write_enable = false;
    }
 
    pub fn read_address(&self) -> u16 {
        return self.address;
    }

    pub fn read_data(&self) -> u16 {
        return self.data;
    }

    pub fn write_enabled(&self) -> bool {
        return self.write_enable;
    }

    pub fn read_enabled(&self) -> bool {
        return self.read_enable;
    }

}

impl Bus {
    pub fn new() -> Self {
        return Bus {
            bus_lines: BusLines {
                address: 0,
                data: 0,
                read_enable: false,
                write_enable: false
            },
            connected_components: vec!()
        }
    }

    pub fn add_component(&mut self, component: Box<dyn Component>) {
        self.connected_components.push(component);
    }
   
    pub fn tick(&mut self) {
        for i in 0..self.connected_components.len() {
            self.connected_components[i].on_clock(&mut self.bus_lines);
        }
    }
}
