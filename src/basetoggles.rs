pub struct BaseToggles {
    pub running: bool,
    pub togle_mouse: bool,
    pub toggle_wireframe: bool,
}

impl BaseToggles {
    pub fn new() -> Self {
        Self {
            running: false,
            togle_mouse: false,
            toggle_wireframe: false,
        }
    }
}
