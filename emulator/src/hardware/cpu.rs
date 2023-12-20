use crate::hardware::bus::BusLines;
use crate::hardware::component::Component;
use std::num::Wrapping;

#[derive(PartialEq)]
enum CPUState {
    Fetch, Execute
}

pub struct CPU {
    flags: u8,
    general_registers: [u16; 14],
    stack_pointer_register: u16,
    instruction_pointer: Wrapping<u16>,
    zero_register: u16,
    cycles: u8,
    state: CPUState,
    current_instruction: u16
}

impl CPU {

    pub fn new() -> Self {
        return CPU {
            flags: 0,
            general_registers: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            stack_pointer_register: 0,
            instruction_pointer: Wrapping(0),
            zero_register: 0,
            cycles: 0,
            state: CPUState::Fetch,
            current_instruction: 0
        }
    }

    fn nop(&mut self) {
        self.cycles = 0;
        self.instruction_pointer += 1;
        return;
    }
    fn ldr(&mut self, mut bus: BusLines, rn: u8, rm: u8) {
        if self.cycles == 0 {
            self.cycles = 3; // This instruction takes 3 CPU cycles
            let address: u16 = self.general_registers[rm as usize]; // Read the address from the
                                                                    // register
            bus.write_address(address); // Write the address to the bus
            bus.write(); // Tells the device to write data to the bus
        }
    }
    fn str(&mut self, mut bus: BusLines, rn: u8, rm: u8){
        if self.cycles == 0 {
            self.cycles = 2; // This instruction takes 2 CPU cycles
            let data: u16 = self.general_registers[rn as usize]; // Read the data from the register
            let address: u16 = self.general_registers[rm as usize]; // Read the address from the
                                                                    // register
            bus.write_data(data); // Write the data to the bus
            bus.write_address(address); // Write the address to the bus
            bus.read(); // Tells the device to read the data on the bus and store its value
            self.cycles -= 1; // The CPU cycle has concluded
        } else if self.cycles == 1 {
            // wait for the device to read data from the bus
            self.cycles -= 1;
            self.instruction_pointer += 1;
        }
    }
    // fn add_r(rn: u8, rm: u8){}
    // fn add_i(rn: u8, imm: u8){}
    // fn sub_r(rn: u8, rm: u8){}
    // fn sub_i(rn: u8, rm: u8){}
    // fn lsl(rn: u8, imm: u8){}
    // fn lsr(rn: u8, imm: u8){}
    // fn and(rn: u8, rm: u8){}
    // fn or(rn: u8, rm: u8){}
    // fn xor(rn: u8, rm: u8){}
    // fn cmp(rn: u8, rm: u8){}
    fn j(&mut self, c: u8, imm: u16){
        let is_negative: bool = (imm & 0x100) >> 8 == 1; // Check if the sign bit of the 9 bit immediate
                                                  // value is set
        println!("Is Negative: {}", is_negative);
        println!("Value: {:#x}", imm);
        match is_negative {
            true => self.instruction_pointer -= !(imm | 0xff00)+1, // calculate the two's
                                                                   // complement 
            false => self.instruction_pointer += imm & 0x00FF
        }
    }
    // fn ret(){}
}

impl Component for CPU {
    fn on_clock(&mut self, bus: &mut BusLines) {
        if self.state == CPUState::Fetch {
            if self.cycles == 0 {
                self.cycles = 3; // It takes 3 cycles to fetch an instruction_pointer
                println!("Address: {}", self.instruction_pointer.0);
                bus.write_address(self.instruction_pointer.0);
                bus.write(); // Tell the BUS device to write to the BUS
                self.cycles -= 1; // This cycle is done
            } else if self.cycles == 2 {
                // Wait for the BUS device
                self.cycles -= 1;
            } else if self.cycles == 1 {
                self.current_instruction = bus.read_data();
                self.cycles -= 1;
                self.state = CPUState::Execute
            }
        } else if self.state == CPUState::Execute {
            // Decode the instruction
            let instruction_code = (self.current_instruction & 0xF000) >> 12;
            println!("Code: {:#x}", instruction_code);
            match instruction_code {
                0x0 => self.nop(),
                0xE => {
                    let c = (self.current_instruction & 0x0E00) >> 9;
                    let imm = self.current_instruction & 0x01FF;
                    self.j(c.try_into().unwrap(), imm);
                }
                0xD => {}
                _ => {
                    println!("Not Implemented {:#x}", instruction_code);
                    self.instruction_pointer += 1;
                }
            }
            if self.cycles <= 0 {
                self.state = CPUState::Fetch;
                bus.write_data(0);
                bus.disable();
            }
        }
    }
}
