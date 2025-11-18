use raylib::prelude::*;

pub struct Light {
    pub position: Vector3,   
    pub color: Vector3,      
    pub intensity: f32,      
    pub range: f32,          
}

impl Light {
    pub fn new(position: Vector3) -> Self {
        Light {
            position,
            color: Vector3::new(1.0, 0.9, 0.8),
            intensity: 1.0,
            range: 200.0,
        }
    }

    pub fn new_with_params(position: Vector3, color: Vector3, intensity: f32, range: f32) -> Self {
        Light { position, color, intensity, range }
    }
}
