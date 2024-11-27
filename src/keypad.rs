const KEY_COUNT: usize = 16;

pub struct KeyPad {
    keys: [bool; KEY_COUNT],
}

impl KeyPad {
    pub fn new() -> KeyPad {
        KeyPad {
            keys: [false; KEY_COUNT],
        }
    }
}
