const SETTING_MAX: u8 = 15;

pub struct Setting {
    value: u8,
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
        static mut BUFFER: [u8; SETTING_MAX as usize + 9] = [b' '; SETTING_MAX as usize + 9];

        unsafe {
            BUFFER[0 + 3] = b'[';
            BUFFER[SETTING_MAX as usize + 4] = b']';
            for i in 1..=SETTING_MAX {
                if i == self.value {
                    BUFFER[i as usize + 3] = b'#';
                } else if i < self.value {
                    BUFFER[i as usize + 3] = b'*';
                } else {
                    BUFFER[i as usize + 3] = b'.';
                }
            }

            core::str::from_utf8_unchecked(&BUFFER)
        }
    }
}
