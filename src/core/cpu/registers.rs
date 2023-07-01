use std::fmt::{Display, Formatter};

#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct RegisterV {
    data: u8,
}

impl RegisterV {
    pub fn new() -> Self {
        RegisterV::default()
    }

    /// Gets the value in this register
    /// ```
    /// # use rc8::core::cpu::registers::RegisterV;
    /// # let register = RegisterV::new();
    /// assert_eq!(register.get(), 0);
    /// ````
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
    pub fn new() -> Self {
        RegisterI::default()
    }

    /// Gets the value in this register
    /// ```
    /// # use rc8::core::cpu::registers::RegisterI;
    /// # let register = RegisterI::new();
    /// assert_eq!(register.get(), 0);
    /// ````
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
