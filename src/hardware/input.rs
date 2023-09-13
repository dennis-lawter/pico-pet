// #[allow(dead_code)]
// #[derive(Default)]
// pub struct InputHandler {
//     pub key0_is_down: bool,
//     pub key1_is_down: bool,
//     pub key2_is_down: bool,
//     pub key3_is_down: bool,
//     pub key0_was_down: bool,
//     pub key1_was_down: bool,
//     pub key2_was_down: bool,
//     pub key3_was_down: bool,
//     pub key0_just_pressed: bool,
//     pub key1_just_pressed: bool,
//     pub key2_just_pressed: bool,
//     pub key3_just_pressed: bool,
//     pub key0_just_released: bool,
//     pub key1_just_released: bool,
//     pub key2_just_released: bool,
//     pub key3_just_released: bool,
// }

// impl InputHandler {
//     pub fn update(&mut self) {
//         let hardware = crate::globals::get_hardware();

//         self.key0_just_pressed = false;
//         self.key1_just_pressed = false;
//         self.key2_just_pressed = false;
//         self.key3_just_pressed = false;
//         self.key0_just_released = false;
//         self.key1_just_released = false;
//         self.key2_just_released = false;
//         self.key3_just_released = false;

//         self.key0_was_down = self.key0_is_down;
//         self.key0_was_down = self.key0_is_down;
//         self.key0_was_down = self.key0_is_down;
//         self.key0_was_down = self.key0_is_down;

//         self.key0_is_down = hardware.key0_pressed();
//         self.key1_is_down = hardware.key1_pressed();
//         self.key2_is_down = hardware.key2_pressed();
//         self.key3_is_down = hardware.key3_pressed();

//         if !self.key0_was_down && self.key0_is_down {
//             self.key0_just_pressed = true;
//         } else if self.key0_was_down && !self.key0_is_down {
//             self.key0_just_released = true;
//         }

//         if !self.key1_was_down && self.key1_is_down {
//             self.key1_just_pressed = true;
//         } else if self.key1_was_down && !self.key1_is_down {
//             self.key1_just_released = true;
//         }

//         if !self.key2_was_down && self.key2_is_down {
//             self.key2_just_pressed = true;
//         } else if self.key2_was_down && !self.key2_is_down {
//             self.key2_just_released = true;
//         }

//         if !self.key3_was_down && self.key3_is_down {
//             self.key3_just_pressed = true;
//         } else if self.key3_was_down && !self.key3_is_down {
//             self.key3_just_released = true;
//         }
//     }
// }

const KEY_REPEAT_FRAMES: usize = 3;
#[allow(dead_code)]
#[derive(Default)]
pub struct KeyState {
    pub is_down: bool,
    pub was_down: bool,
    pub just_pressed: bool,
    pub just_released: bool,
    pub key_held_frames: usize,
    pub key_repeat_triggered: bool,
}

impl KeyState {
    fn update(&mut self, is_down: bool) {
        self.was_down = self.is_down;
        self.is_down = is_down;
        self.just_pressed = !self.was_down && is_down;
        self.just_released = self.was_down && !is_down;
        if is_down {
            self.key_held_frames += 1;
        } else {
            self.key_held_frames = 0;
        }
        self.key_repeat_triggered = self.key_held_frames % KEY_REPEAT_FRAMES == 1;
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum KeyNames {
    Back = 0,
    Left = 1,
    Right = 2,
    Confirm = 3,
}
impl Into<usize> for KeyNames {
    fn into(self) -> usize {
        self as usize
    }
}

#[allow(dead_code)]
#[derive(Default)]
pub struct InputHandler {
    pub keys: [KeyState; 4],
}

impl InputHandler {
    pub fn update(&mut self) {
        let hardware = crate::globals::get_hardware();
        let key_positions = [
            hardware.key0_pressed(),
            hardware.key1_pressed(),
            hardware.key2_pressed(),
            hardware.key3_pressed(),
        ];
        for i in 0..self.keys.len() {
            self.keys[i].update(key_positions[i]);
        }
    }

    pub fn get_state(&self, name: &KeyNames) -> &KeyState {
        &self.keys[name.clone() as usize]
    }
}
