use std::collections::HashMap;

use math3d::mat4vf::Mat4vf;
use mxg11l::GlFunctions;

pub struct Shader<'a> {
    id: u32,
    locs: HashMap<&'a str, u32>,
    gl: &'a GlFunctions,
}

impl<'a> Shader<'a> {
    pub fn new(verts: Vec<&'a str>, gl: &'a GlFunctions) -> Self {
        let mut temp_locs: HashMap<&str, u32> = HashMap::new();
        let program = gl.compilation_shaders(
            &gl,
            verts.get(0).expect("oh vs"),
            verts.get(1).expect("oh fs"),
        );
        for src in verts.iter() {
            for line in src.lines() {
                if line.contains("layout") && !line.contains("in") {
                    // 1. Ищем индекс в layout (location = X)
                    let loc_start = line.find('=').unwrap_or(0) + 1;
                    let loc_end = line.find(')').unwrap_or(0);
                    let location_str = line[loc_start..loc_end].trim();

                    // 2. Ищем имя переменной (последнее слово перед точкой с запятой)
                    let name = line
                        .trim_end_matches(';')
                        .split_whitespace()
                        .last()
                        .unwrap_or("");

                    temp_locs
                        .entry(name)
                        .or_insert(location_str.parse::<u32>().expect("Not a valid number"));
                } else if !line.contains("layout")
                    && !line.contains("in")
                    && line.contains("uniform")
                {
                    // 1. Ищем индекс в layout (location = X)
                    // --
                    // 2. Ищем имя переменной (последнее слово перед точкой с запятой)
                    let name = line
                        .trim_end_matches(';')
                        .split_whitespace()
                        .last()
                        .unwrap_or("");

                    //println!("Переменная: {}", name);
                    let loc = gl.get_location(program, name);
                    temp_locs.entry(name).or_insert(loc as u32);
                }
            }
        }
        Self {
            id: program,
            locs: temp_locs,
            gl: gl,
        }
    }
    pub fn get_uniform(&self, uname: &str) -> i32 {
        *self.locs.get(uname).expect("oh") as i32
    }
    pub fn use_shader(&self) {
        self.gl.use_program(self.id);
    }
    pub fn set_int(&self, uname: &str, int: i32) {
        self.gl.uniform_1i(self.get_uniform(uname), int);
    }
    pub fn set_mat4(&self, uname: &str, mat: &Mat4vf) {
        self.gl
            .uniform_matrix_4fv(self.get_uniform(uname), 1, mat.as_ptr());
    }
}

impl<'a> Drop for Shader<'a> {
    fn drop(&mut self) {
        self.gl.delete_program(self.id);
        //self.locs = HashMap::new();
    }
}
