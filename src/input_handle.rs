use math3d::mat4vf::Mat4vf;
use mxg11l::{
    Event, GlFunctions, GlWindow, KEY_A, KEY_D, KEY_ESCAPE, KEY_S, KEY_TAB, KEY_W, KM_BUTTON_LEFT,
};

use crate::{BaseState, basetoggles::BaseToggles, camera::Camera};

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
    pub fn handle_events(&mut self, camera: &mut Camera, base: &mut BaseState) {
        for event in base.window.poll_events() {
            match event {
                Event::ClientMessage => base.basetoggles.running = false,

                Event::Resize { width, height } => {
                    base.gl.viewport(width, height);
                    base.perspective_config
                        .set_aspect(width as f32 / height as f32);
                    base.window_config.set_center_x(width as f32 * 0.5);
                    base.window_config.set_center_y(height as f32 * 0.5);
                    base.projection = Mat4vf::perspective(
                        base.perspective_config.fov(),
                        base.perspective_config.aspect(),
                        base.perspective_config.near(),
                        base.perspective_config.far(),
                    );
                }

                Event::MouseMove { x, y } => {
                    if base.basetoggles.togle_mouse {
                        let dx = x - base.window_config.center_x() as i32;
                        let dy = y - base.window_config.center_y() as i32;
                        if dx != 0 || dy != 0 {
                            camera.update_angles(dx, dy);
                            base.window.warp_center(
                                base.window_config.center_x() as i32,
                                base.window_config.center_y() as i32,
                            );
                        }
                    }
                }

                Event::KeyPress { keysym, .. } => {
                    self.process_key(keysym, true, &mut base.basetoggles, &base.window, base.gl)
                }
                Event::KeyRelease { keysym, .. } => {
                    self.process_key(keysym, false, &mut base.basetoggles, &base.window, base.gl)
                }

                Event::MouseButtonPress { button, .. } => {
                    if button == KM_BUTTON_LEFT && !base.basetoggles.togle_mouse {
                        base.basetoggles.togle_mouse = true;
                        base.window.hide_cursor();
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
