const KEY_REPEAT_FRAMES: usize = 5;
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
