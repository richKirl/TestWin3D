struct Fps_timer {
    fps_timer: f32,
    frame_count: i32,
}

impl Fps_timer {
    pub fn new() -> Self {
        Self {
            fps_timer: 0.0,
            frame_count: 0,
        }
    }
    pub fn update(&mut self, timer: Timer) {
        self.fps_timer += timer.delta_time;
        self.frame_count += 1;

        if self.fps_timer >= 1.0 {
            println!("FPS: {}", self.frame_count);
            // Или выведи в заголовок окна:
            // window.set_title(&format!("My Engine - FPS: {}", frame_count));
            self.fps_timer = 0.0;
            self.frame_count = 0;
        }
    }
}
