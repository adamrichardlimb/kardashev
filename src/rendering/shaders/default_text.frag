#version 330 core

in vec2 frag_uv;
out vec4 color;

uniform sampler2D text_texture;

void main() {
    color = texture(text_texture, frag_uv);
}
