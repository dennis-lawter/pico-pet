use core::{convert::TryFrom, fmt::Display};

use crate::{
    display::text_writer::{self, FontStyle},
    nvm::page_canon::PageCanon,
};

// const NVM_SETTINGS_PAGE: u16 = 0x002;

pub enum Seed {
    Cuke = 0,
    Corn,
    Onion,
    Tater,
    Carrot,
    Spinach,
    Mater,
    Pump,

    COUNT,
}
impl TryFrom<u8> for Seed {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == 255 || value <= (Seed::COUNT as u8) {
            Ok(unsafe { core::mem::transmute(value) })
        } else {
            Err(())
        }
    }
}
impl Display for Seed {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Seed::Cuke => write!(f, "Cucumber"),
            Seed::Corn => write!(f, "Corn    "),
            Seed::Onion => write!(f, "Onion   "),
            Seed::Tater => write!(f, "Potato  "),
            Seed::Carrot => write!(f, "Carrot  "),
            Seed::Spinach => write!(f, "Spinach "),
            Seed::Mater => write!(f, "Tomato  "),
            Seed::Pump => write!(f, "Pumpkin "),
            Seed::COUNT => write!(f, "        "),
        }
    }
}

#[derive(Debug)]
struct SeedCountData(u8);
impl SeedCountData {
    const MAX: u8 = 99;
    const DISCOVERY_BIT: u8 = 0b1000_0000;
    const QTY_BITS: u8 = 0b0111_1111;
    fn new(qty: u8) -> Self {
        if qty == 0 {
            Self(0)
        } else {
            let qty = if qty > Self::MAX { Self::MAX } else { qty };
            Self(qty | Self::DISCOVERY_BIT)
        }
    }
    fn is_discovered(&self) -> bool {
        self.0 & Self::DISCOVERY_BIT != 0
    }
    fn qty(&self) -> u8 {
        self.0 & Self::QTY_BITS
    }
    fn discover(&mut self) {
        self.0 |= Self::DISCOVERY_BIT;
    }
    fn add(&mut self, x: u8) {
        let mut qty = self.qty();
        let x = if x >= Self::MAX { Self::MAX } else { x };

        qty += x;
        if qty > Self::MAX {
            qty = Self::MAX;
        }

        self.0 = qty;

        self.discover()
    }
    fn sub(&mut self, x: u8) {
        let mut qty = self.qty();
        qty = qty.saturating_sub(x);
        self.0 = qty;

        self.discover()
    }
}

pub struct SeedInventory {
    data: [SeedCountData; Seed::COUNT as usize - 1], // exclude cuke
}
impl Default for SeedInventory {
    fn default() -> Self {
        let _rng = crate::globals::get_rng();
        Self {
            data: [
                // For testing
                // SeedCountData::new((rng.next() / 2).saturating_sub(14)),
                // SeedCountData::new((rng.next() / 2).saturating_sub(14)),
                // SeedCountData::new((rng.next() / 2).saturating_sub(14)),
                // SeedCountData::new((rng.next() / 2).saturating_sub(14)),
                // SeedCountData::new((rng.next() / 2).saturating_sub(14)),
                // SeedCountData::new((rng.next() / 2).saturating_sub(14)),
                // SeedCountData::new((rng.next() / 2).saturating_sub(14)),
                SeedCountData::new(0),
                SeedCountData::new(0),
                SeedCountData::new(0),
                SeedCountData::new(0),
                SeedCountData::new(0),
                SeedCountData::new(0),
                SeedCountData::new(0),
            ],
        }
    }
}
impl SeedInventory {
    pub fn load() -> Self {
        let hardware = crate::globals::get_hardware();
        let data_u8s = hardware.get_nvm_page(PageCanon::SeedInventory.into());
        let data = [
            SeedCountData::new(data_u8s[0]),
            SeedCountData::new(data_u8s[1]),
            SeedCountData::new(data_u8s[2]),
            SeedCountData::new(data_u8s[3]),
            SeedCountData::new(data_u8s[4]),
            SeedCountData::new(data_u8s[5]),
            SeedCountData::new(data_u8s[6]),
        ];
        Self { data: data }
    }
    pub fn save(&self) {
        todo!()
    }
    pub fn display(&self, x: i32, y: i32) {
        let text = fixedstr::str_format!(fixedstr::str24, "{} x  ", Seed::Cuke);
        text_writer::draw_text(x, y, FontStyle::Small, 0b000_000_00, &text);
        text_writer::draw_text(x + 50, y, FontStyle::Icon, 0b000_000_00, "pq"); // infinity

        for i in 1..Seed::COUNT as u8 {
            let seed = Seed::try_from(i).unwrap();
            let data = &self.data[i as usize - 1]; // exclude cuke
            let text = if data.is_discovered() {
                fixedstr::str_format!(fixedstr::str24, "{} x\\c060{}", seed, data.qty())
            } else {
                fixedstr::str24::from("???????? x00")
            };
            text_writer::draw_text(x, y + (i as i32 * 8), FontStyle::Small, 0b000_000_00, &text);
        }
    }
}
