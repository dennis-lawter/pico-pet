use self::garden_inventory::GardenInventory;

pub mod garden_inventory;

pub struct NvmInventory {
    pub seed_inventory: GardenInventory,
}
impl Default for NvmInventory {
    fn default() -> Self {
        Self {
            seed_inventory: GardenInventory::default(),
        }
    }
}
impl NvmInventory {
    pub fn load() -> Self {
        Self {
            seed_inventory: GardenInventory::load(),
        }
    }
    pub fn write(&self) {
        self.seed_inventory.write()
    }
}
