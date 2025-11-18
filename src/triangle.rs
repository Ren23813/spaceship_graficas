use crate::fragment::{Fragment};
use crate::light::Light;
use crate::vertex::Vertex;
use raylib::prelude::*;

fn barycentric_coordinates(p_x: f32, p_y: f32, a: &Vertex, b: &Vertex, c: &Vertex)  -> (f32, f32, f32) {
    let a_x = a.transformed_position.x;
    let b_x = b.transformed_position.x;
    let c_x = c.transformed_position.x;
    let a_y = a.transformed_position.y;
    let b_y = b.transformed_position.y;
    let c_y = c.transformed_position.y;

    let area = (b_y - c_y) * (a_x - c_x) + (c_x - b_x) * (a_y - c_y);

    if area.abs() < 1e-10  {
        return (-1.0, -1.0, -1.0);
    }
    
    let w = ((b_y - c_y) * (p_x - c_x) + (c_x - b_x) * (p_y - c_y)) / area;
    let v = ((c_y - a_y) * (p_x - c_x) + (a_x - c_x) * (p_y - c_y)) / area;
    let u = 1.0 - w - v;

    (w, v, u)
}

pub fn triangle(
    v1: &Vertex, v2: &Vertex, v3: &Vertex,
    light: &Light,
    uniforms: &crate::Uniforms, // Necesitarás los uniforms para el fragment shader
    framebuffer: &mut crate::Framebuffer,
    fragment_shader: fn(&Fragment, &crate::Uniforms, &Light) -> Vector3,
) {
    // let mut fragments = Vec::new();
    let base_color = Vector3::new(0.2,0.9,0.7);

    let min_x = (v1.transformed_position.x.min(v2.transformed_position.x).min(v3.transformed_position.x).floor() as i32)
    .max(0);
let max_x = (v1.transformed_position.x.max(v2.transformed_position.x).max(v3.transformed_position.x).ceil() as i32)
    .min(framebuffer.width - 1);
let min_y = (v1.transformed_position.y.min(v2.transformed_position.y).min(v3.transformed_position.y).floor() as i32)
    .max(0);
let max_y = (v1.transformed_position.y.max(v2.transformed_position.y).max(v3.transformed_position.y).ceil() as i32)
    .min(framebuffer.height - 1);

// Iterar sobre cada píxel en el cuadro delimitador
for y in min_y..=max_y {
    for x in min_x..=max_x {
        let p_x = x as f32 + 0.5; // Muestra de centro del píxel
        let p_y = y as f32 + 0.5;

        // Calcular coordenadas baricéntricas
        let (w1, w2, w3) = barycentric_coordinates(p_x, p_y, v1, v2, v3);

        // Verificar si el punto está dentro del triángulo
        if w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0 {
            // Interpolación de normales usando coordenadas baricéntricas
            let interpolated_normal = Vector3::new(
             w1 * v1.transformed_normal.x + w2 * v2.transformed_normal.x + w3 * v3.transformed_normal.x,
             w1 * v1.transformed_normal.y + w2 * v2.transformed_normal.y + w3 * v3.transformed_normal.y,
             w1 * v1.transformed_normal.z + w2 * v2.transformed_normal.z + w3 * v3.transformed_normal.z,
            );

            // Normalizar la normal interpolada
            let normal_length = (interpolated_normal.x * interpolated_normal.x +
                                 interpolated_normal.y * interpolated_normal.y +
                                 interpolated_normal.z * interpolated_normal.z).sqrt();

            let mut normalized_normal = interpolated_normal;
            if normal_length > 0.0 {
                normalized_normal.x /= normal_length;
                normalized_normal.y /= normal_length;
                normalized_normal.z /= normal_length;
            }

            // Calcular la posición en el espacio mundial para este fragmento
            let world_pos = Vector3::new(
            w1 * v1.position.x + w2 * v2.position.x + w3 * v3.position.x ,
            w1 * v1.position.y + w2 * v2.position.y + w3 * v3.position.y ,
            w1 * v1.position.z + w2 * v2.position.z + w3 * v3.position.z ,
        ); 
        // Dirección de la luz (desde la superficie hacia la luz) para este fragmento
        let mut light_dir = Vector3::new(
             light.position.x - world_pos.x,
             light.position.y - world_pos.y,
             light.position.z - world_pos.z,
        );
// Normalizar la dirección de la luz
        let light_length = (light_dir.x * light_dir.x + light_dir.y * light_dir.y + light_dir.z * light_dir.z).sqrt();
        if light_length >0.0{
        light_dir.x /= light_length;
        light_dir.y /= light_length;
        light_dir.z /= light_length;
        }

// Calcular la intensidad de la iluminación por fragmento usando la normal interpolada (sombrado lambertiano)
let intensity = (normalized_normal.x * light_dir.x + normalized_normal.y * light_dir.y + normalized_normal.z * light_dir.z).max(0.0);

// Aplicar sombreado a color base
let shaded_color = Vector3::new(
base_color.x * intensity,
 base_color.y * intensity,
 base_color.z * intensity,
);
// Interpolar la profundidad usando las coordenadas baricéntricas
let depth = w1 * v1.transformed_position.z + w2 * v2.transformed_position.z + w3 * v3.transformed_position.z;

let fragment = Fragment::new(p_x, p_y, shaded_color, depth); // 'shaded_color' es temporal
let final_color = fragment_shader(&fragment, uniforms, light);

framebuffer.point(
                    x, // Usa el 'x' e 'y' enteros del bucle
                    y,
                    fragment.depth,
                    final_color,
                );
                
// Agregar el fragmento al buffer de fragmentos
// fragments.push(Fragment::new(p_x, p_y, shaded_color, depth));
    }
}
}
// fragments
}
    
