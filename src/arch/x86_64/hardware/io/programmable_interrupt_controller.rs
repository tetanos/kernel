use super::ProgrammedIO;
use crate::common::io::IO;

pub static mut MASTER: ProgrammableInterruptController =
    ProgrammableInterruptController::new(Type::Master);
pub static mut SLAVE: ProgrammableInterruptController =
    ProgrammableInterruptController::new(Type::Slave);

pub unsafe fn init() {
    // init magic
    MASTER.command.write(Command::Initialize as u8);
    SLAVE.command.write(Command::Initialize as u8);

    // offsets
    MASTER.data.write(0x20);
    SLAVE.data.write(0x28);

    // cascade
    MASTER.data.write(4);
    SLAVE.data.write(2);

    // interrupt mode to 8086/88 mode
    MASTER.data.write(Mode::Intel8086 as u8);
    SLAVE.data.write(Mode::Intel8086 as u8);

    // unmask interrupts
    MASTER.data.write(0);
    SLAVE.data.write(0);

    // acknowledge remaining interrupts
    MASTER.acknowledge();
    SLAVE.acknowledge();
}

pub struct ProgrammableInterruptController {
    command: ProgrammedIO<u8>,
    data: ProgrammedIO<u8>,
}

impl ProgrammableInterruptController {
    pub const fn new(controller_type: Type) -> Self {
        let port = controller_type as u16;
        ProgrammableInterruptController {
            command: ProgrammedIO::new(port),
            data: ProgrammedIO::new(port + 1),
        }
    }

    pub fn acknowledge(&mut self) {
        self.command.write(Command::Acknowledge as u8);
    }

    pub fn mask_set(&mut self, irq: u8) {
        let value = self.data.read() | 1 << irq;
        self.data.write(value);
    }

    pub fn mask_unset(&mut self, irq: u8) {
        let value = self.data.read() & !(1 << irq);
        self.data.write(value);
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum Command {
    Initialize = 0x11,
    Acknowledge = 0x20,
}

#[derive(Copy, Clone, Debug)]
#[repr(u16)]
pub enum Type {
    Master = 0x20,
    Slave = 0xa0,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
enum Mode {
    Intel8086 = 1,
    AutoEndOfInterrupt = 2,
}
