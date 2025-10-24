//shaders.rs
use raylib::prelude::*;
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
// use rand::random;

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
    vertex.position.x,
    vertex.position.y,
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
pub fn fragment_shaders(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
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