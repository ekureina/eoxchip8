use std::fmt::{Display, Formatter};

use crate::core::memory::Address;

#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct RegisterV {
    data: u8,
}

impl RegisterV {
    #[must_use]
    pub fn new() -> Self {
        RegisterV::default()
    }

    /// Gets the value in this register
    /// ```
    /// # use rc8::core::cpu::registers::RegisterV;
    /// # let register = RegisterV::new();
    /// assert_eq!(register.get(), 0);
    /// ````
    #[must_use]
    pub fn get(&self) -> u8 {
        self.data
    }

    /// Sets the value in this register
    /// ```
    /// # use rc8::core::cpu::registers::RegisterV;
    /// # let mut register = RegisterV::new();
    /// register.set(10);
    /// # assert_eq!(register.get(), 10);
    pub fn set(&mut self, data: u8) {
        self.data = data;
    }

    /// Adds to the value in this register
    pub fn add(&mut self, value: u8) -> bool {
        let true_result = u16::from(self.data) + u16::from(value);
        self.data = (true_result & 0x00FF) as u8;
        (true_result & 0xFF00) != 0
    }
}

impl Display for RegisterV {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct RegisterI {
    data: u16,
}

impl RegisterI {
    #[must_use]
    pub fn new() -> Self {
        RegisterI::default()
    }

    /// Gets the value in this register
    /// ```
    /// # use rc8::core::cpu::registers::RegisterI;
    /// # let register = RegisterI::new();
    /// assert_eq!(register.get(), 0);
    /// ````
    #[must_use]
    pub fn get(&self) -> u16 {
        self.data
    }

    /// Sets the value in this register
    /// ```
    /// # use rc8::core::cpu::registers::RegisterI;
    /// # let mut register = RegisterI::new();
    /// register.set(10);
    /// # assert_eq!(register.get(), 10);
    pub fn set(&mut self, data: u16) {
        self.data = data;
    }
}

impl Display for RegisterI {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct RegisterPC {
    data: Address,
}

impl RegisterPC {
    #[must_use]
    pub fn new() -> Self {
        RegisterPC::default()
    }

    pub fn inc(&mut self) {
        self.data.0 += 2;
    }

    #[must_use]
    pub fn get(&self) -> Address {
        self.data
    }

    pub fn set(&mut self, new_address: Address) {
        self.data = new_address;
    }
}

impl Default for RegisterPC {
    fn default() -> Self {
        RegisterPC {
            data: Address(0x200),
        }
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct RegisterF {
    carry: bool,
}

impl RegisterF {
    #[must_use]
    pub fn new() -> Self {
        RegisterF::default()
    }

    pub fn set_carry(&mut self, value: bool) {
        self.carry = value;
    }
}
