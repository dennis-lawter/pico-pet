const SETTING_BAR_SIZE: usize = 15;

pub struct Setting {
    // must be pub to allow static instantiation
    pub value: u8,
    pub max_value: u8,
}
impl Setting {
    pub fn get_value(&self) -> u8 {
        self.value
    }

    pub fn inc(&mut self) {
        self.value += 1;
        if self.value > self.max_value {
            self.value = self.max_value;
        }
    }

    pub fn dec(&mut self) {
        self.value = self.value.saturating_sub(1);
    }

    pub fn generate_bar(&self, active: bool) -> &'static str {
        static mut BUFFER: [u8; SETTING_BAR_SIZE + 1] = [b'='; SETTING_BAR_SIZE + 1];

        let value_out_of_15 = (((self.value as u32) * 15) / (self.max_value as u32)) as u8;

        unsafe {
            BUFFER[0] = b'[';
            BUFFER[SETTING_BAR_SIZE] = b']';
            for i in 0..=SETTING_BAR_SIZE {
                if i as u8 == value_out_of_15 && i > 0 {
                    if active {
                        BUFFER[i as usize] = b'5';
                    } else {
                        BUFFER[i as usize] = b'4';
                    }
                } else if (i as u8) < value_out_of_15 {
                    BUFFER[i as usize] = b'4';
                } else if i == SETTING_BAR_SIZE {
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
