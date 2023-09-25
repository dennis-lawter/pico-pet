use self::seed_inventory::SeedInventory;

pub mod seed_inventory;

pub struct Inventory {
    pub seed_inventory: SeedInventory,
}
impl Default for Inventory {
    fn default() -> Self {
        Self {
            seed_inventory: SeedInventory::default(),
        }
    }
}
