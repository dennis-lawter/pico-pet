pub enum PageCanon {
    Header = 0x000,
    Settings,
    SeedInventory,
    VegetableInventory,

    PagesInUse,
}
impl Into<u16> for PageCanon {
    fn into(self) -> u16 {
        self as u16
    }
}
