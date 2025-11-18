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
let sun_fragment: fn(&Fragment, &Uniforms, &Light) -> Vector3 = ultra_mega_fragment_shader;
// posición base del sol (puedes dejar en el origen)
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



    while !window.window_should_close() {
        let elapsed = start_time.elapsed().as_secs_f32();
        camera.process_input(&window);

        framebuffer.clear();
        framebuffer.set_current_color(Color::new(200, 200, 255, 255));

        let view_matrix = camera.get_view_matrix();
        let projection_matrix = create_projection_matrix(PI / 3.0, window_width as f32 / window_height as f32, 0.1, 100.0);
        let viewport_matrix = create_viewport_matrix(0.0, 0.0, window_width as f32, window_height as f32);

     
      for planet in &planets {
        let angle = elapsed * planet.orbit_speed + planet.orbit_phase;
        let px = angle.cos() * planet.orbit_radius;
        let pz = angle.sin() * planet.orbit_radius;
        let py = planet.height;

        // Traslación orbital (world position)
        let planet_pos = Vector3::new(px, py, pz);

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

    // --------- render de la nave pegada a la cámara (igual que antes) ----------
    let mut forward = Vector3::new(
        camera.target.x - camera.eye.x,
        camera.target.y - camera.eye.y,
        camera.target.z - camera.eye.z,
    );
    let forward_len = (forward.x * forward.x + forward.y * forward.y + forward.z * forward.z).sqrt();
    if forward_len > 1e-6 {
        forward = Vector3::new(forward.x / forward_len, forward.y / forward_len, forward.z / forward_len);
    } else {
        forward = Vector3::new(0.0, 0.0, -1.0);
    }

    let forward_offset = 1.4_f32;
    let down_offset = -0.35_f32;

    let ship_pos = Vector3::new(
        camera.eye.x + forward.x * forward_offset,
        camera.eye.y + forward.y * forward_offset + down_offset,
        camera.eye.z + forward.z * forward_offset,
    );

    let ship_rotation = Vector3::new(camera.pitch, camera.yaw, 0.0);
    let ship_scale = 0.35_f32;

    let model_matrix_ship = create_model_matrix(ship_pos, ship_scale, ship_rotation);
    let uniforms_ship = Uniforms {
        model_matrix: model_matrix_ship,
        view_matrix,
        projection_matrix,
        viewport_matrix,
        time: elapsed,
    };

    render(                                      
        &mut framebuffer,
        &uniforms_ship,
        &vertex_nave,
        &light,
        &vertex_shader_nave, 
        fragment_shader_nave,
    );
    


let pixel_bytes: &[u8] = unsafe {

    let raw_ptr = framebuffer.color_buffer.data();

    let pixel_count = (framebuffer.width * framebuffer.height) as usize;
    
    let byte_count = pixel_count * std::mem::size_of::<Color>();

    let u8_ptr = raw_ptr as *const u8;

    std::slice::from_raw_parts(u8_ptr, byte_count)
};

// Ahora sí, actualizamos la textura con el slice de bytes
screen_texture.update_texture(pixel_bytes);

let mut d = window.begin_drawing(&raylib_thread);
    d.clear_background(Color::BLACK); // o tu color de fondo
    d.draw_texture(&screen_texture, 0, 0, Color::WHITE);


        thread::sleep(Duration::from_millis(16));
    }
}
