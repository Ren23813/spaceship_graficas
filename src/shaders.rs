//shaders.rs
use raylib::prelude::*;
use crate::{light::Light, vertex::Vertex};
use crate::Uniforms;
use crate::fragment::Fragment;
use noise::{NoiseFn, Simplex, SuperSimplex};
// use rand::random;


fn project_world_to_screen(light_pos: Vector3, uniforms: &Uniforms) -> Vector2 {
    // light_pos -> vec4
    let pos4 = Vector4::new(light_pos.x, light_pos.y, light_pos.z, 1.0);
    // world -> view
    let view_pos = multiply_matrix_vector4(&uniforms.view_matrix, &pos4);
    // view -> clip
    let clip_pos = multiply_matrix_vector4(&uniforms.projection_matrix, &view_pos);

    let ndc = if clip_pos.w != 0.0 {
        Vector3::new(clip_pos.x / clip_pos.w, clip_pos.y / clip_pos.w, clip_pos.z / clip_pos.w)
    } else {
        Vector3::new(clip_pos.x, clip_pos.y, clip_pos.z)
    };

    let ndc4 = Vector4::new(ndc.x, ndc.y, ndc.z, 1.0);
    let screen = multiply_matrix_vector4(&uniforms.viewport_matrix, &ndc4);
    Vector2::new(screen.x as f32, screen.y as f32)
}

// attenuation simple tipo gaussian para glow en pantalla
fn screen_attenuation(dist: f32, range: f32) -> f32 {
    // clamp para evitar división por cero; la forma gaussian da un bonito fade
    let r = if range <= 0.0 { 1.0 } else { range };
    (- (dist * dist) / (2.0 * r * r)).exp()
}

fn multiply_matrix_vector4(matrix: &Matrix, vector: &Vector4) -> Vector4 {
    Vector4::new(
        matrix.m0 * vector.x + matrix.m4 * vector.y + matrix.m8 * vector.z + matrix.m12 * vector.w,
        matrix.m1 * vector.x + matrix.m5 * vector.y + matrix.m9 * vector.z + matrix.m13 * vector.w,
        matrix.m2 * vector.x + matrix.m6 * vector.y + matrix.m10 * vector.z + matrix.m14 * vector.w,
        matrix.m3 * vector.x + matrix.m7 * vector.y + matrix.m11 * vector.z + matrix.m15 * vector.w,
    )
}

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
  let position_vec4 = Vector4::new(
    vertex.position.x, //tan
    vertex.position.y, //cos
    vertex.position.z,
    1.0
  );

  let world_position = multiply_matrix_vector4(&uniforms.model_matrix, &position_vec4);

  let view_position = multiply_matrix_vector4(&uniforms.view_matrix, &world_position);

  let clip_position = multiply_matrix_vector4(&uniforms.projection_matrix, &view_position);

  let ndc = if clip_position.w != 0.0 {
      Vector3::new(
          clip_position.x / clip_position.w,
          clip_position.y / clip_position.w,
          clip_position.z / clip_position.w,
      )
  } else {
      Vector3::new(clip_position.x, clip_position.y, clip_position.z)
  };

  let ndc_vec4 = Vector4::new(ndc.x, ndc.y, ndc.z, 1.0);
  let screen_position = multiply_matrix_vector4(&uniforms.viewport_matrix, &ndc_vec4);

  let transformed_position = Vector3::new(
      screen_position.x,
      screen_position.y,
      screen_position.z,
  );

  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    transformed_position,
    transformed_normal: transform_normal(&vertex.normal, &uniforms.model_matrix),
  }
}

pub fn vertex_shader2(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
  let position_vec4 = Vector4::new(
    vertex.position.x, //tan
    vertex.position.y, //cos
    vertex.position.z,
    1.0
  );

  let world_position = multiply_matrix_vector4(&uniforms.model_matrix, &position_vec4);

  let view_position = multiply_matrix_vector4(&uniforms.view_matrix, &world_position);

  let clip_position = multiply_matrix_vector4(&uniforms.projection_matrix, &view_position);

  let ndc = if clip_position.w != 0.0 {
      Vector3::new(
          clip_position.x / clip_position.w,
          clip_position.y / clip_position.w,
          clip_position.z / clip_position.w,
      )
  } else {
      Vector3::new(clip_position.x, clip_position.y, clip_position.z)
  };

  let ndc_vec4 = Vector4::new(ndc.x, ndc.y, ndc.z, 1.0);
  let screen_position = multiply_matrix_vector4(&uniforms.viewport_matrix, &ndc_vec4);

  let transformed_position = Vector3::new(
      screen_position.x,
      screen_position.y,
      screen_position.z,
  );

  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    transformed_position,
    transformed_normal: transform_normal(&vertex.normal, &uniforms.model_matrix),
  }
}

pub fn vertex_shader3(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
  let position_vec4 = Vector4::new(
    vertex.position.x.tan(), 
    vertex.position.y.cos(), 
    vertex.position.z,
    1.0
  );

  let world_position = multiply_matrix_vector4(&uniforms.model_matrix, &position_vec4);

  let view_position = multiply_matrix_vector4(&uniforms.view_matrix, &world_position);

  let clip_position = multiply_matrix_vector4(&uniforms.projection_matrix, &view_position);

  let ndc = if clip_position.w != 0.0 {
      Vector3::new(
          clip_position.x / clip_position.w,
          clip_position.y / clip_position.w,
          clip_position.z / clip_position.w,
      )
  } else {
      Vector3::new(clip_position.x, clip_position.y, clip_position.z)
  };

  let ndc_vec4 = Vector4::new(ndc.x, ndc.y, ndc.z, 1.0);
  let screen_position = multiply_matrix_vector4(&uniforms.viewport_matrix, &ndc_vec4);

  let transformed_position = Vector3::new(
      screen_position.x,
      screen_position.y,
      screen_position.z,
  );

  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    transformed_position,
    transformed_normal: transform_normal(&vertex.normal, &uniforms.model_matrix),
  }
}

