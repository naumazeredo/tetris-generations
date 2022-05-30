#version 330 core

// input vertex data, different for all executions of this shaders
layout(location = 0) in vec3 position;
layout(location = 1) in vec4 color;
layout(location = 2) in vec2 uv;

// @TODO change this to uniform mat4 transform; ?
layout(location = 3) in vec2 pivot;
layout(location = 4) in float rotation;
layout(location = 5) in vec3 translation;

// output data; will be interpolated for each fragment
out vec4 frag_color;
out vec2 frag_uv;

uniform mat4 u_view_mat;
uniform mat4 u_proj_mat;

//uniform mat4 model_mat;

mat4 build_translation_matrix(vec3 t) {
  return mat4(
    vec4(1.0, 0.0, 0.0, 0.0),
    vec4(0.0, 1.0, 0.0, 0.0),
    vec4(0.0, 0.0, 1.0, 0.0),
    vec4(t.x, t.y, t.z, 1.0)
  );
}

mat4 build_2d_rotation_matrix(float r) {
  float a = radians(r);
  float s = sin(a);
  float c = cos(a);

  return mat4(
    vec4(c, -s, 0.0, 0.0),
    vec4(s, c,  0.0, 0.0),
    vec4(0.0, 0.0, 1.0, 0.0),
    vec4(0.0, 0.0, 0.0, 1.0)
  );
}

mat4 build_model_matrix() {
  mat4 pivot_mat = build_translation_matrix(vec3(-pivot.x, -pivot.y, 0.0));
  mat4 rot_mat = build_2d_rotation_matrix(rotation);
  mat4 trans_mat = build_translation_matrix(translation);

  return pivot_mat * rot_mat * trans_mat;
}

void main() {
  frag_uv = uv;
  frag_color = color;

  mat4 model_mat = build_model_matrix();

  gl_Position = u_proj_mat * u_view_mat * model_mat * vec4(position, 1);
}
