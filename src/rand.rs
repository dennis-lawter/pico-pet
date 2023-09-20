pub struct Lcg {
    state: u32,
}

impl Lcg {
    const M: u32 = (1 << 16) + 1;
    const A: u32 = 75;
    const C: u32 = 74;

    pub fn new(seed: u32) -> Self {
        Self { state: seed }
    }
    pub fn next(&mut self) -> u8 {
        let next = self.state * Self::A + Self::C;
        let next = next % Self::M;
        self.state = next;

        // isolate bits 7..0
        let output = next & 0xFF;

        output as u8
    }
}

impl Default for Lcg {
    fn default() -> Self {
        let hardware = crate::globals::get_hardware();
        let time = hardware.get_time();
        let seed = time.hr as u32 * 60 * 60 + time.min as u32 * 60 + time.sec as u32;

        Self::new(seed)
    }
}
