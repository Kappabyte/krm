use crate::hardware::bus::BusLines;

pub trait Component {
    fn on_clock(&mut self, bus: &mut BusLines);
}
