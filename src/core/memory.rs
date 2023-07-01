use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct Address(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct Ram {
    data: [u8; 4096],
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Error)]
pub enum MemoryAccessError {
    #[error("Address out of Bounds: {0:?}")]
    AddressOutOfBounds(Address),
    #[error("Address unaligned for access: {0:?}")]
    AddressUnaligned(Address),
}

type MemoryResult<T> = Result<T, MemoryAccessError>;

impl Ram {
    /// Create a new stick of Chip-8 RAM
    /// ```
    /// # use rc8::core::memory::*;
    /// let ram = Ram::new();
    /// # for addr in 0x200..4096 {
    /// # assert_eq!(ram.get(Address(addr)), Ok(0));
    /// }
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Ram::default()
    }

    pub fn get(&self, address: Address) -> MemoryResult<u8> {
        if address.0 as usize >= self.data.len() {
            return Err(MemoryAccessError::AddressOutOfBounds(address));
        }

        Ok(self.data[address.0 as usize])
    }

    pub fn get_wide(&self, address: Address) -> MemoryResult<u16> {
        if address.0 % 2 != 0 {
            return Err(MemoryAccessError::AddressUnaligned(address));
        }

        let mut data = u16::from(self.get(address)?) << 8;
        data |= u16::from(self.get(Address(address.0 + 1))?);

        Ok(data)
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn load_program(&mut self, program: &[u8]) -> MemoryResult<()> {
        if 0x200 + program.len() > self.data.len() {
            return Err(MemoryAccessError::AddressOutOfBounds(Address(
                0x200 + program.len() as u16,
            )));
        }
        for (offset, byte) in program.iter().enumerate() {
            self.data[0x200 + offset] = *byte;
        }
        Ok(())
    }
}

impl Default for Ram {
    fn default() -> Self {
        Ram { data: [0; 4096] }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct Chip8Display {
    data: [[bool; 32]; 64],
}

impl Chip8Display {
    #[must_use]
    pub fn new() -> Self {
        Chip8Display::default()
    }

    /// Clears the Chip8's display
    pub fn clear(&mut self) {
        self.data = [[false; 32]; 64];
    }

    /// Flips a pixel in the Chip8's display
    pub fn flip_pixel(&mut self, x: u8, y: u8) {
        self.data[x as usize][y as usize] ^= true;
    }

    /// Gets a reference to the Chip8's display memory
    #[must_use]
    pub fn get(&self) -> &[[bool; 32]] {
        &self.data
    }
}

impl Default for Chip8Display {
    fn default() -> Self {
        Chip8Display {
            data: [[false; 32]; 64],
        }
    }
}

impl std::fmt::Display for Chip8Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for column in &self.data {
            for pixel in column {
                if *pixel {
                    write!(f, "█")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[must_use]
pub fn memory_to_flip_instructions(memory: &[u8]) -> Vec<Vec<bool>> {
    memory
        .iter()
        .map(|byte| byte_to_flip_instructions(*byte))
        .collect()
}

fn byte_to_flip_instructions(byte: u8) -> Vec<bool> {
    let mut mask = 0x80;
    let mut result = vec![];
    for _ in 0..u8::BITS {
        result.push((byte & mask) != 0);
        mask >>= 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_memory_in_bounds() {
        let ram = Ram::new();
        assert_eq!(ram.get(Address(0x200)), Ok(0));
    }

    #[test]
    fn test_get_memory_high_address() {
        let ram = Ram::new();
        assert_eq!(
            ram.get(Address(4097)),
            Err(MemoryAccessError::AddressOutOfBounds(Address(4097)))
        );
    }

    #[test]
    fn test_get_memory_ram_boundary() {
        let ram = Ram::new();
        assert_eq!(
            ram.get(Address(4096)),
            Err(MemoryAccessError::AddressOutOfBounds(Address(4096)))
        );
        assert_eq!(ram.get(Address(4095)), Ok(0));
    }

    #[test]
    fn test_load_short_program() {
        let mut ram = Ram::new();
        let program = [0x10, 0x20, 0x30, 0x40];
        ram.load_program(&program).unwrap();
        assert_eq!(ram.data[0x200], 0x10);
        assert_eq!(ram.data[0x201], 0x20);
        assert_eq!(ram.data[0x202], 0x30);
        assert_eq!(ram.data[0x203], 0x40);
    }

    #[test]
    fn test_load_long_program() {
        let mut ram = Ram::new();
        let program = [0x20; (u16::MAX - 0x200) as usize];
        assert_eq!(
            ram.load_program(&program),
            Err(MemoryAccessError::AddressOutOfBounds(Address(u16::MAX)))
        );
    }

    #[test]
    fn test_flip_pixels_display() {
        let mut display = Chip8Display::new();
        display.flip_pixel(0, 0);
        assert!(display.get()[0][0]);
        display.flip_pixel(0, 0);
        assert!(!display.get()[0][0]);
    }

    #[test]
    fn test_clear_display() {
        let mut display = Chip8Display::new();
        display.flip_pixel(0, 0);
        display.flip_pixel(0, 1);
        assert!(display.get()[0][0]);
        assert!(display.get()[0][1]);
        display.clear();
        assert!(!display.get()[0][0]);
        assert!(!display.get()[0][1]);
    }

    #[test]
    #[should_panic]
    fn test_display_out_of_bounds_access_x() {
        let mut display = Chip8Display::new();
        display.flip_pixel(64, 0);
    }

    #[test]
    #[should_panic]
    fn test_display_out_of_bounds_access_y() {
        let mut display = Chip8Display::new();
        display.flip_pixel(0, 32);
    }
}
