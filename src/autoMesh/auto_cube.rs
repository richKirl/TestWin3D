// use crate::autoMesh::auto_mesh::vertices;
use math3d::mat4vf::Mat4vf;
use mxg11l::GlFunctions;

use crate::{Shader, autoMesh::auto_mesh::VERTICES_CUBE};
pub struct Cube<'a> {
    vao: u32,
    vbo: u32,
    texture: u32,
    // loc_tex: i32,
    // loc_model: i32,
    gl: &'a GlFunctions,
}

impl<'a> Cube<'a> {
    pub fn new(gl: &'a GlFunctions, text: u32) -> Self {
        let vertices = VERTICES_CUBE;
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

        Self {
            vao: vao,
            vbo: vbo,
            texture: text,
            // loc_tex: loct,
            // loc_model: locm,
            gl: gl,
        }
    }
    pub fn draw(&self, gl: &GlFunctions, shader: &Shader) {
        shader.set_int("tex", 0);
        shader.set_mat4("model", Mat4vf::IDENTITY);
        // gl.uniform_1i(self.loc_tex, 0);
        // gl.uniform_matrix_4fv(self.loc_model, 1, Mat4vf::IDENTITY.as_ptr());

        gl.active_texture0();
        gl.bind_texture_2d(self.texture);
        gl.bind_vertex_array(self.vao);
        gl.draw_arrays_triangles(0, 36);
    }
}
impl<'a> Drop for Cube<'a> {
    fn drop(&mut self) {
        self.gl.delete_vertex_arrays(1, &self.vao);
        self.gl.delete_buffers(1, &self.vbo);
        self.gl.delete_textures(1, &self.texture);
    }
}
