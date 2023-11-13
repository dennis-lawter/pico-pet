use self::seed_inventory::SeedInventory;

pub mod seed_inventory;

pub struct NvmInventory {
    pub seed_inventory: SeedInventory,
}
impl Default for NvmInventory {
    fn default() -> Self {
        Self {
            seed_inventory: SeedInventory::default(),
        }
    }
}
impl NvmInventory {
    pub fn load() -> Self {
        Self {
            seed_inventory: SeedInventory::load(),
        }
    }
    pub fn write(&self) {
        self.seed_inventory.write()
    }
}
