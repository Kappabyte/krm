use std::ops::Range;
use crate::hardware::component::Component;
use crate::hardware::bus::BusLines;

pub struct Rom {
    memory_map: Range<u16>,
    memory: Vec<u16>
}

impl Rom {
    pub fn new(memory_map_start: u16, rom: Vec<u16>) -> Self {
        return Rom {
            memory_map: Range {start: memory_map_start, end: memory_map_start + rom.len() as u16},
            memory: rom
        }
    }
}

impl Component for Rom {
    fn on_clock(&mut self, bus: &mut BusLines) {
        let address = bus.read_address();
        if(bus.write_enabled() && address < self.memory_map.end && address >= self.memory_map.start) {
            let data = self.memory.get((address - self.memory_map.start) as usize);
            if let Some(d) = data {
                bus.write_data(*d);
            }
        }
    }
}
