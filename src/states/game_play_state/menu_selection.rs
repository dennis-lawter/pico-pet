#[derive(Clone, Copy, PartialEq)]
pub enum MenuSelection {
    Feed = 0,
    Item1,
    Item2,
    Item3,
    Item4,
    Item5,
    Item6,
    Item7,
    Item8,
    Settings,
    None = 255,
}

impl MenuSelection {
    const MAX_VALUE: u8 = MenuSelection::Settings as u8;

    pub fn next(&self) -> MenuSelection {
        let value = (*self as u8).wrapping_add(1);
        if value > Self::MAX_VALUE {
            return MenuSelection::Feed;
        }
        Self::from_u8(value).unwrap_or(MenuSelection::None)
    }

    pub fn prev(&self) -> MenuSelection {
        let value = if *self == MenuSelection::Feed {
            Self::MAX_VALUE
        } else {
            (*self as u8).wrapping_sub(1)
        };
        Self::from_u8(value).unwrap_or(MenuSelection::None)
    }
}

impl From<MenuSelection> for u8 {
    fn from(item: MenuSelection) -> u8 {
        item as u8
    }
}

impl MenuSelection {
    pub fn from_u8(value: u8) -> Option<Self> {
        if value <= Self::MAX_VALUE || value == MenuSelection::None as u8 {
            Some(unsafe { core::mem::transmute(value) })
        } else {
            None
        }
    }
}
