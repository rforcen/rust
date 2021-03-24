#version 410

const uint black = 0xff000000u;

out uint out_color;

uniform int         n_points;
uniform ivec2       size;
uniform sampler2D   points; // points (x, y, color) all [0..1]

vec2  get_point(int i) {  return texture(points, vec2(i/3./n_points, 0.0)).xy;    }
float get_color(int i) {  return texture(points, vec2(i/3./n_points, 0.0)).z;   }

uint f2uint(float f) {  return black | (uint(f * float(0xffffffu)) & 0x00ffffffu );  } // 0..1 -> 0..0xffffff rgb

uint generate_point() {
    int   width= size.x, height=size.y;
    
    vec2 current_point = vec2( float(gl_VertexID % width) / width, 
                               float(gl_VertexID / width) / height );

    int   ind  = -1;
    float dist = 1e32, circ_diam=1e-3;

    for (int it = 0; it < n_points; it++) {
        float d = distance( get_point(it), current_point );

        if (d < circ_diam) return black; // draw center circle??
        if (d < dist) { dist = d;  ind = it; }
    }
    return (ind != -1) ? f2uint( get_color(ind) ) : black; 
}

void main() {  
    out_color = generate_point();   
}          