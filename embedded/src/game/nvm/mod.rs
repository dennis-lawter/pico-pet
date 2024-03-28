pub mod header;
pub mod inventory;
pub mod page_canon;
mod pet;
pub mod settings;

pub use self::header::NvmHeader;
use self::inventory::NvmInventory;
use self::page_canon::PageCanon;
pub use self::pet::NvmPet;
pub use self::settings::NvmSettings;

pub const NVM_BLANK: u8 = 0xff;

pub struct Nvm {
    pub parity: NvmHeader,
    pub settings: NvmSettings,
    pub inventory: NvmInventory,
    pub pet: NvmPet,
}
impl Default for Nvm {
    fn default() -> Self {
        Self {
            parity: NvmHeader::default(),
            settings: NvmSettings::default(),
            inventory: NvmInventory::default(),
            pet: NvmPet::default(),
        }
    }
}
impl Nvm {
    pub fn load_or_write_default() -> Self {
        match NvmHeader::try_load() {
            Some(parity) => {
                let new_nvm = Self {
                    parity,
                    settings: NvmSettings::load(),
                    inventory: NvmInventory::load(),
                    pet: NvmPet::load(),
                };

                new_nvm
            }
            None => {
                let mut new_nvm = Self::default();

                new_nvm.write_all();

                new_nvm
            }
        }
    }

    pub fn write_all(&mut self) {
        self.parity.write();
        self.settings.write();
        self.inventory.write();
        self.pet.write();
    }

    pub fn erase_all_then_reboot(&mut self) {
        let hardware = crate::game::globals::get_hardware();

        let blank_data_buffer = [NVM_BLANK; 8];
        for page in 0..PageCanon::PagesInUse.into() {
            hardware.write_nvm_page(page, &blank_data_buffer);
        }

        crate::game::exit::reboot();
    }
}
