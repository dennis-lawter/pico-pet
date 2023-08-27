pub const SETTING_MAX: u8 = 15;

pub struct Setting {
    pub value: u8,
}
#[allow(dead_code)]
impl Setting {
    pub fn new(value: u8) -> Result<Self, ()> {
        if value <= SETTING_MAX {
            Ok(Self { value })
        } else {
            Err(())
        }
    }

    pub fn new_max() -> Self {
        Self { value: SETTING_MAX }
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }

    pub fn set_value(&mut self, value: u8) -> Result<(), ()> {
        if value <= SETTING_MAX {
            self.value = value;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn inc(&mut self) {
        if self.value < SETTING_MAX {
            self.value += 1;
        }
    }

    pub fn dec(&mut self) {
        if self.value > 0 {
            self.value -= 1;
        }
    }

    pub fn generate_bar(&self) -> &'static str {
        static mut BUFFER: [u8; SETTING_MAX as usize + 1] = [b'='; SETTING_MAX as usize + 1];

        unsafe {
            BUFFER[0] = b'[';
            BUFFER[SETTING_MAX as usize] = b']';
            for i in 0..=SETTING_MAX {
                if i == self.value && i > 0 {
                    BUFFER[i as usize] = b'4';
                } else if i < self.value {
                    BUFFER[i as usize] = b'4';
                } else if i == SETTING_MAX {
                    BUFFER[i as usize] = b']';
                } else if i == 0 {
                    BUFFER[i as usize] = b'[';
                } else {
                    BUFFER[i as usize] = b'=';
                }
            }

            core::str::from_utf8_unchecked(&BUFFER)
        }
    }
}
