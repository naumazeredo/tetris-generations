#version 330 core

// input vertex data, different for all executions of this shaders
layout(location = 0) in vec3 position;
layout(location = 1) in vec4 color;
layout(location = 2) in vec2 uv;

// output data; will be interpolated for each fragment
out vec4 frag_color;
out vec2 frag_uv;

uniform mat4 u_view_mat;
uniform mat4 u_proj_mat;

void main() {
  frag_uv = uv;
  frag_color = color;

  gl_Position = u_proj_mat * u_view_mat * vec4(position, 1);
}
