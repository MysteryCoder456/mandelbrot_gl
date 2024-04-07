#version 330

layout(location = 0) in vec2 aPos;

uniform mat4 view;

void main() {
    gl_Position = view * vec4(aPos, 0.0, 1.0);
}
