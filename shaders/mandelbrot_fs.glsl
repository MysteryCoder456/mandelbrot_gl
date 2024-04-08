#version 330

#define MAX_ITERATIONS 50
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
    // Compute iterations
    vec2 z = vec2(0.0);
    for (int i = 0; i < MAX_ITERATIONS; i++) {
        z = complex_product(z, z) + coord;
    }

    // Check if z is bounded
    if (distance(z, coord) < BOUNDED_THRESHOLD)
        FragColor = vec4(1.0);
    else
        FragColor = vec4(vec3(0.0), 1.0);
}
