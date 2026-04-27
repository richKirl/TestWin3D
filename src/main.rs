#[allow(non_snake_case, unused)]
use math3d::{dualquatf, frustum, mat4vf, quatf, vec3d, vec3f, vec4d, vec4f};
use math3d::{mat4vf::Mat4vf, vec3f::Vec3f};
use mxg11l::{
    Event, GlDebugExt, GlDepthExt, GlFunctions, GlVsyncExt, GlWindow, KEY_A, KEY_D, KEY_ESCAPE,
    KEY_S, KEY_TAB, KEY_W, KM_BUTTON_LEFT, Timer, XDisplay,
};
use mxgimage::TgaImage;
#[allow(non_snake_case)]
mod autoMesh;
mod basetoggles;
mod camera;
mod input_handle;
mod shader;
mod shaders;
use crate::{
    autoMesh::auto_cube::Cube,
    basetoggles::BaseToggles,
    camera::Camera,
    input_handle::InputState,
    shader::Shader,
    shaders::{FRAG_SRC, VERT_SRC},
};

fn main() {
    let width = 800.0;
    let height = 600.0;
    let gl = GlFunctions::load();
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
    .display_version_debug()
    .add_vsync(&gl, true)
    .add_depth_with_alpha();

    // 1. Исходники шейдеров
    // ============================================================
    // ============================================================

    let mut aspect_r = width / height;
    let mut center_x = (width / 2.0) as i32;
    let mut center_y = (height / 2.0) as i32;

    let mut toogles = BaseToggles::new();

    let shader_main = Shader::new(vec![VERT_SRC, FRAG_SRC], &gl);
    // ===========================================================
    // ===========================================================*vec4(0.3,0.5,0.3,1.0);
    let mut camera = Camera::new(Vec3f::new(0.0, 0.0, 5.0));
    // 2. Цикл отрисовки
    let image = TgaImage::load("geometry2.tga");
    let tex = gl.create_texture_bgra(512, 512, &image.pixels);
    let cube = Cube::new(&gl, tex);

    let mut input = InputState::new();
    let mut timer = Timer::new();
    toogles.running = true;
    while toogles.running {
        timer.update();

        // Красивый и безопасный опрос событий
        for event in window.poll_events() {
            match event {
                Event::ClientMessage => {
                    toogles.running = false;
                }
                Event::Resize { width, height } => {
                    gl.viewport(width, height);
                    aspect_r = width as f32 / height as f32;
                    center_x = (width as f32 / 2.0) as i32;
                    center_y = (height as f32 / 2.0) as i32;
                }
                Event::MouseMove { x, y } => {
                    // 1. Считаем смещение относительно центра
                    if toogles.togle_mouse {
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
                        if toogles.togle_mouse {
                            toogles.togle_mouse = false;
                            window.show_cursor();
                        } else if !toogles.togle_mouse {
                            toogles.running = false;
                        }
                    } // Esc lag
                    if keysym == KEY_TAB {
                        toogles.toggle_wireframe = !toogles.toggle_wireframe;
                        if toogles.toggle_wireframe {
                            gl.polygonmode_front_back_line();
                        } else {
                            gl.polygonmode_front_back_fill();
                        }
                    } // tab  lag
                    if keysym == KEY_W {
                        input.w = true;
                    } //w nolag
                    if keysym == KEY_S {
                        input.s = true;
                    } //s nolag
                    if keysym == KEY_A {
                        input.a = true;
                    } //a nolag
                    if keysym == KEY_D {
                        input.d = true;
                    } //d nolag
                }
                Event::KeyRelease { keysym, .. } => {
                    if keysym == KEY_W {
                        input.w = false;
                    } //w nolag
                    if keysym == KEY_S {
                        input.s = false;
                    } //s nolag
                    if keysym == KEY_A {
                        input.a = false;
                    } //a nolag
                    if keysym == KEY_D {
                        input.d = false;
                    } //d nolag
                }
                Event::MouseButtonPress { button, .. } => {
                    if button == KM_BUTTON_LEFT {
                        if !toogles.togle_mouse {
                            toogles.togle_mouse = true;
                            window.hide_cursor();
                        }
                    } //left lag
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
        shader_main.use_shader();
        shader_main.set_mat4("pv", &pv);
        cube.draw(&gl, &shader_main);
        window.swap_buffers();
    }
}
