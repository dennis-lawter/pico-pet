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
        self.value = (self.value + 1).min(self.max_value);
    }

    pub fn dec(&mut self) {
        self.value = self.value.saturating_sub(1);
    }

    pub fn generate_bar(&self, active: bool) -> &'static str {
        static mut BUFFER: [u8; SETTING_BAR_SIZE + 1] = [b'='; SETTING_BAR_SIZE + 1];

        let value_out_of_15 = (((self.value as u32) * 15) / (self.max_value as u32)) as usize;

        unsafe {
            BUFFER[0] = if self.value == 0 {
                if active {
                    b'5'
                } else {
                    b'['
                }
            } else {
                b'4'
            };

            BUFFER[SETTING_BAR_SIZE] = b']';
            for i in 1..SETTING_BAR_SIZE {
                BUFFER[i] = match i.cmp(&value_out_of_15) {
                    core::cmp::Ordering::Less => b'4',
                    core::cmp::Ordering::Equal => {
                        if active {
                            b'5'
                        } else {
                            b'4'
                        }
                    }
                    _ => b'=',
                };
            }

            core::str::from_utf8_unchecked(&BUFFER)
        }
    }
}
