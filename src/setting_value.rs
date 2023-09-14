const SETTING_BAR_SIZE: usize = 15;

pub struct Setting {
    // must be pub to allow static instantiation
    pub value: u8,
    pub max_value: u8,
}

const EMPTY_LEFT_BRACKET: u8 = b'[';
const EMPTY_RIGHT_BRACKET: u8 = b']';
const SLIDER_ACTIVE: u8 = b'5';
const BAR_FILLED: u8 = b'4';
const BAR_EMPTY: u8 = b'=';

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
        static mut BUFFER: [u8; SETTING_BAR_SIZE + 1] = [BAR_EMPTY; SETTING_BAR_SIZE + 1];

        let value_out_of_15 = (((self.value as u32) * 15) / (self.max_value as u32)) as usize;

        unsafe {
            BUFFER[0] = if self.value == 0 {
                if active {
                    SLIDER_ACTIVE
                } else {
                    EMPTY_LEFT_BRACKET
                }
            } else {
                BAR_FILLED
            };
            BUFFER[SETTING_BAR_SIZE] = if self.value == self.max_value {
                if active {
                    SLIDER_ACTIVE
                } else {
                    BAR_FILLED
                }
            } else {
                EMPTY_RIGHT_BRACKET
            };

            for i in 1..SETTING_BAR_SIZE {
                BUFFER[i] = match i.cmp(&value_out_of_15) {
                    core::cmp::Ordering::Less => BAR_FILLED,
                    core::cmp::Ordering::Equal => {
                        if active {
                            SLIDER_ACTIVE
                        } else if self.value == self.max_value {
                            BAR_FILLED
                        } else {
                            BAR_EMPTY
                        }
                    }
                    _ => BAR_EMPTY,
                };
            }

            core::str::from_utf8_unchecked(&BUFFER)
        }
    }
}
