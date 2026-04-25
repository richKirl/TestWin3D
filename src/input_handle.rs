pub struct InputState {
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            a: false,
            d: false,
            s: false,
            w: false,
        }
    }
}
