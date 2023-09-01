pub enum MenuSelection {
    Item0,
    Item1,
    Item2,
    Item3,
    Item4,
    Item5,
    Item6,
    Item7,
    Item8,
    Settings,

    None,
}
impl MenuSelection {
    const MAX_VALUE: u8 = 9;
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => MenuSelection::Item0,
            1 => MenuSelection::Item1,
            2 => MenuSelection::Item2,
            3 => MenuSelection::Item3,
            4 => MenuSelection::Item4,
            5 => MenuSelection::Item5,
            6 => MenuSelection::Item6,
            7 => MenuSelection::Item7,
            8 => MenuSelection::Item8,
            9 => MenuSelection::Settings,
            _ => MenuSelection::None,
        }
    }
    pub fn to_u8(&self) -> u8 {
        match self {
            MenuSelection::Item0 => 0,
            MenuSelection::Item1 => 1,
            MenuSelection::Item2 => 2,
            MenuSelection::Item3 => 3,
            MenuSelection::Item4 => 4,
            MenuSelection::Item5 => 5,
            MenuSelection::Item6 => 6,
            MenuSelection::Item7 => 7,
            MenuSelection::Item8 => 8,
            MenuSelection::Settings => 9,
            MenuSelection::None => 255, // TODO: remove gross sentinal value
        }
    }
    pub fn next(&self) -> MenuSelection {
        let mut value = self.to_u8();
        value += 1;
        if value > Self::MAX_VALUE {
            value = 0; // loops to 0
        }
        Self::from_u8(value)
    }
    pub fn prev(&self) -> MenuSelection {
        let mut value = self.to_u8();
        if value == 0 {
            value = Self::MAX_VALUE; // loops to the MAX_VALUE
        } else {
            value -= 1;
        }
        Self::from_u8(value)
    }
}
