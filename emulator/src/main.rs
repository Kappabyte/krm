mod hardware;
use hardware::system::System;
use hardware::cpu::CPU;
use hardware::rom::Rom;

fn main() {
    let system: &mut System = &mut System::new();
    system.add_component(Box::new(CPU::new()));
    let rom: Rom = Rom::new(0, vec!(0xE004, 0x0000, 0x0000, 0x0000, 0xE3FC ));
    system.add_component(Box::new(rom));
    system.start();
}
