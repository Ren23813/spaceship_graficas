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
use shaders::{fragment_shaders,vertex_shader};
use camera::Camera;

use crate::matrix::create_view_matrix;


pub struct Uniforms{
    pub model_matrix: Matrix,
    pub view_matrix: Matrix,
    pub projection_matrix: Matrix,
    pub viewport_matrix: Matrix,
}


// fn transform(vertex: Vector3, translation: Vector3, scale: f32, rotation: Vector3) -> Vector3 {
//     let model : Matrix = create_model_matrix(translation,scale,rotation);
//     let view:Matrix=create_view_matrix(Vector3::new(0.0, 0.0, 1.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.1));

//     let projection:Matrix = create_projection_matrix(PI/3.0, 1900.0/1200.0, 0.1, 100.0);
//     let viewport : Matrix = create_viewport_matrix(0.0, 0.0, 1900.0, 1200.0);
//     let vertex4=Vector4::new(vertex.x,vertex.y,vertex.z,1.0);

//     let world_transform = multiply_matrix_vector4(&model, &vertex4);
//     let view_transform = multiply_matrix_vector4(&model, &world_transform);
//     let projection_transform = multiply_matrix_vector4(&projection, &view_transform);
//     let transformed_vertex4 = multiply_matrix_vector4(&viewport, &projection_transform);
//     // let transformed_vertex4 = view_transform;

//     let transformed_vertex3 = Vector3::new(transformed_vertex4.x/transformed_vertex4.w, transformed_vertex4.y/transformed_vertex4.w, transformed_vertex4.z/transformed_vertex4.w);
    

    
//     transformed_vertex3
// }

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], light: &Light) {
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

        let final_color = fragment_shaders(&fragment, uniforms);
            
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

    while !window.window_should_close() {
        camera.process_input(&window);
        
        framebuffer.clear();
        framebuffer.set_current_color(Color::new(200, 200, 255, 255));
        
        let model_matrix = create_model_matrix(translation, scale, rotation);
        let view_matrix = camera.get_view_matrix();
        let projection_matrix = create_projection_matrix(PI / 3.0, window_width as f32 / window_height as f32, 0.1, 100.0);
        let viewport_matrix = create_viewport_matrix(0.0, 0.0, window_width as f32, window_height as f32);

        let uniforms = Uniforms {
            model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
        };

        render(&mut framebuffer, &uniforms, &vertex_array, &light);

        framebuffer.swap_buffers(&mut window, &raylib_thread);
        
        thread::sleep(Duration::from_millis(16));
    }
}