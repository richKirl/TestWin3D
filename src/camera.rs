use math3d::{mat4vf::Mat4vf, vec3f::Vec3f};
use mxg11l::Timer;

use crate::InputState;
pub const WORLD_UP: Vec3f = Vec3f {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};
pub struct Camera {
    pub position: Vec3f,
    pub forward: Vec3f,
    pub yaw: f32,   // Поворот влево-вправо
    pub pitch: f32, // Поворот вверх-вниз
    pub walk_speed: f32,
}
#[allow(non_snake_case, unused)]
#[rustfmt::skip]
impl Camera {
    pub fn new(position: Vec3f) -> Self {
        let mut cam = Self {
            position,
            forward: Vec3f::new(0.0, 0.0, -1.0), // Сразу не нулевой
            yaw: -90.0,
            pitch: 0.0,
            walk_speed: 3.6,
        };
        cam.update_vectors(); // Считаем правильный forward сразу
        cam
    }

    // Выносим расчет векторов отдельно
    fn update_vectors(&mut self) {
        let yaw_rad = self.yaw.to_radians();
        let pitch_rad = self.pitch.to_radians();

        self.forward = Vec3f::new(
            yaw_rad.cos() * pitch_rad.cos(),
            pitch_rad.sin(),
            yaw_rad.sin() * pitch_rad.cos(),
        ).normalize();
    }

    pub fn get_view_matrix(&mut self) -> Mat4vf {
        // Теперь здесь не нужно считать forward,
        // он всегда актуален благодаря update_vectors
        Mat4vf::look_at(
            self.position,
            self.position + self.forward,
            WORLD_UP,
        )
    }

    pub fn update_angles(&mut self, xrel: i32, yrel: i32) {
        self.yaw += xrel as f32 * 0.1;
        self.pitch -= yrel as f32 * 0.1;
        self.pitch = self.pitch.clamp(-89.0, 89.0);

        self.update_vectors(); // Обновляем вектор сразу после поворота головы
    }

    pub fn update_input(&mut self, input: &InputState, timer: &Timer) {
        let speed = self.walk_speed * timer.delta_time;

        // Кэшируем вектор "вправо", чтобы не считать cross() дважды
        let right = self.forward.cross(WORLD_UP).normalize();

        if input.w { self.position += self.forward * speed; }
        if input.s { self.position -= self.forward * speed; }
        if input.a { self.position -= right * speed; }
        if input.d { self.position += right * speed; }
    }
}
