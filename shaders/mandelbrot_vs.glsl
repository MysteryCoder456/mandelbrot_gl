#version 330

layout(location = 0) in vec2 aPos;

out vec2 coord;

uniform vec2 pan;
uniform float zoom;

void main() {
    gl_Position = vec4(aPos, 0.0, 1.0);
    coord = aPos / zoom + pan;
}
