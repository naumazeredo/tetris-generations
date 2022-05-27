#version 330 core

in vec4 frag_color;
in vec2 frag_uv;

uniform sampler2D u_texture;

out vec4 out_color;

void main() {
  // output color = color of the texture at the specific UV
  out_color = frag_color * texture(u_texture, frag_uv.st);
}
