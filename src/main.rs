#[allow(non_snake_case, unused)]
use math3d::{dualquatf, frustum, mat4vf, quatf, vec3d, vec3f, vec4d, vec4f};
use math3d::{mat4vf::Mat4vf, vec3f::Vec3f};
use mxg11l::{
    Event, GlDebugExt, GlFunctions, GlWindow, KEY_A, KEY_D, KEY_ESCAPE, KEY_S, KEY_TAB, KEY_W,
    KM_BUTTON_LEFT, Timer, XDisplay,
};
use mxgimage::TgaImage;
#[allow(non_snake_case)]
mod autoMesh;
mod camera;
mod input_handle;
mod shaders;
use crate::{
    autoMesh::{auto_cube::Cube, auto_plane::Plane},
    camera::Camera,
    input_handle::InputState,
    shaders::{FRAG_SRC, VERT_SRC},
};

fn main() {
    let width = 800.0;
    let height = 600.0;
    // 1. Инициализация (все unsafe скрыто внутри)
    let display = XDisplay::open().expect("Не удалось открыть X11 -- display");
    let window = GlWindow::new(
        &display,
        "Test Rust Library",
        width as u32,
        height as u32,
        4,
        6,
    )
    .expect("Не удалось открыть window -- window")
    .display_version_debug();

    let gl = GlFunctions::load();
    window.set_vsync(&gl, true);
    // 1. Исходники шейдеров
    // ============================================================
    // ============================================================

    let mut aspect_r = width / height;
    let mut center_x = (width / 2.0) as i32;
    let mut center_y = (height / 2.0) as i32;
    //window.warp_center(center_x, center_y);
    let mut running = true;
    let mut togle_mouse = false;
    let mut toggle_warframe = false;
    let program = gl.compilation_shaders(&gl, VERT_SRC, FRAG_SRC);
    // ===========================================================
    // ===========================================================*vec4(0.3,0.5,0.3,1.0);
    // 2. Внутри main (после создания контекста)
    // ===========================================================
    // ===========================================================

    // 3. Данные треугольника (VBO/VAO)

    // =============================================================================
    // =============================================================================

    let mut camera = Camera::new(Vec3f::new(0.0, 0.0, 5.0));
    // 2. Цикл отрисовки
    let pvloc = gl.get_location(program, "pv");
    let texloc = gl.get_location(program, "tex");
    let modelloc = gl.get_location(program, "model");
    let image = TgaImage::load("geometry2.tga");
    let tex = gl.create_texture_bgra(512, 512, &image.pixels);
    let cube = Cube::new(&gl, tex, texloc, modelloc);
    let palne = Plane::new(&gl, tex, texloc, modelloc);
    //println!("{:?}", pvloc);
    gl.enable_depth_test();
    let mut input = InputState::new();
    let mut timer = Timer::new();

    //gl.disable_cull_face();
    while running {
        timer.update();

        // Красивый и безопасный опрос событий
        for event in window.poll_events() {
            match event {
                Event::ClientMessage => {
                    running = false;
                }
                Event::Resize { width, height } => {
                    gl.viewport(width, height);
                    aspect_r = width as f32 / height as f32;
                    center_x = (width as f32 / 2.0) as i32;
                    center_y = (height as f32 / 2.0) as i32;
                }
                Event::MouseMove { x, y } => {
                    // 1. Считаем смещение относительно центра
                    if togle_mouse {
                        let dx = x - center_x;
                        let dy = y - center_y;

                        // 2. Если мышь сдвинулась, обновляем камеру и возвращаем курсор назад
                        if dx != 0 || dy != 0 {
                            camera.update_angles(dx, dy);

                            // 3. Важно: возвращаем мышь в центр, чтобы она никогда не дошла до края
                            window.warp_center(center_x, center_y);
                        }
                    }
                }
                Event::KeyPress { keysym, .. } => {
                    if keysym == KEY_ESCAPE {
                        if togle_mouse {
                            togle_mouse = false;
                            window.show_cursor();
                        } else if !togle_mouse {
                            running = false;
                        }
                    } // Esc
                    if keysym == KEY_TAB {
                        toggle_warframe = !toggle_warframe;
                        if toggle_warframe {
                            gl.polygonmode_front_back_line();
                        } else {
                            gl.polygonmode_front_back_fill();
                        }
                    } // tab
                    if keysym == KEY_W {
                        input.w = true;
                    }
                    if keysym == KEY_S {
                        input.s = true;
                    }
                    if keysym == KEY_A {
                        input.a = true;
                    }
                    if keysym == KEY_D {
                        input.d = true;
                    }
                }
                Event::KeyRelease { keysym, .. } => {
                    if keysym == KEY_W {
                        input.w = false;
                    }
                    if keysym == KEY_S {
                        input.s = false;
                    }
                    if keysym == KEY_A {
                        input.a = false;
                    }
                    if keysym == KEY_D {
                        input.d = false;
                    }
                }
                Event::MouseButtonPress { button, .. } => {
                    if button == KM_BUTTON_LEFT {
                        if !togle_mouse {
                            togle_mouse = true;
                            window.hide_cursor();
                        }
                    }
                }
                _ => {}
            }
        }
        camera.update_input(&input, &timer);
        //println!("{:?}", camera.position);
        gl.clear_color_depth(0.2, 0.4, 0.6, 1.0);
        let view = camera.get_view_matrix();
        let proj = Mat4vf::perspective(45.0f32.to_radians(), aspect_r, 0.1, 1000.0);
        let pv = proj * view; //proj * view * Mat4vf::identity()
        gl.use_program(program);
        gl.uniform_matrix_4fv(pvloc, 1, pv.cols.as_ptr() as *const f32);
        cube.draw(&gl);
        palne.draw(&gl);
        window.swap_buffers();
    }
    gl.delete_program(program);
}
