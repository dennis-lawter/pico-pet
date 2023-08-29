pub const SETTING_MAX: u8 = 15;

pub struct Setting {
    // must be pub to allow static instantiation
    pub value: u8,
    pub step_size: u8,
}
impl Setting {
    pub fn get_value(&self) -> u8 {
        self.value
    }

    pub fn inc(&mut self) {
        self.value += self.step_size;
        if self.value > SETTING_MAX {
            self.value = SETTING_MAX;
        }
    }

    pub fn dec(&mut self) {
        self.value = self.value.saturating_sub(self.step_size);
    }

    pub fn generate_bar(&self, active: bool) -> &'static str {
        static mut BUFFER: [u8; SETTING_MAX as usize + 1] = [b'='; SETTING_MAX as usize + 1];

        unsafe {
            BUFFER[0] = b'[';
            BUFFER[SETTING_MAX as usize] = b']';
            for i in 0..=SETTING_MAX {
                if i == self.value && i > 0 {
                    if active {
                        BUFFER[i as usize] = b'5';
                    } else {
                        BUFFER[i as usize] = b'4';
                    }
                } else if i < self.value {
                    BUFFER[i as usize] = b'4';
                } else if i == SETTING_MAX {
                    BUFFER[i as usize] = b']';
                } else if i == 0 {
                    if active {
                        BUFFER[i as usize] = b'5';
                    } else {
                        BUFFER[i as usize] = b'[';
                    }
                } else {
                    BUFFER[i as usize] = b'=';
                }
            }

            core::str::from_utf8_unchecked(&BUFFER)
        }
    }
}
