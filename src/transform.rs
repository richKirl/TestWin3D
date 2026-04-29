use mxg11m::{mat4vf::Mat4vf, quatf::Quatf, vec3f::Vec3f};

pub struct Transformer {
    pub position: Vec3f,
    pub scale: Vec3f,
    pub rotation: Quatf,
}
#[allow(unused)]
impl Transformer {
    pub fn new(position: Vec3f, rotation: Quatf, scale: Vec3f) -> Self {
        Self {
            position,
            scale,
            rotation,
        }
    }
    pub fn translate(&mut self, vec: &Vec3f) {
        self.position += *vec;
    }

    pub fn scale(&mut self, vec: &Vec3f) {
        self.scale *= *vec;
    }

    pub fn rotation(&mut self, rot_speed: f32, axis: &Vec3f, dt: f32) {
        let delta = Quatf::from_axis_angle(*axis, rot_speed * dt);
        self.rotation = self.rotation * delta;
    }

    pub fn get_model_matrix(&self) -> Mat4vf {
        let t = Mat4vf::translate_v3f(self.position);
        let r: Mat4vf = self.rotation.into();
        let s = Mat4vf::scale_v3f(self.scale);

        t * r * s // Возвращаем результат умножения
    }
}
