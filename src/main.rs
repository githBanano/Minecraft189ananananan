use macroquad::prelude::*;

const WORLD_SIZE: i32 = 24;
const BLOCK_SIZE: f32 = 1.0;
const PLAYER_SPEED: f32 = 8.0;
const MOUSE_SENSITIVITY: f32 = 0.0025;

struct PlayerCamera {
    position: Vec3,
    yaw: f32,
    pitch: f32,
}

impl PlayerCamera {
    fn new() -> Self {
        Self {
            position: vec3(12.0, 9.0, 18.0),
            yaw: -0.7,
            pitch: -0.25,
        }
    }

    fn forward(&self) -> Vec3 {
        vec3(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize()
    }

    fn update(&mut self, dt: f32) {
        if is_mouse_button_pressed(MouseButton::Left) {
            set_cursor_grab(true);
            show_mouse(false);
        }
        if is_key_pressed(KeyCode::Escape) {
            set_cursor_grab(false);
            show_mouse(true);
        }

        if is_mouse_button_down(MouseButton::Left) {
            let delta = mouse_delta_position();
            self.yaw -= delta.x * MOUSE_SENSITIVITY * 100.0;
            self.pitch = (self.pitch + delta.y * MOUSE_SENSITIVITY * 100.0)
                .clamp(-1.54, 1.54);
        }

        let forward = self.forward();
        let flat_forward = vec3(forward.x, 0.0, forward.z).normalize_or_zero();
        let right = vec3(-flat_forward.z, 0.0, flat_forward.x);
        let mut movement = Vec3::ZERO;

        if is_key_down(KeyCode::W) { movement += flat_forward; }
        if is_key_down(KeyCode::S) { movement -= flat_forward; }
        if is_key_down(KeyCode::D) { movement += right; }
        if is_key_down(KeyCode::A) { movement -= right; }
        if is_key_down(KeyCode::Space) { movement.y += 1.0; }
        if is_key_down(KeyCode::LeftShift) { movement.y -= 1.0; }

        if movement.length_squared() > 0.0 {
            self.position += movement.normalize() * PLAYER_SPEED * dt;
        }
    }

    fn activate(&self) {
        let target = self.position + self.forward();
        set_camera(&Camera3D {
            position: self.position,
            target,
            up: Vec3::Y,
            fovy: 70.0,
            aspect: Some(screen_width() / screen_height()),
            near: 0.05,
            far: 500.0,
            ..Default::default()
        });
    }
}

fn terrain_height(x: i32, z: i32) -> i32 {
    let wave = ((x as f32 * 0.31).sin() * 1.4
        + (z as f32 * 0.27).cos() * 1.2
        + ((x + z) as f32 * 0.13).sin() * 0.8)
        .round() as i32;
    2 + wave.max(-1)
}

fn draw_world() {
    for x in 0..WORLD_SIZE {
        for z in 0..WORLD_SIZE {
            let height = terrain_height(x, z);
            for y in 0..height {
                let color = if y == height - 1 {
                    Color::from_rgba(91, 166, 74, 255)
                } else if y >= height - 3 {
                    Color::from_rgba(125, 91, 61, 255)
                } else {
                    Color::from_rgba(112, 112, 112, 255)
                };
                draw_cube(
                    vec3(x as f32, y as f32, z as f32),
                    vec3(BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE),
                    None,
                    color,
                );
            }
        }
    }
}

#[macroquad::main("Minecraft 1.8.9 — Rust Prototype")]
async fn main() {
    let mut camera = PlayerCamera::new();

    loop {
        let dt = get_frame_time().min(0.05);
        camera.update(dt);

        clear_background(Color::from_rgba(116, 178, 235, 255));
        camera.activate();
        draw_world();

        set_default_camera();
        draw_text("MINECRAFT 1.8.9 — RUST REIMPLEMENTATION", 20.0, 30.0, 22.0, WHITE);
        draw_text("Prototype: procedural voxel terrain | WASD + mouse | SPACE/SHIFT fly", 20.0, 55.0, 18.0, WHITE);
        draw_text("Click to capture mouse • ESC to release", 20.0, 80.0, 18.0, WHITE);
        draw_text("+", screen_width() * 0.5 - 5.0, screen_height() * 0.5 + 6.0, 24.0, WHITE);

        next_frame().await;
    }
}
