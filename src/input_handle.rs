use math3d::{mat4vf::Mat4vf, vec4f::Vec4f};
use mxg11l::{
    Event, GlFunctions, GlWindow, KEY_A, KEY_D, KEY_ESCAPE, KEY_S, KEY_TAB, KEY_W, KM_BUTTON_LEFT,
};

use crate::{basetoggles::BaseToggles, camera::Camera};

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
    pub fn handle_events(
        &mut self,
        window: &GlWindow,
        gl: &GlFunctions,
        camera: &mut Camera,
        toggles: &mut BaseToggles,
        // Параметры окна и проекции передаем как мутабельные ссылки
        opt_window: &mut Vec4f,
        opt_persp: &mut Vec4f,
        proj: &mut Mat4vf,
    ) {
        for event in window.poll_events() {
            match event {
                Event::ClientMessage => toggles.running = false,

                Event::Resize { width, height } => {
                    gl.viewport(width, height);
                    opt_persp.set_aspect(width as f32 / height as f32);
                    opt_window.set_center_x(width as f32 * 0.5);
                    opt_window.set_center_y(height as f32 * 0.5);
                    *proj = Mat4vf::perspective(
                        opt_persp.fov(),
                        opt_persp.aspect(),
                        opt_persp.near(),
                        opt_persp.far(),
                    );
                }

                Event::MouseMove { x, y } => {
                    if toggles.togle_mouse {
                        let dx = x - opt_window.center_x() as i32;
                        let dy = y - opt_window.center_y() as i32;
                        if dx != 0 || dy != 0 {
                            camera.update_angles(dx, dy);
                            window.warp_center(
                                opt_window.center_x() as i32,
                                opt_window.center_y() as i32,
                            );
                        }
                    }
                }

                Event::KeyPress { keysym, .. } => {
                    self.process_key(keysym, true, toggles, window, gl)
                }
                Event::KeyRelease { keysym, .. } => {
                    self.process_key(keysym, false, toggles, window, gl)
                }

                Event::MouseButtonPress { button, .. } => {
                    if button == KM_BUTTON_LEFT && !toggles.togle_mouse {
                        toggles.togle_mouse = true;
                        window.hide_cursor();
                    }
                }
                _ => {}
            }
        }
    }

    fn process_key(
        &mut self,
        keysym: u64,
        pressed: bool,
        toggles: &mut BaseToggles,
        window: &GlWindow,
        gl: &GlFunctions,
    ) {
        match keysym {
            KEY_W => self.w = pressed,
            KEY_S => self.s = pressed,
            KEY_A => self.a = pressed,
            KEY_D => self.d = pressed,
            KEY_ESCAPE if pressed => {
                if toggles.togle_mouse {
                    toggles.togle_mouse = false;
                    window.show_cursor();
                } else {
                    toggles.running = false;
                }
            }
            KEY_TAB if pressed => {
                toggles.toggle_wireframe = !toggles.toggle_wireframe;
                if toggles.toggle_wireframe {
                    gl.polygonmode_front_back_line();
                } else {
                    gl.polygonmode_front_back_fill();
                }
            }
            _ => {}
        }
    }
}
