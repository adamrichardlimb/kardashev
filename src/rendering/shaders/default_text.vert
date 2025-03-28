#version 330 core

layout (location = 0) in vec2 pos;
layout (location = 1) in vec2 uv;

uniform vec2 screen_size; // in pixels
uniform vec2 screen_pos;  // top-left position of the text quad in pixels
uniform vec2 scale;       // size of the text quad in pixels

out vec2 frag_uv;

void main() {
    // Convert local quad space (0..1) into pixel space
    vec2 pixel_pos = screen_pos + (pos * scale);

    // Convert to clip space (-1..1)
    vec2 zero_to_one = pixel_pos / screen_size;
    vec2 clip_space = zero_to_one * 2.0 - 1.0;

    // Flip Y axis to match top-left origin
    gl_Position = vec4(clip_space.x, -clip_space.y, 0.0, 1.0);
    frag_uv = uv;
}