fn transform_normal(normal: &Vector3, model_matrix: &Matrix) -> Vector3 {
    let normal_vec4 = Vector4::new(normal.x, normal.y, normal.z, 0.0);

    let transformed_normal_vec4 = multiply_matrix_vector4(model_matrix, &normal_vec4);

    let mut transformed_normal = Vector3::new(
        transformed_normal_vec4.x,
        transformed_normal_vec4.y,
        transformed_normal_vec4.z,
    );
    
    transformed_normal.normalize();
    transformed_normal
}
pub fn fragment_shader1(fragment: &Fragment, uniforms: &Uniforms, light:&Light) -> Vector3 {
    let base_color = fragment.color+0.2;

    let x_pattern = (fragment.position.x / 20.0) * 0.35 + 0.1;
    let y_pattern = (fragment.position.y / 250.0).cos() * 0.9 + 0.1;

    let pattern_color = Vector3::new(
        x_pattern,
        y_pattern,
        (x_pattern + y_pattern).cos() - 0.9,
    );

    base_color * 0.4 + pattern_color * 0.6
}

pub fn fragment_shader2(fragment: &Fragment, uniforms: &Uniforms, light:&Light) -> Vector3 {
    let base_color = fragment.color-0.6;

    let x_pattern = (fragment.position.x / 20.0).sin() * 0.5 + 0.5;
    let y_pattern = (fragment.position.y / 10.0).cos() * 0.1 + 0.3;

    let pattern_color = Vector3::new(
        x_pattern,
        y_pattern,
        (x_pattern + y_pattern).atan(),
    );

    base_color * 0.5 + pattern_color * 0.5
}

pub fn fragment_shader3(fragment: &Fragment, uniforms: &Uniforms, light:&Light) -> Vector3 {
    let base_color = Vector3::new(0.0, 0.8, 0.6); // verde turquesa base

    let ripple = ((fragment.position.x * 0.05).sin() + (fragment.position.y * 0.05).cos()) * 0.5 + 0.5;

    let wave_color = Vector3::new(
        base_color.x * ripple,
        base_color.y * ripple,
        base_color.z + 0.2 * ripple,
    );

    wave_color
}

pub fn ultra_mega_vertex_shader(vertex: &Vertex, uniforms: &Uniforms) ->Vertex{
  let position_vec4 = Vector4::new(
    vertex_shader(vertex, uniforms).position.x * vertex_shader2(vertex, uniforms).position.x * vertex_shader3(vertex, uniforms).position.x *3.5, 
    vertex_shader(vertex, uniforms).position.y * vertex_shader2(vertex, uniforms).position.y * vertex_shader3(vertex, uniforms).position.y *3.5, 
    vertex_shader(vertex, uniforms).position.z * vertex_shader2(vertex, uniforms).position.z * vertex_shader3(vertex, uniforms).position.z *3.5,
    1.0
  );

  let world_position = multiply_matrix_vector4(&uniforms.model_matrix, &position_vec4);

  let view_position = multiply_matrix_vector4(&uniforms.view_matrix, &world_position);

  let clip_position = multiply_matrix_vector4(&uniforms.projection_matrix, &view_position);

  let ndc = if clip_position.w != 0.0 {
      Vector3::new(
          clip_position.x / clip_position.w,
          clip_position.y / clip_position.w,
          clip_position.z / clip_position.w,
      )
  } else {
      Vector3::new(clip_position.x, clip_position.y, clip_position.z)
  };

  let ndc_vec4 = Vector4::new(ndc.x, ndc.y, ndc.z, 1.0);
  let screen_position = multiply_matrix_vector4(&uniforms.viewport_matrix, &ndc_vec4);

  let transformed_position = Vector3::new(
      screen_position.x,
      screen_position.y,
      screen_position.z,
  );

  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    transformed_position,
    transformed_normal: transform_normal(&vertex.normal, &uniforms.model_matrix),
  }
}

pub fn ultra_mega_fragment_shader(fragment: &Fragment, uniforms: &Uniforms, light:&Light) -> Vector3 {
    let simplex: SuperSimplex= SuperSimplex::new(42); 
    let freq = 0.05;

    let x = fragment.position.x * freq;
    let y = fragment.position.y * freq;
    let t = uniforms.time * 0.8;

    let raw = simplex.get([x as f64, y as f64, t as f64]);
    let noise_val = ((raw + 1.0) * 0.5) as f32;

    let color = 
        // fragment_shader1(fragment, uniforms)+
        Vector3::new(1.0, 0.0, 1.0)     //R-G-B
        + fragment_shader2(fragment, uniforms,light)
        + fragment_shader3(fragment, uniforms,light)
        ;

    let light_screen = project_world_to_screen(light.position, uniforms);
    let dx = fragment.position.x - light_screen.x;
    let dy = fragment.position.y - light_screen.y;
    let dist = (dx * dx + dy * dy).sqrt();

    let local_light = 5.0;
    // rango en píxeles: usa light.range
    let att = screen_attenuation(dist, light.range) * light.intensity * local_light;
    
    // sumar emisión (additiva) con el color de la luz
    let mut color2 = color + light.color * att;

      color2 = Vector3::new(
        color2.x.min(1.0),
        color2.y.min(1.0),
        color2.z.min(1.0),
    );

    color2 * Vector3::new(noise_val, noise_val, noise_val)
}