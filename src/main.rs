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

fn main() {
    let mut option_window = Vec4f::new(800.0, 600.0, 800.0 * 0.5, 600.0 * 0.5);
    let backgound = Vec4f::new(0.22, 0.44, 0.66, 1.0);
    let mut option_perspective = Vec4f::new(
        45.0f32.to_radians(),
        option_window.w() / option_window.h(),
        0.1,
        1000.0,
    );
    // ============================================================
    // ============================================================
    let gl = GlFunctions::load();
    // ============================================================
    // ============================================================
    // 1. Инициализация (все unsafe скрыто внутри)
    let display = XDisplay::open().expect("Не удалось открыть X11 -- display");
    let window = GlWindow::new(
        &display,
        "Test Rust Library",
        option_window.w() as u32,
        option_window.h() as u32,
        4,
        6,
    )
    .expect("Не удалось открыть window -- window")
    //.display_version_debug()
    .add_vsync(&gl, true)
    .add_depth_with_alpha();

    // ============================================================
    // ============================================================

    let mut toogles = BaseToggles::new();
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
    let mut input = InputState::new();
    // ============================================================
    // ============================================================
    let mut timer = Timer::new();
    // ============================================================
    // ============================================================
    toogles.running = true;
    // ============================================================
    // ============================================================
    let mut proj = Mat4vf::perspective(
        option_perspective.fov(),    //FOV
        option_perspective.aspect(), //aspect
        option_perspective.near(),   //near
        option_perspective.far(),    //far
    );

    while toogles.running {
        timer.update();

        input.handle_events(
            &window,
            &gl,
            &mut camera,
            &mut toogles,
            &mut option_window,
            &mut option_perspective,
            &mut proj,
        );
        camera.update_input(&input, &timer);
        cube_transform.rotation(1.0, &Vec3f::new(0.0, 1.0, 0.0), timer.delta_time);
        //println!("{:?}", camera.position);
        gl.clear_color_depth(backgound.r(), backgound.g(), backgound.b(), backgound.a());
        let view = camera.get_view_matrix();
        let model = cube_transform.get_model_matrix();
        let pv = proj * view * model; //proj * view * Mat4vf::identity()
        shader_main.use_shader();
        shader_main.set_mat4("pv", &pv);
        cube.draw(&gl, &shader_main);
        window.swap_buffers();
    }
}
