#version 330

// in vec2 position;
out uint out_color;
uniform ivec2 size;

void main() {
	float i=float(gl_VertexID), j=i;
	out_color = uint(gl_VertexID+size.x+size.y); // vec2(i+size.y, i * size.x/123.9);
}