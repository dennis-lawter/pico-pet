pub enum PageCanon {
    Header = 0x000,
    Settings = 0x002, // moved past bad sector...

    PagesInUse,
}
impl Into<u16> for PageCanon {
    fn into(self) -> u16 {
        self as u16
    }
}
