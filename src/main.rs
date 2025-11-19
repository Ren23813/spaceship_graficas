// main.rs

mod framebuffer;
mod line;
mod triangle;
mod obj;
mod matrix;
mod fragment;
mod vertex;
mod light;
mod shaders;
mod camera;

use obj::Obj;
use triangle::triangle;
use framebuffer::Framebuffer;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use std::f32::consts::PI;
use matrix::{create_model_matrix,create_projection_matrix,create_viewport_matrix,multiply_matrix_vector4};
use light::Light;
use vertex::Vertex;
use shaders::{fragment_shader1,fragment_shader2,fragment_shader3,vertex_shader,vertex_shader2,vertex_shader3,ultra_mega_vertex_shader,ultra_mega_fragment_shader};
use camera::Camera;
use rand::Rng;

use crate::{fragment::Fragment, matrix::create_view_matrix, shaders::{fragment_shader_nave, vertex_shader_nave}};


pub struct Uniforms{
    pub model_matrix: Matrix,
    pub view_matrix: Matrix,
    pub projection_matrix: Matrix,
    pub viewport_matrix: Matrix,
    pub time:f32
}


fn render(framebuffer: &mut Framebuffer, 
    uniforms: &Uniforms, 
    vertex_array: &[Vertex], 
    light: &Light,
    vertex_shader: &dyn Fn(&Vertex, &Uniforms) -> Vertex,  
    fragment_shader: fn(&Fragment, &Uniforms, &Light) -> Vector3,
    ) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly Stage
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage
    // let mut fragments = Vec::new();
    for tri in &triangles {
        triangle(
            &tri[0], &tri[1], &tri[2], 
            light, 
            uniforms, 
            framebuffer, 
            fragment_shader
        );    
    }

    // Fragment Processing Stage
    // for fragment in fragments {
    // let final_color = fragment_shader(&fragment, uniforms, light);          
    //     framebuffer.point(
    //         fragment.position.x as i32,
    //         fragment.position.y as i32,
    //         fragment.depth,
    //         final_color,
    //     );
    // }
}

fn generate_stars(count: usize) -> Vec<Vector3> {
    let mut rng = rand::thread_rng();
    let mut stars = Vec::with_capacity(count);

    for _ in 0..count {
        // Generamos un punto aleatorio en una esfera unitaria
        let theta = rng.gen_range(0.0..2.0 * PI);
        let phi = rng.gen_range(0.0..PI);
        
        let x = phi.sin() * theta.cos();
        let y = phi.sin() * theta.sin();
        let z = phi.cos();

        // Lo guardamos. No necesitamos escalarlo a "lejos" porque
        // manipularemos la proyección, pero para visualización mental,
        // imagínalas en una esfera de radio 1.
        stars.push(Vector3::new(x, y, z));
    }
    stars
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let start_time = std::time::Instant::now();

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("nave")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width, window_height);
    let mut screen_texture = window
    .load_texture_from_image(&raylib_thread, &framebuffer.color_buffer)
    .expect("Failed to create initial texture from framebuffer");

    let mut translation = Vector3::new(0.0, 0.0,0.0);
    let mut scale = 1.0;
    let mut rotation = Vector3:: new(0.0, 0.0, 0.0);
let mut light = Light::new_with_params(
    Vector3::new(0.0, 0.0, 0.0),            // inicial; lo actualizaremos cada frame al sol_pos
    Vector3::new(1.0, 0.95, 0.8),
    6.0,
    400.0,
);


struct Planet {
    orbit_radius: f32,
    orbit_speed: f32, // rad/s
    orbit_phase: f32, // offset
    spin_speed: f32,  // rad/s (rotación propia)
    scale: f32,
    height: f32,      // offset Y
    vertex: Box<dyn Fn(&Vertex, &Uniforms) -> Vertex>,
    fragment: fn(&Fragment, &Uniforms, &Light) -> Vector3,
}

