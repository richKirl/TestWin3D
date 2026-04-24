use math3d::{mat4vf::Mat4vf, vec3f::Vec3f};

//pub struct Camera {
//     pub position: Vec3f,
//     pub forward: Vec3f,
//     pub velocity: Vec3f,
//     pub yaw: f32,   // Поворот влево-вправо
//     pub pitch: f32, // Поворот вверх-вниз
//     pub speed: f32,
//     pub sensitivity: f32,
//     pub walk_speed: f32,
//     pub jump_power: f32,
//     pub gravity: f32,
//     pub is_grounded: bool,
//     pub is_moving: bool,
// }
// pub fn new(position: Vec3f) -> Self {
//     Self {
//         position,
//         velocity: Vec3f::new(0.0, 0.0, 0.0),
//         forward: Vec3f::new(0.0, 0.0, 0.0),
//         yaw: -90.0, // Чтобы камера смотрела "вперед" по умолчанию
//         pitch: 0.0,
//         speed: 15.5,
//         sensitivity: 0.1,
//         walk_speed: 0.3,
//         jump_power: 0.4,
//         gravity: -0.015,
//         is_grounded: false,
//         is_moving: false,
//     }
// }
pub struct Camera {
    pub position: Vec3f,
    pub forward: Vec3f,
    pub yaw: f32,   // Поворот влево-вправо
    pub pitch: f32, // Поворот вверх-вниз
    pub walk_speed: f32,
}
#[rustfmt::skip]
impl Camera {
    pub fn new(position: Vec3f) -> Self {
        Self {
            position,
            forward: Vec3f::new(0.0, 0.0, 0.0),
            yaw: -90.0, // Чтобы камера смотрела "вперед" по умолчанию
            pitch: 0.0,
            walk_speed: 0.3,
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
        self.yaw += xrel as f32 * 0.1;
        self.pitch -= yrel as f32 * 0.1;

        // Ограничение, чтобы не "перевернуться" через голову
        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }
    }
}
