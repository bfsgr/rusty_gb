use super::gpu::{*};
use super::memory::{*};
use super::cartrigbe::{*};

#[derive(Default)]
pub struct Bus {
    memory: Memory,
    gpu: GPU,
    cartrigbe: Cartrigbe,
    //everything with memory mapped I/O registers goes in here
}

pub enum State {
    Success,
    IO,
    Illegal,
}

impl Bus {
    pub fn write_byte(&mut self, addr: u16, byte: u8) -> State {

        State::Illegal
    }
}
