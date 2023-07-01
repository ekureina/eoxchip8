use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct Address(u16);

#[derive(Debug, Clone, Copy)]
pub struct Ram {
    data: [u8; 4096],
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Error)]
pub enum MemoryAccessError {
    #[error("Address out of Bounds: {0:?}")]
    AddressOutOfBounds(Address),
}

type MemoryResult<T> = Result<T, MemoryAccessError>;

impl Ram {
    /// Create a new stick of Chip-8 RAM
    /// ```
    /// let ram = Ram::new();
    /// # assert_eq!(ram.data, [0; 4096]);
    /// ```
    pub fn new() -> Self {
        Ram::default()
    }

    pub fn get(&self, address: Address) -> MemoryResult<u8> {
        if address.0 < 0x200 {
            return Err(MemoryAccessError::AddressOutOfBounds(address));
        }

        if address.0 as usize >= self.data.len() {
            return Err(MemoryAccessError::AddressOutOfBounds(address));
        }

        Ok(self.data[address.0 as usize])
    }
}

impl Default for Ram {
    fn default() -> Self {
        Ram { data: [0; 4096] }
    }
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
    fn test_get_memory_low_address() {
        let ram = Ram::default();
        assert_eq!(
            ram.get(Address(0)),
            Err(MemoryAccessError::AddressOutOfBounds(Address(0)))
        );
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
    fn test_get_memory_interpreter_boundary() {
        let ram = Ram::new();
        assert_eq!(
            ram.get(Address(0x1FF)),
            Err(MemoryAccessError::AddressOutOfBounds(Address(0x1FF)))
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
}
