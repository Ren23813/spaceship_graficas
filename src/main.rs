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

use crate::{fragment::Fragment, matrix::create_view_matrix};


pub struct Uniforms{
    pub model_matrix: Matrix,
    pub view_matrix: Matrix,
    pub projection_matrix: Matrix,
    pub viewport_matrix: Matrix,
}


fn render(framebuffer: &mut Framebuffer, 
    uniforms: &Uniforms, 
    vertex_array: &[Vertex], 
    light: &Light,
    vertex_shader: &dyn Fn(&Vertex, &Uniforms) -> Vertex,  
    fragment_shader: fn(&Fragment, &Uniforms) -> Vector3,
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
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2], light));
    }

    // Fragment Processing Stage
    for fragment in fragments {
    let final_color = fragment_shader(&fragment, uniforms);            
        framebuffer.point(
            fragment.position.x as i32,
            fragment.position.y as i32,
            fragment.depth,
            final_color,
        );
    }
}

fn main() {
    let window_width = 1000;
    let window_height = 720;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("nave")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width, window_height);
    let mut translation = Vector3::new(0.0, 0.0,0.0);
    let mut scale = 1.0;
    let mut rotation = Vector3:: new(0.0, 0.0, 0.0);
    let light = Light::new(Vector3::new(5.0, 5.0, 5.0));

    let obj = Obj::load("models/sphere.obj").expect("Error al leer archivo"); //se puede cambiar al spaceship 
    let vertex_array = obj.get_vertex_array();
    let mut camera = Camera::new(
        Vector3::new(0.0, 0.0, 5.0), // eye
        Vector3::new(0.0, 0.0, 0.0), // target
        Vector3::new(0.0, 1.0, 0.0), // up
    );

    framebuffer.set_background_color(Color::new(35,6, 48,1));

    // estado del modo activo: 1, 2, o 3 (switch)
    let mut active_mode: u8 = 1; // default

    let model_matrix = create_model_matrix(translation, scale, rotation);
    let model_matrix_bottom = create_model_matrix(
        Vector3::new(0.0, -2.5, 0.0),  // move down
        scale,
        Vector3::new(PI, 0.0, 0.0),  // flip on Y axis
    );

    while !window.window_should_close() {
        camera.process_input(&window);

        // --- DETECTAR PULSACIONES (switch behavior) ---
        // Usamos is_key_pressed para que sea una pulsación única (toggle-like).
        if window.is_key_pressed(KeyboardKey::KEY_ONE) {
            active_mode = 1;
        } else if window.is_key_pressed(KeyboardKey::KEY_TWO) {
            active_mode = 2;
        } else if window.is_key_pressed(KeyboardKey::KEY_THREE) {
            active_mode = 3;
        } else if window.is_key_pressed(KeyboardKey::KEY_FOUR) {
            active_mode = 4;
        }

        framebuffer.clear();
        framebuffer.set_current_color(Color::new(200, 200, 255, 255));

        let view_matrix = camera.get_view_matrix();
        let projection_matrix = create_projection_matrix(PI / 3.0, window_width as f32 / window_height as f32, 0.1, 100.0);
        let viewport_matrix = create_viewport_matrix(0.0, 0.0, window_width as f32, window_height as f32);

        // --- ELECCION DE SHADERS PARA EL OBJETO SUPERIOR SEGUN active_mode ---
        let (vertex_top, fragment_top): (
            Box<dyn Fn(&Vertex, &Uniforms) -> Vertex>,
            fn(&Fragment, &Uniforms) -> Vector3
        ) = match active_mode {
            1 => (Box::new(vertex_shader), fragment_shader1),
            2 => (Box::new(vertex_shader2), fragment_shader2),
            3 => (Box::new(vertex_shader3), fragment_shader3),
            4 => (Box::new(ultra_mega_vertex_shader), ultra_mega_fragment_shader),
            _ => (Box::new(vertex_shader), fragment_shader1),
        };

        // Para la copia inferior (solo renderizamos si active_mode == 3)
        let (vertex_bottom, fragment_bottom): (
            Box<dyn Fn(&Vertex, &Uniforms) -> Vertex>,
            fn(&Fragment, &Uniforms) -> Vector3
        ) = (Box::new(vertex_shader), fragment_shader1); // valores por defecto si se llegara a usar

        // uniforms para la parte superior
        let uniforms = Uniforms {
            model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
        };

        // render superior (siempre)
        render(&mut framebuffer, &uniforms, &vertex_array, &light, &vertex_top.as_ref(), fragment_top);

        // Si el modo es 3, dibujamos la copia inferior (duplicado). Si quieres que la copia tenga
        // un fragment shader distinto, cámbialo aquí (por ejemplo fragment_shader2).
        if active_mode == 3 {
            let uniforms_bottom = Uniforms {
                model_matrix: model_matrix_bottom,
                view_matrix,
                projection_matrix,
                viewport_matrix,
            };
            // aquí usamos vertex_shader3 también para la copia inferior (si quieres que la copia sea
            // el resultado de vertex_shader3). Si en su lugar quieres que la copia sea la geometría
            // normal pero con otro fragment, cambia vertex_bottom por Box::new(vertex_shader).
            render(&mut framebuffer, &uniforms_bottom, &vertex_array, &light, vertex_top.as_ref(), fragment_top);
        }

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(16));
    }
}