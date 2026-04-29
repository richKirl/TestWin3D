use mxg11l::{GlDepthExt, GlFunctions, GlVsyncExt, GlWindow, XDisplay};
use mxg11m::{mat4vf::Mat4vf, vec4f::Vec4f};

use crate::basetoggles::BaseToggles;

pub struct BaseState<'a> {
    pub window: GlWindow<'a>, // Теперь окно знает, что дисплей живет снаружи
    pub window_config: Vec4f,
    pub perspective_config: Vec4f,
    pub projection: Mat4vf,
    pub background: Vec4f,
    pub basetoggles: BaseToggles,
    pub gl: &'a GlFunctions,
}

impl<'a> BaseState<'a> {
    pub fn new(display: &'a XDisplay, gl: &'a GlFunctions) -> Self {
        let w = 800.0;
        let h = 600.0;

        let window_config = Vec4f::new(w, h, w * 0.5, h * 0.5);
        let perspective_config = Vec4f::new(45.0f32.to_radians(), w / h, 0.1, 1000.0);
        let projection = Mat4vf::perspective(
            perspective_config.fov(),
            perspective_config.aspect(),
            perspective_config.near(),
            perspective_config.far(),
        );

        // Передаем ссылку на display, который потом переместим в структуру
        let window = GlWindow::new(&display, "Test Rust Library", w as u32, h as u32, 4, 6)
            .expect("Не удалось открыть window")
            .add_vsync(&gl, true)
            .add_depth_with_alpha();

        Self {
            // display, // Сохраняем здесь!
            window,
            window_config,
            perspective_config,
            projection,
            background: Vec4f::new(0.22, 0.44, 0.66, 1.0),
            basetoggles: BaseToggles::new(),
            gl: gl,
        }
    }
}
