#version 330

in vec2 coord;
out vec4 FragColor;

void main() {
    // TODO: implement Mandelbrot fractal logic
    float val = float(int(coord.x) % 10 == 0 || int(coord.y) % 10 == 0);
    FragColor = vec4(val * 0.1, val * 0.1, val * 0.3, 1.0);
}
