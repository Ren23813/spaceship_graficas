// camera.rs
#![allow(dead_code)]

use raylib::prelude::*;
use crate::matrix::create_view_matrix;
use std::f32::consts::PI;

pub struct Camera {
    pub eye: Vector3,
    pub target: Vector3,
    pub up: Vector3,

    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,

    pub rotation_speed: f32,
    pub zoom_speed: f32,
    pub pan_speed: f32,

    pub min_distance: f32,
    pub max_distance: f32,
}

impl Camera {
    pub fn new(eye: Vector3, target: Vector3, up: Vector3) -> Self {
        let direction = Vector3::new(
            eye.x - target.x,
            eye.y - target.y,
            eye.z - target.z,
        );

        let distance = (direction.x * direction.x + direction.y * direction.y + direction.z * direction.z).sqrt();
        let pitch = if distance > 0.0 { (direction.y / distance).asin() } else { 0.0 };
        let yaw = direction.z.atan2(direction.x);

        let min_distance = 2.0; 
        let max_distance = 100.0;

        Camera {
            eye,
            target,
            up,
            yaw,
            pitch,
            distance: distance.clamp(min_distance, max_distance),
            rotation_speed: 0.05,
            zoom_speed: 0.5,
            pan_speed: 0.1,
            min_distance,
            max_distance,
        }
    }

    /// Update camera eye position based on yaw, pitch, and distance
    fn update_eye_position(&mut self) {
        // Clamp pitch para evitar que la cámara se voltee completamente
        self.pitch = self.pitch.clamp(-PI / 2.0 + 0.01, PI / 2.0 - 0.01);

        // Aseguramos distance en rango válido
        self.distance = self.distance.clamp(self.min_distance, self.max_distance);

        // Usamos trigonometría esférica para colocar eye con seguridad
        let cos_pitch = self.pitch.cos();
        let sin_pitch = self.pitch.sin();
        let cos_yaw = self.yaw.cos();
        let sin_yaw = self.yaw.sin();

        // Evitar producir NaN si distance es 0 (ya lo protegimos con clamp)
        let d = if self.distance.is_finite() { self.distance } else { self.min_distance };

        self.eye.x = self.target.x + d * cos_pitch * cos_yaw;
        self.eye.y = self.target.y + d * sin_pitch;
        self.eye.z = self.target.z + d * cos_pitch * sin_yaw;
    }

    pub fn get_view_matrix(&self) -> Matrix {
        create_view_matrix(self.eye, self.target, self.up)
    }

    pub fn process_input(&mut self, window: &RaylibHandle) {
        // Rotation controls (yaw)
        if window.is_key_down(KeyboardKey::KEY_A) {
            self.yaw += self.rotation_speed;
            self.update_eye_position();
        }
        if window.is_key_down(KeyboardKey::KEY_D) {
            self.yaw -= self.rotation_speed;
            self.update_eye_position();
        }

        // Rotation controls (pitch)
        if window.is_key_down(KeyboardKey::KEY_W) {
            self.pitch += self.rotation_speed;
            self.update_eye_position();
        }
        if window.is_key_down(KeyboardKey::KEY_S) {
            self.pitch -= self.rotation_speed;
            self.update_eye_position();
        }

        // Zoom controls (distance from target) - arrow keys
        if window.is_key_down(KeyboardKey::KEY_UP) {
            self.distance -= self.zoom_speed;
            // clamp inmediatamente para evitar valores muy pequeños
            self.distance = self.distance.clamp(self.min_distance, self.max_distance);
            self.update_eye_position();
        }
        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            self.distance += self.zoom_speed;
            self.distance = self.distance.clamp(self.min_distance, self.max_distance);
            self.update_eye_position();
        }

        // Pan controls (move target/center point)
        // Calculamos forward en el plano XZ; no normalizar si longitud ~0
        let forward = Vector3::new(
            self.target.x - self.eye.x,
            0.0,
            self.target.z - self.eye.z,
        );
        let forward_len = (forward.x * forward.x + forward.z * forward.z).sqrt();
        let forward_normalized = if forward_len > 1e-6 {
            Vector3::new(forward.x / forward_len, 0.0, forward.z / forward_len)
        } else {
            // si la cámara está justo encima del objetivo en XZ, elegimos un forward seguro
            Vector3::new(0.0, 0.0, 1.0)
        };

        let right = Vector3::new(
            forward_normalized.z,
            0.0,
            -forward_normalized.x,
        );

        // Q/E keys for horizontal panning
        if window.is_key_down(KeyboardKey::KEY_Q) {
            self.target.x += right.x * self.pan_speed;
            self.target.z += right.z * self.pan_speed;
            self.update_eye_position();
        }
        if window.is_key_down(KeyboardKey::KEY_E) {
            self.target.x -= right.x * self.pan_speed;
            self.target.z -= right.z * self.pan_speed;
            self.update_eye_position();
        }

        // Left/Right arrow keys for horizontal panning
        if window.is_key_down(KeyboardKey::KEY_LEFT) {
            self.target.x += right.x * self.pan_speed;
            self.target.z += right.z * self.pan_speed;
            self.update_eye_position();
        }
        if window.is_key_down(KeyboardKey::KEY_RIGHT) {
            self.target.x -= right.x * self.pan_speed;
            self.target.z -= right.z * self.pan_speed;
            self.update_eye_position();
        }

        // Vertical panning
        if window.is_key_down(KeyboardKey::KEY_R) {
            self.target.y += self.pan_speed;
            self.update_eye_position();
        }
        if window.is_key_down(KeyboardKey::KEY_F) {
            self.target.y -= self.pan_speed;
            self.update_eye_position();
        }
    }
}