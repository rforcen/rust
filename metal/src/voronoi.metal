//
//  Voronoi.metal
//

/*
rm -f Voronoi.metallib *air
xcrun -sdk macosx metal -c *metal
xcrun -sdk macosx metallib *air -o Voronoi.metallib
rm -f *air
echo 'generated Voronoi.metallib'
*/

#include <metal_stdlib>
using namespace metal;

typedef uint32_t color;  // aa bb gg rr  32 bit color
typedef uint8_t  byte;

struct VoronoiInput {
  device color* pixels;

  device uint16_t* x;
  device uint16_t* y;
  device color* color;
  
  device uint *width;
  device uint *n_points;
};


inline int sqMag(uint px, uint py, uint x, uint y) {
  int xd = x - px;
  int yd = y - py;
  return (xd * xd) + (yd * yd);
}

color genPixel(uint i, uint j, device VoronoiInput&input) {
  int ind = -1, dist = INT_MAX;

  for (uint it = 0; it < *input.n_points; it++) {
    int d = sqMag(input.x[it], input.y[it], i, j);
    if (d < 4) return 0xff000000;
    if (d < dist) {
      dist = d;
      ind = (int)it;
    }
  }

  return 0xff000000 | ((ind > -1) ? input.color[ind] : 0);
}



kernel void Voronoi(device VoronoiInput&input [[ buffer(0) ]],
                    uint2 position [[thread_position_in_grid]] ) {

  uint w = *input.width;
  uint index = position.x+position.y*w;

  if (index < w*w)
    input.pixels[index] = genPixel(position.x, position.y, input);
}
