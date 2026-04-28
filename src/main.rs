#[allow(non_snake_case, unused)]
use math3d::{dualquatf, frustum, mat4vf, quatf, vec3d, vec3f, vec4d, vec4f};
use math3d::{quatf::Quatf, vec3f::Vec3f};
use mxg11l::{GlFunctions, Timer, XDisplay};
use mxgimage::TgaImage;
#[allow(non_snake_case)]
mod autoMesh;
mod basestate;
mod basetoggles;
mod camera;
mod input_handle;
mod shader;
mod shaders;
mod transform;
use crate::{
    autoMesh::auto_cube::Cube,
    basestate::BaseState,
    camera::Camera,
    input_handle::InputState,
    shader::Shader,
    shaders::{FRAG_SRC, VERT_SRC},
    transform::Transformer,
};

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
