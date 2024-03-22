#[repr(u16)]
pub enum PageCanon {
    Header1 = 0x000,

    Settings1 = 0x001,
    Settings2 = 0x002,

    Inventory1 = 0x003,

    _Pet = 0x004,
    _Pet2 = 0x005,

    PagesInUse,
}
impl Into<u16> for PageCanon {
    fn into(self) -> u16 {
        self as u16
    }
}
