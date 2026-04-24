use math3d::mat4vf::Mat4vf;
use mxg11l::GlFunctions;
use mxgimage::TgaImage;

pub struct Cube {
    vao: u32,
    vbo: u32,
    texture: u32,
    loc_tex: i32,
    loc_model: i32,
}

impl Cube {
    pub fn new(gl: &GlFunctions, path: &str, loct: i32, locm: i32) -> Self {
        let vertices: [f32; 180] = [
            // Первый треугольник front
            //  X      Y     Z     U    V
            -0.5, 0.5, 0.5, 0.0, 1.0, // Левый верхний
            -0.5, -0.5, 0.5, 0.0, 0.0, // Левый нижний
            0.5, -0.5, 0.5, 1.0, 0.0, // Правый нижний
            // Второй треугольник
            0.5, -0.5, 0.5, 1.0, 0.0, // Правый нижний
            0.5, 0.5, 0.5, 1.0, 1.0, // Правый верхний
            -0.5, 0.5, 0.5, 0.0, 1.0, // Левый верхний
            // Первый треугольник back
            //  X      Y     Z     U    V
            -0.5, 0.5, -0.5, 0.0, 1.0, // Левый верхний
            -0.5, -0.5, -0.5, 0.0, 0.0, // Левый нижний
            0.5, -0.5, -0.5, 1.0, 0.0, // Правый нижний
            // Второй треугольник
            0.5, -0.5, -0.5, 1.0, 0.0, // Правый нижний
            0.5, 0.5, -0.5, 1.0, 1.0, // Правый верхний
            -0.5, 0.5, -0.5, 0.0, 1.0, // Левый верхний
            // Первый треугольник left
            //  X      Y     Z     U    V
            -0.5, 0.5, -0.5, 0.0, 1.0, // Левый верхний
            -0.5, -0.5, -0.5, 0.0, 0.0, // Левый нижний
            -0.5, -0.5, 0.5, 1.0, 0.0, // Правый нижний
            // Второй треугольник
            -0.5, -0.5, 0.5, 1.0, 0.0, // Правый нижний
            -0.5, 0.5, 0.5, 1.0, 1.0, // Правый верхний
            -0.5, 0.5, -0.5, 0.0, 1.0, // Левый верхний
            // Первый треугольник right
            //  X      Y     Z     U    V
            0.5, 0.5, -0.5, 0.0, 1.0, // Левый верхний
            0.5, -0.5, -0.5, 0.0, 0.0, // Левый нижний
            0.5, -0.5, 0.5, 1.0, 0.0, // Правый нижний
            // Второй треугольник
            0.5, -0.5, 0.5, 1.0, 0.0, // Правый нижний
            0.5, 0.5, 0.5, 1.0, 1.0, // Правый верхний
            0.5, 0.5, -0.5, 0.0, 1.0, // Левый верхний
            // Первый треугольник up
            //  X      Y     Z     U    V
            -0.5, 0.5, -0.5, 0.0, 1.0, // Левый верхний
            -0.5, 0.5, 0.5, 0.0, 0.0, // Левый нижний
            0.5, 0.5, 0.5, 1.0, 0.0, // Правый нижний
            // Второй треугольник
            0.5, 0.5, 0.5, 1.0, 0.0, // Правый нижний
            0.5, 0.5, -0.5, 1.0, 1.0, // Правый верхний
            -0.5, 0.5, -0.5, 0.0, 1.0, // Левый верхний
            // Первый треугольник down
            //  X      Y     Z     U    V
            -0.5, -0.5, -0.5, 0.0, 1.0, // Левый верхний
            -0.5, -0.5, 0.5, 0.0, 0.0, // Левый нижний
            0.5, -0.5, 0.5, 1.0, 0.0, // Правый нижний
            // Второй треугольник
            0.5, -0.5, 0.5, 1.0, 0.0, // Правый нижний
            0.5, -0.5, -0.5, 1.0, 1.0, // Правый верхний
            -0.5, -0.5, -0.5, 0.0, 1.0, // Левый верхний
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
        let image = TgaImage::load(path);
        let tex = gl.create_texture_bgra(512, 512, &image.pixels);
        Self {
            vao: vao,
            vbo: vbo,
            texture: tex,
            loc_tex: loct,
            loc_model: locm,
        }
    }
    pub fn draw(&self, gl: &GlFunctions) {
        gl.uniform_1i(self.loc_tex, 0);
        gl.uniform_matrix_4fv(
            self.loc_model,
            1,
            Mat4vf::identity().cols.as_ptr() as *const f32,
        );

        gl.active_texture0();
        gl.bind_texture_2d(self.texture);
        gl.bind_vertex_array(self.vao);
        gl.draw_arrays_triangles(0, 36);
    }
}
// impl Drop for Cube {
//     fn drop(&mut self) {
//         unsafe {
//             gl::DeleteVertexArrays(1, &self.vao);
//             gl::DeleteBuffers(1, &self.vbo);
//         }
//     }
// }
