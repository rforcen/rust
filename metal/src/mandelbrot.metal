//
//  Mandelbrot.metal
//  Mandelbrot
//
//  Created by asd on 20/04/2019.
//  Copyright © 2019 voicesync. All rights reserved.
//

#include <metal_stdlib>
using namespace metal;

#include "mandelbrot.h"

struct MandelInput {
    color device*pixels;
    range device&rng;    // range
    uint  device&side;   // side x side
    uint  device&iters;   // iters
};

kernel void Mandelbrot( MandelInput device &input[[ buffer(0) ]],                
                        uint2 position [[thread_position_in_grid]] )
{
    class Mandelbrot mb(input.rng, input.side, input.side, input.iters);
    
    input.pixels[position.x + position.y * input.side] = mb.generateZ(position.x, position.y, 4);
}
