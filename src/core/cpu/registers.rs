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
    /// # let register = RegisterV::new();
    /// assert_eq!(register.get(), 0);
    /// ````
    pub fn get(&self) -> u8 {
        self.data
    }

    /// Sets the value in this register
    /// ```
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