let planets: Vec<Planet> = vec![
    Planet {
        orbit_radius: 5.0,
        orbit_speed: 0.6,
        orbit_phase: 0.0,
        spin_speed: 1.2,
        scale: 0.8,
        height: 0.2,
        vertex: Box::new(vertex_shader),        // shader "1"
        fragment: fragment_shader1,
    },
    Planet {
        orbit_radius: 8.0,
        orbit_speed: 0.35,
        orbit_phase: 1.0,
        spin_speed: 0.8,
        scale: 1.1,
        height: 0.5,
        vertex: Box::new(vertex_shader2),       // shader "2"
        fragment: fragment_shader2,
    },
    Planet {
        orbit_radius: 11.0,
        orbit_speed: 0.18,
        orbit_phase: 2.4,
        spin_speed: 0.5,
        scale: 0.6,
        height: -0.3,
        vertex: Box::new(vertex_shader3),       // shader "3"
        fragment: fragment_shader3,
    },
];

let sun_scale = 2.5_f32;
let sun_vertex: Box<dyn Fn(&Vertex, &Uniforms) -> Vertex> = Box::new(ultra_mega_vertex_shader);
let sun_fragment= ultra_mega_fragment_shader;
let sun_base_pos = Vector3::new(0.0, 0.0, 0.0);


    let obj = Obj::load("models/sphere.obj").expect("Error al leer archivo"); //se puede cambiar al spaceship 
    let vertex_array = obj.get_vertex_array();
    let nave = Obj::load("models/improvisada.obj").expect("Error al leer archivo");
    let vertex_nave = nave.get_vertex_array();

    let mut camera = Camera::new(
        Vector3::new(0.0, 0.0, 5.0), // eye
        Vector3::new(0.0, 0.0, 0.0), // target
        Vector3::new(0.0, 1.0, 0.0), // up
    );

    framebuffer.set_background_color(Color::new(35,6, 48,255));

    let model_matrix = create_model_matrix(translation, scale, rotation);
    let model_matrix_bottom = create_model_matrix(
        Vector3::new(0.0, -2.5, 0.0),  // move down
        scale,
        Vector3::new(PI, 0.0, 0.0),  // flip on Y axis
    );

