#[allow(non_snake_case, unused)]
use math3d::{dualquatf, frustum, mat4vf, quatf, vec3d, vec3f, vec4d, vec4f};
use math3d::{mat4vf::Mat4vf, quatf::Quatf, vec3f::Vec3f, vec4f::Vec4f};
use mxg11l::{GlDepthExt, GlFunctions, GlVsyncExt, GlWindow, Timer, XDisplay};
use mxgimage::TgaImage;
#[allow(non_snake_case)]
mod autoMesh;
mod basetoggles;
mod camera;
mod input_handle;
mod shader;
mod shaders;
mod transform;
use crate::{
    autoMesh::auto_cube::Cube,
    basetoggles::BaseToggles,
    camera::Camera,
    input_handle::InputState,
    shader::Shader,
    shaders::{FRAG_SRC, VERT_SRC},
    transform::Transformer,
};
pub struct BaseState<'a> {
    pub window: GlWindow<'a>, // Теперь окно знает, что дисплей живет снаружи
    pub window_config: Vec4f,
    pub perspective_config: Vec4f,
    pub projection: Mat4vf,
    pub background: Vec4f,
    pub basetoggles: BaseToggles,
    gl: &'a GlFunctions,
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

fn main() {
    let gl = GlFunctions::load();
    let display = XDisplay::open().expect("Не удалось открыть X11 -- display");
    let mut input = InputState::new();
    let mut base = BaseState::new(&display, &gl);

    // ============================================================
    // ============================================================

    let shader_main = Shader::new(vec![VERT_SRC, FRAG_SRC], &gl);
    // ===========================================================
    // ===========================================================*vec4(0.3,0.5,0.3,1.0);
    let mut camera = Camera::new(Vec3f::new(0.0, 0.0, 5.0));
    // ============================================================
    // ============================================================
    // 2. Цикл отрисовки
    let image = TgaImage::load("geometry2.tga");
    let tex = gl.create_texture_bgra(512, 512, &image.pixels);
    // ============================================================
    // ============================================================
    let cube = Cube::new(&gl, tex);
    let mut cube_transform = Transformer::new(
        Vec3f::new(0.0, 0.0, 0.0),
        Quatf::identity(),
        Vec3f::new(1.0, 1.0, 1.0),
    );
    // ============================================================
    // ============================================================

    let mut timer = Timer::new();
    // ============================================================
    // ============================================================
    base.basetoggles.running = true;
    // ============================================================
    // ============================================================

    while base.basetoggles.running {
        timer.update();
        input.handle_events(&mut camera, &mut base);
        camera.update_input(&input, &timer);
        cube_transform.rotation(1.0, &Vec3f::new(0.0, 1.0, 0.0), timer.delta_time);
        //println!("{:?}", camera.position);
        gl.clear_color_depth(
            base.background.r(),
            base.background.g(),
            base.background.b(),
            base.background.a(),
        );
        let view = camera.get_view_matrix();
        let model = cube_transform.get_model_matrix();
        let pv = base.projection * view * model; //proj * view * Mat4vf::identity()
        shader_main.use_shader();
        shader_main.set_mat4("pv", &pv);
        cube.draw(&gl, &shader_main);
        base.window.swap_buffers();
    }
}
