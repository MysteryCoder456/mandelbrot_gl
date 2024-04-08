#version 330

#define MAX_ITERATIONS 1000
#define BOUNDED_THRESHOLD 2.0

in vec2 coord;
out vec4 FragColor;

// Product of two complex numbers
// Function treats x component as real and y component as imaginary
vec2 complex_product(vec2 a, vec2 b) {
    float real = a.x * b.x - a.y * b.y;
    float imag = a.x * b.y + a.y * b.x;
    return vec2(real, imag);
}

void main() {
    // Initial condition
    vec2 z = vec2(0.0); // z0 = 0 + 0i

    // Compute iterations
    int i = 0;
    while (i++ < MAX_ITERATIONS && length(z) < BOUNDED_THRESHOLD)
        z = complex_product(z, z) + coord; // f(z) = z^2 + c

    // TODO: More vibrant colors
    float val = i < MAX_ITERATIONS ? 1 - float(i) / MAX_ITERATIONS : 0.0;
    vec3 bgColor = vec3(0.3, 0.3, 0.7);
    FragColor = vec4(bgColor * val, 1.0);
}