let stars = generate_stars(2000);

    while !window.window_should_close() {
        let elapsed = start_time.elapsed().as_secs_f32();
        camera.process_input(&window);

        framebuffer.clear();
        let front = Vector3::new(
    camera.target.x - camera.eye.x,
    camera.target.y - camera.eye.y,
    camera.target.z - camera.eye.z,
).normalized();

//    Creamos una vista desde (0,0,0) mirando hacia esa dirección.
let view_rotate_only = create_view_matrix(
    Vector3::zero(), // Ojo en el origen
    front,           // Mirando hacia donde mira la cámara real
    camera.up
);
        let projection_matrix = create_projection_matrix(PI / 3.0, window_width as f32 / window_height as f32, 0.1, 100.0);

// 2. Dibujar cada estrella
for star in &stars { // 'stars' es el Vec que creaste al inicio
    // A. Transformar (Model es identidad, así que pasamos directo a View)
    //    Convertimos el Vector3 a Vector4 para multiplicar
    let vertex = Vector4::new(star.x, star.y, star.z, 1.0);

    // B. Aplicar Vista (Solo rotación)
    let view_pos = multiply_matrix_vector4(&view_rotate_only, &vertex);

    // C. Aplicar Proyección
    let proj_pos = multiply_matrix_vector4(&projection_matrix, &view_pos);

    // D. División de Perspectiva (Perspective Divide)
    //    Si w <= 0, la estrella está detrás de la cámara, no la dibujamos
    if proj_pos.w > 0.0 {
        let ndc_x = proj_pos.x / proj_pos.w;
        let ndc_y = proj_pos.y / proj_pos.w;

        // E. Viewport (Pantalla)
        //    Solo dibujamos si está dentro de los límites normalizados (-1 a 1)
        if ndc_x >= -1.0 && ndc_x <= 1.0 && ndc_y >= -1.0 && ndc_y <= 1.0 {
            let screen_x = (ndc_x + 1.0) * 0.5 * window_width as f32;
            let screen_y = (1.0 - ndc_y) * 0.5 * window_height as f32; // Invertimos Y

            // F. Dibujar punto (Color blanco o grisáceo)
            //    IMPORTANTE: Dibujamos directo al buffer de color, IGNORANDO el depth buffer.
            //    Así las estrellas siempre quedan "al fondo".
            framebuffer.set_current_color(Color::WHITE);
            framebuffer.set_pixel(screen_x as i32, screen_y as i32);
        }
    }
}
        framebuffer.set_current_color(Color::new(200, 200, 255, 255));

        let view_matrix = camera.get_view_matrix();
        let viewport_matrix = create_viewport_matrix(0.0, 0.0, window_width as f32, window_height as f32);

     
      for planet in &planets {
        let angle = elapsed * planet.orbit_speed + planet.orbit_phase;
        let px = angle.cos() * planet.orbit_radius;
        let pz = angle.sin() * planet.orbit_radius;
        let py = planet.height;

        // Traslación orbital (world position)
        let planet_pos = Vector3::new(px, py, pz);

        let dist_to_planet = (
            (camera.eye.x - planet_pos.x).powi(2) + 
            (camera.eye.y - planet_pos.y).powi(2) + 
            (camera.eye.z - planet_pos.z).powi(2)
        ).sqrt();

        let collision_radius = planet.scale * 1.6;
        if dist_to_planet < collision_radius { 
            // camera.distance += 10.0;
            //en teoría esto lo resuelve, pero no siempre funciona la empujada para atras
        }
        // Rotación propia (spin) aplicada en Y
        let self_angle = elapsed * planet.spin_speed;
        let planet_rot = Vector3::new(0.0, self_angle, 0.0);

        let model_matrix_planet = create_model_matrix(planet_pos, planet.scale, planet_rot);

        let uniforms_planet = Uniforms {
            model_matrix: model_matrix_planet,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time: elapsed,
        };

        // render: pasamos vertex shader del planeta y fragment shader del planeta
        render(                                                  //AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
            &mut framebuffer,
            &uniforms_planet,
            &vertex_array,
            &light,
            planet.vertex.as_ref(),
            planet.fragment,
        );
    }

      let sun_pos = sun_base_pos; // Vector3::new(0.0, 0.0, 0.0);

    // actualizamos la posición de la luz para que ilumine desde el sol
    // recuerda que 'light' es mutable
    light.position = sun_pos;

    let sun_model = create_model_matrix(sun_pos, sun_scale, Vector3::new(0.0, elapsed * 0.2, 0.0));
    let uniforms_sun = Uniforms {
        model_matrix: sun_model,
        view_matrix,
        projection_matrix,
        viewport_matrix,
        time: elapsed,
    };

    render(                     //AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
        &mut framebuffer,
        &uniforms_sun,
        &vertex_array,
        &light,
        sun_vertex.as_ref(),
        sun_fragment,
    );

    let ship_scale = 0.001;
    framebuffer.clear_depth();

    let ship_pos_in_screen = Vector3::new(
            0.0,    // Centrada horizontalmente
            2.0,  // Un poco abajo (down_offset)
            -7.0    // Hacia adelante (forward_offset) para que no te corte la cara
        );
    let ship_rotation_fixed = Vector3::new(0.0, PI, 0.0);
    let model_matrix_ship = create_model_matrix(ship_pos_in_screen, ship_scale, ship_rotation_fixed);
    let static_view_matrix = Matrix::identity();

    let uniforms_ship = Uniforms {
        model_matrix: model_matrix_ship,
        view_matrix: static_view_matrix, // <--- AQUÍ ESTÁ LA MAGIA
        projection_matrix,               // La proyección sí debe ser la misma (perspectiva)
        viewport_matrix,
        time: elapsed,
    };

    render(
        &mut framebuffer,
        &uniforms_ship,
        &vertex_nave,
        &light,
        &vertex_shader_nave,  // Usa tus shaders de nave aquí
        fragment_shader_nave,
    );


    let pixel_bytes: &[u8] = unsafe {

        let raw_ptr = framebuffer.color_buffer.data();

        let pixel_count = (framebuffer.width * framebuffer.height) as usize;
        
        let byte_count = pixel_count * std::mem::size_of::<Color>();

        let u8_ptr = raw_ptr as *const u8;

        std::slice::from_raw_parts(u8_ptr, byte_count)
    };

    screen_texture.update_texture(pixel_bytes);

    let mut d = window.begin_drawing(&raylib_thread);
    d.clear_background(Color::BLACK); // o tu color de fondo
    d.draw_texture(&screen_texture, 0, 0, Color::WHITE);


        thread::sleep(Duration::from_millis(16));
    }
}
