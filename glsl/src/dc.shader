#version 330

// complex arithmetics: +,-, neg direct vec2 support
vec2 mul(vec2 a, vec2 b) { return vec2(a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x);  }
vec2 div(vec2 a, vec2 b) {
	float _div = (b.x * b.x) + (b.y * b.y);
	return vec2( ((a.x * b.x) + (a.y * b.y)) / _div, ((a.y * b.x) - (a.x * b.y)) / _div );
}

float cabs(vec2 a)     { return dot(a,a); } 
float sqmag(vec2 a)    { return dot(a,a); } 
float arg(vec2 a)      { return atan(a.y, a.x); }
						
vec2 cpow(vec2 a, float n) { // (𝑎+𝑖𝑏)𝑁=𝑟𝑁(cos(𝑁𝜃)+𝑖sin(𝑁𝜃))
	float rn = pow(length(a), n), na = n * arg(a);
	return vec2(rn * cos(na), rn * sin(na));
}

vec2 cpow(vec2 a, vec2 z) { 
	float c = z.x, d = z.y;
	float m = pow(sqmag(a), c / 2) * exp(-d * arg(a));
	float _re = m * cos(c * arg(a) + 1 / 2 * d * log(sqmag(a))),
			_im = m * sin(c * arg(a) + 1 / 2 * d * log(sqmag(a)));
	return vec2(_re, _im);
}

vec2 csqrt(vec2 z) {
	float a = length(z);
	return vec2(sqrt((a + z.x) / 2), sign(z.y) * sqrt((a - z.x) / 2));
}

vec2 clog(vec2 z)  { return vec2(log(length(z)), arg(z)); }

vec2 ccosh(vec2 z) { return vec2(cosh(z.x) * cos(z.y), sinh(z.x) * sin(z.y));  }
vec2 csinh(vec2 z) { return vec2(sinh(z.x) * cos(z.y), cosh(z.x) * sin(z.y));  }
vec2 csin(vec2 z)  { return vec2(sin(z.x) * cosh(z.y), cos(z.x) * sinh(z.y));  }
vec2 ccos(vec2 z)  { return vec2(cos(z.x) * cosh(z.y), -sin(z.x) * sinh(z.y)); }
vec2 ctan(vec2 z)  { return div(sin(z) , cos(z)); }

vec2 casinh(vec2 z) {
	vec2 t = vec2((z.x - z.y) * (z.x + z.y) + 1, 2 * z.x * z.y);
	return	 log(sqrt(t)+z);
}     

vec2 casin(vec2 z) {
	vec2 t = asinh( vec2(-z.y, z.x) );
	return vec2(t.y, -t.x);
}
vec2 cacos(vec2 z) {
	vec2 t = asin(z);
	return vec2(1.7514 - t.x, -t.y);
}

////////////////////////      

uint argbf2uint(uint alpha, float r, float g, float b) { 
	return (alpha << 24)         |
	(uint(255.*r)  & 0xffu     ) |  
	((uint(255.*g) & 0xffu)<<8 ) | 
	((uint(255.*b) & 0xffu)<<16) ;
}

uint rgbf2uint(vec3 v) { 
	v*=255.;
	return 0xff000000u        | // alpha 0xff
	( uint(v.r) & 0xffu     ) |  
	((uint(v.g) & 0xffu)<<8 ) | 
	((uint(v.b) & 0xffu)<<16) ;
}
		
uint HSV2int(float h, float s, float v) { // convert hsv to int with alpha 0xff00000
	float r = 0, g = 0, b = 0;
	
	if (s == 0) r = g = b = v;
	else {
		if (h == 1)  h = 0;
		
		float z = floor(h * 6.),
			f = h * 6 - z,
			p = v * (1 - s), q = v * (1 - s * f),
			t = v * (1 - s * (1 - f));
		
		return rgbf2uint( vec3[]( vec3(v,t,p), vec3(q,v,p), vec3(p,v,t), 
								  vec3(p,q,v), vec3(t,p,v), vec3(v,p,q) ) [int(z) % 6] );
	}
	return rgbf2uint(vec3(r, g, b));
}

vec2 domain_color_func(vec2); // domain coloring func

uint dc_get_color(int x, int y, int w, int h) {

	const float E = 2.7182818284590452353602874713527,
				M_PI = 3.141592653589793238462643383,
				PI = M_PI, PI2 = PI * 2.;
	
	const float limit=PI,  rmi = -limit, rma = limit, imi = -limit, ima = limit;
	
	vec2 z = vec2( ima - (ima - imi) * y / (h - 1),  rma - (rma - rmi) * x / (w - 1) );
	
	vec2 v = domain_color_func(z); // evaluate domain coloring func
	
	
	float 	m, ranges, rangee; //  prop. e^n < m < e^(n-1)
	for (m=length(v), ranges=0, rangee=1; m > rangee; rangee *= E) ranges = rangee;
	
	float 	k  = (m - ranges) / (rangee - ranges),
		  	kk = (k < 0.5 ? k * 2. : 1. - (k - 0.5) * 2);
	
	float 	ang = mod(abs(arg(v)), PI2) / PI2,    // -> hsv
			sat = 0.4 + (1. - pow(1. - kk, 3.))       * 0.6,
			val = 0.6 + (1. - pow(1. - (1 - kk), 3.)) * 0.4;
	
	return HSV2int(ang, sat, val);
}            

/////////////////////// the domain coloring func

vec2 domain_color_func(vec2 z) { // f(z)
	return z;	
	vec2 z1 = div(cpow(z,4)+1., cpow(z,3)-1.);
	// vec2 z1 = mul(pow(z,4), cos(z)) + pow(z,4);
	z1 +=  mul( z/5 , csin(z) ) ;
	return z1;
}


out uint out_color; // RGBA

uniform ivec2 size;

void main() {
	int x = gl_VertexID % size.x, y = gl_VertexID /  size.x;
	
	out_color = dc_get_color(x, y, size.x, size.y);
}