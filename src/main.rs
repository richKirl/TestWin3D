#[allow(non_snake_case, unused)]
use math3d::{dualquatf, frustum, mat4vf, quatf, vec3d, vec3f, vec4d, vec4f};
use math3d::{mat4vf::Mat4vf, vec3f::Vec3f};
use mxg11l::{Event, GlFunctions, GlWindow, KEY_ESCAPE, KEY_S, KEY_W, KM_BUTTON_LEFT, XDisplay};
use mxgimage::TgaImage;
pub struct Camera {
    pub position: Vec3f,
    pub forward: Vec3f,
    pub velocity: Vec3f,
    pub yaw: f32,   // Поворот влево-вправо
    pub pitch: f32, // Поворот вверх-вниз
    pub speed: f32,
    pub sensitivity: f32,
    pub walk_speed: f32,
    pub jump_power: f32,
    pub gravity: f32,
    pub is_grounded: bool,
    pub is_moving: bool,
}
#[rustfmt::skip]
impl Camera {
    pub fn new(position: Vec3f) -> Self {
        Self {
            position,
            velocity: Vec3f::new(0.0, 0.0, 0.0),
            forward: Vec3f::new(0.0, 0.0, 0.0),
            yaw: -90.0, // Чтобы камера смотрела "вперед" по умолчанию
            pitch: 0.0,
            speed: 15.5,
            sensitivity: 0.1,
            walk_speed: 0.3,
            jump_power: 0.4,
            gravity: -0.015,
            is_grounded: false,
            is_moving: false,
        }
    }
    pub fn get_ortho_matrix(&mut self, w: f32, h: f32) -> Mat4vf {
        return Mat4vf::orthographic(-w / 2.0, w / 2.0, -h / 2.0, h / 2.0, -1.0, 1.0);
    }
    pub fn get_view_matrix(&mut self) -> Mat4vf {
        // Вычисляем вектор направления взгляда из углов
        let yaw_rad = self.yaw.to_radians();
        let pitch_rad = self.pitch.to_radians();

        self.forward = Vec3f::new(
            yaw_rad.cos() * pitch_rad.cos(),
            pitch_rad.sin(),
            yaw_rad.sin() * pitch_rad.cos(),
        )
        .normalize();

        let up = Vec3f::new(0.0, 1.0, 0.0);

        // Используем ваш метод LookAt
        Mat4vf::look_at(
            self.position,
            self.position + self.forward,
            up,
        )
    }
    pub fn update_angles(&mut self, xrel: i32, yrel: i32) {
        self.yaw += xrel as f32 * self.sensitivity;
        self.pitch -= yrel as f32 * self.sensitivity;

        // Ограничение, чтобы не "перевернуться" через голову
        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }
    }
}
fn main() {
    // 1. Инициализация (все unsafe скрыто внутри)
    let display = XDisplay::open().expect("Не удалось открыть X11 -- display");
    let window = GlWindow::new(&display, "Test Rust Library", 800, 600)
        .expect("Не удалось открыть window -- window");
    //println!("Окно и GL контекст созданы успешно!");
    let gl = GlFunctions::load();
    window.set_vsync(&gl, true);
    // 1. Исходники шейдеров
    // ============================================================
    // ============================================================
    const VERT_SRC: &str = r#"
#version 460 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aT;
out vec2 aTex;
layout (location = 5)uniform mat4 pv;
void main() {
    gl_Position = pv*vec4(aPos, 1.0);
    aTex=aT;
}
"#;

    const FRAG_SRC: &str = r#"
#version 460 core
out vec4 FragColor;
in vec2 aTex;
uniform sampler2D tex;
void main() {
    FragColor = texture(tex, aTex);
}
"#;
    // ===========================================================
    // ===========================================================*vec4(0.3,0.5,0.3,1.0);
    // 2. Внутри main (после создания контекста)
    // ===========================================================
    // ===========================================================
    let program = gl.compilation_shaders(&gl, VERT_SRC, FRAG_SRC);
    // 3. Данные треугольника (VBO/VAO)
    let vertices: [f32; 30] = [
        // Первый треугольник
        //  X      Y     Z     U    V
        -0.5, 0.5, 0.5, 0.0, 1.0, // Левый верхний
        -0.5, -0.5, 0.5, 0.0, 0.0, // Левый нижний
        0.5, -0.5, 0.5, 1.0, 0.0, // Правый нижний
        // Второй треугольник
        0.5, -0.5, 0.5, 1.0, 0.0, // Правый нижний
        0.5, 0.5, 0.5, 1.0, 1.0, // Правый верхний
        -0.5, 0.5, 0.5, 0.0, 1.0, // Левый верхний
    ];

    let (mut vao, mut vbo) = (0, 0);
    gl.gen_vertex_arrays(1, &mut vao);
    gl.gen_buffers(1, &mut vbo);

    gl.bind_vertex_array(vao);
    gl.bind_buffer_array(vbo);

    gl.bind_buffer_data_array_static_draw(
        (vertices.len() * 4) as isize,
        vertices.as_ptr() as *const _,
    );

    let stride = (5 * std::mem::size_of::<f32>()) as i32; // 20 байт

    // Атрибут 0: Позиция (X, Y, Z)
    gl.vertex_attrib_pointer_float(
        0,
        3,
        stride, // шаг до следующей вершины
        std::ptr::null(),
    );
    gl.enable_vertex_attrib_array(0);

    // Атрибут 1: Текстурные координаты (U, V)
    gl.vertex_attrib_pointer_float(
        1,
        2,
        stride,              // шаг тот же самый (20 байт) // пропускаем 3 числа (X,Y,Z)
        (3 * 4) as *const _, // смещение в байтах (12)
    );
    gl.enable_vertex_attrib_array(1);

    let image = TgaImage::load("/home/kiji/Documents/math3d/geometry.tga");
    let tex = gl.create_texture_bgra(512, 512, &image.pixels);
    // =============================================================================
    // =============================================================================
    let mut width = 800.0;
    let mut height = 600.0;
    let mut aspect_r = (width / height);
    let mut camera = Camera::new(Vec3f::new(0.0, 0.0, -5.0));
    // 2. Цикл отрисовки
    let mut running = true;
    let pvloc = gl.get_location(program, "pv");
    let texloc = gl.get_location(program, "tex");
    println!("{:?}", pvloc);
    gl.enable_depth_test();
    let mut center_x = (width / 2.0) as i32;
    let mut center_y = (height / 2.0) as i32;
    //window.warp_center(center_x, center_y);
    let mut togle_mouse = false;
    //gl.disable_cull_face();
    while running {
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
                        } else if !togle_mouse {
                            running = false;
                        }
                    } // Esc
                    if keysym == KEY_W {
                        camera.position += camera.forward * camera.walk_speed;
                    }
                    if keysym == KEY_S {
                        camera.position -= camera.forward * camera.walk_speed;
                    }
                }
                Event::MouseButtonPress { button, .. } => {
                    if button == KM_BUTTON_LEFT {
                        if !togle_mouse {
                            togle_mouse = true;
                        }
                    }
                }
                _ => {}
            }
        }
        //println!("{:?}", camera.position);
        gl.clear_color_depth(&gl, 0.2, 0.4, 0.6, 1.0); //фишка моей библиотеки окно может залить себя теми двумя командами так как она знает контекст
        let view = camera.get_view_matrix();
        let proj = Mat4vf::perspective(45.0f32.to_radians(), aspect_r, 0.1, 1000.0);
        let pv = proj * view * Mat4vf::identity(); //proj * view * Mat4vf::identity()
        gl.use_program(program);
        gl.uniform_1i(texloc, 0);
        gl.uniform_matrix_4fv(pvloc, 1, pv.cols.as_ptr() as *const f32);
        gl.bind_vertex_array(vao);

        gl.active_texture0();
        gl.bind_texture_2d(tex);

        gl.draw_arrays_triangles(0, 6);
        window.swap_buffers();
    }
}
