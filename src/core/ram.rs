use crate::{
    error::Result,
    io::{Read, Write},
    kb,
};
use std::fmt::Display;

pub const RAM_SIZE: usize = kb!(2);

#[derive(Debug)]
pub struct Ram([u8; RAM_SIZE]);

impl Ram {
    pub fn load(&mut self, offset: u16, data: &[u8]) {
        self.0[offset as usize..(offset as usize + data.len())].copy_from_slice(data);
    }

    pub fn dump(&self) -> [u8; RAM_SIZE] {
        self.0
    }
}

impl Default for Ram {
    fn default() -> Self {
        Self([0x00; RAM_SIZE])
    }
}

impl Read for Ram {
    fn read_byte(&mut self, addr: u16) -> Result<u8> {
        Ok(self.0[addr as usize])
    }
}

impl Write for Ram {
    fn write_byte(&mut self, addr: u16, byte: u8) -> Result<()> {
        self.0[addr as usize] = byte;

        Ok(())
    }
}

impl AsRef<[u8; RAM_SIZE]> for Ram {
    fn as_ref(&self) -> &[u8; RAM_SIZE] {
        &self.0
    }
}

impl Display for Ram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:#}]",
            self.0
                .iter()
                .fold(String::new(), |acc, byte| format!("{acc} {byte}"))
        )
    }
}
