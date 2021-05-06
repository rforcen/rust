// examples.rs
/*
     * This image shows 8,000 ellipses. For each k=1,2,3,...,8000 the foci of the k-th ellipse are:
     * A(k)+iB(k)+C(k)e^(300πik/8000) and A(k)+iB(k)-C(k)e^(300πik/8000)
     * and the eccentricity of the k-th ellipse is
     * D(k), where
     * A(k)=(3/4)sin(2πk/8000)cos(6πk/8000)+(1/4)sin(28πk/8000),
     * B(k)=(3/4)cos(2πk/8000)cos(8πk/8000)+(1/4)cos(28πk/8000),
     * C(k)=(1/18)+(1/20)cos(24πk/8000),
     * D(k)=(49/50)-(1/7)(sin(10πk/8000))^4
     */

/*
 Hamid Naderi Yeganeh, "1,000 Line Segments (1)" (August 2014)
 This image shows 1,000 line segments. For each i=1,2,3,...,1000 the endpoints of the i-th line segment are: (-sin(2*pi*i/1000), -cos(2*pi*i/1000)) and ((-1/2)sin(8*pi*i/1000), (-1/2)cos(12*pi*i/1000))

 - See more at: http://www.ams.org/mathimagery/displayimage.php?album=40&pid=565#top_display_media

 http://www.math.wustl.edu/News2015/News2015_Feb_Yeganeh.html

 http://www.huffingtonpost.com/hamid-naderi-yeganeh/using-mathematical-formul_b_9313484.html


 Circles:

 This image shows 4,000 circles. For each k=1,2,3,...,4000 the center of the k-th circle is:

 ((2/3)(sin(6πk/4000))^3+(1/3)sin(26πk/4000), (2/3)(cos(6πk/4000))^3+(1/3)cos(26πk/4000))

 and the radius of the k-th circle is:

 (1/10)(sin(34πk/4000))^2+(1/12)(cos(136πk/4000))^4.


 Ellipses
 This image shows 8,000 ellipses. For each k=1,2,3,...,8000 the foci of the k-th ellipse are:

 A(k)+iB(k)+C(k)e^(300πik/8000)

 and

 A(k)+iB(k)-C(k)e^(300πik/8000)

 and the eccentricity of the k-th ellipse is D(k), where

 A(k)=(sin(14πk/8000))^3,

 B(k)=sin(14πk/8000)sin(10πk/8000),

 C(k)=(1/200)+(1/20)+(1/20)cos(22πk/8000),

 D(K)=(199/200)-(1/7)(sin(22πk/8000))^8.



 This image shows 5,000 line segments. For each k=1,2,3,...,5000 the endpoints of the k-th line segment are:

 (sin(56πk/5000)cos(6πk/5000), cos(54πk/5000)sin(6πk/5000))

 and

 (sin(52πk/5000)cos(6πk/5000), cos(50πk/5000)sin(6πk/5000)).



 http://www.huffingtonpost.com/hamid-naderi-yeganeh/these-beautiful-images-ar_1_b_8705738.html
 */

 
#![allow(dead_code)]

// lines
pub const LINES_1: &str =
    "lines,1..1000, (−sin(2πi/1000),−cos(2πi/1000)), ((−1/2)sin(8πi/1000),(−1/2)cos(12πi/1000))";
pub const LINES_2: &str =
    "lines,1..1000, (−sin(4πi/1000),−cos(2πi/1000)) , ((−1/2)sin(8πi/1000),(−1/2)cos(4πi/1000))";
pub const LINES_3: &str =
    "lines,1..1000, (−sin(8πi/1000),−cos(2πi/1000)) , ((−1/2)sin(6πi/1000),(−1/2)cos(2πi/1000))";
pub const LINES_4: &str = "lines,1..601, (sin(10π(i+699)/2000),cos(8π(i+699)/2000)), 
(sin(12π(i+699)/2000),cos(10π(i+699)/2000))";
pub const LINES_BIRD: &str =
    "lines,1..2000, (3(sin(2πi/2000)^3),−cos(8πi/2000)) , ((3/2)(sin(2πi/2000)^3),(−1/2)cos(6πi/2000))";
// circles
pub const TEST_EXP: &str = "circles,1..4000, 
X(k)=(6/5)((cos(141πk/40000))⁹) (1−(1/2)(sin(πk/40000))³),
Y(k)=k*k,
R(k)=k*k";
pub const OLIVE_BRANCH : &str="circles,
1..4000,
X(k)=(2k/4000)+(1/28)sin(42πk/4000)+(1/9)(sin(21πk/4000))⁸+(1/4)(sin(21πk/4000))⁶sin((2π/5)(k/4000)¹²),
Y(k)=(1/4)(k/4000)²+(1/4)((sin(21πk/4000))⁵+(1/28)sin(42πk/4000))cos((π/2)(k/4000)¹²),
R(k)=(1/170)+(1/67)(sin(42πk/4000))²(1-(cos(21πk/4000))⁴)";
pub const BUTTERFLY1: &str = "circles,
1..40000,
X(k)=(6/5)((cos(141πk/40000))⁹)(1−(1/2)(sin(πk/40000))³)∗
    (1−(1/4)((cos(2πk/40000))³⁰)(1+(2/3)(cos(30πk/40000))²⁰)−
    ((sin(2πk/40000))¹⁰)((sin(6πk/40000))¹⁰)∗((1/5)+(4/5)(cos(24πk/40000))²⁰)),
Y(k)=cos(2πk/40000)((cos(141πk/40000))²)(1+(1/4)((cos(πk/40000))²⁴)∗((cos(3πk/40000))²⁴)
    (cos(19πk/40000))²⁴),
R(k)=(1/100)+(1/40)(((cos(2820πk/40000))⁶)+(sin(141πk/40000))²)(1−((cos(πk/40000))¹⁶)∗
    ((cos(3πk/40000))¹⁶)(cos(12πk/40000))¹⁶)
";
pub const BUTTERFLY3:&str="circles,
1..40000,
X(k)=(3/2)((cos(141πk/40000))⁹)∗(1−(1/2)sin(πk/40000))∗(1−(1/4)((cos(2πk/40000))³⁰)∗(1+(cos(32πk/40000))²⁰))∗(1−(1/2)((sin(2πk/40000))³⁰)
    ∗((sin(6πk/40000))¹⁰)∗((1/2)+(1/2)(sin(18πk/40000))²⁰)),
Y(k)=cos(2πk/40000)∗((cos(141πk/40000))²)∗(1+(1/4)((cos(πk/40000))²⁴)∗((cos(3πk/40000))²⁴)∗(cos(21πk/40000))²⁴),
R(k)=(1/100)+(1/40)(((cos(141πk/40000))¹⁴)+(sin(141πk/40000))⁶)∗(1−((cos(πk/40000))¹⁶)((cos(3πk/40000))¹⁶)∗(cos(12πk/40000))^16)";

pub const CIRCLES_10K: &str = "circles,
1..10000,
X(k)=cos(38πi/10000)^3,
Y(k)=sin(10πi/10000),
R(k)=(1/3)sin(16πi/10000)^2";
// (x−A(k))2+(y−B(k))2=(R(k))2
pub const BIRD_CIRC:&str="circles,
−10000..10000, 
A(k)=(3k/20000)+sin((π/2)(k/10000)^7)((cos(41πk/10000))^6)+(1/4)((cos(41πk/10000))^16)((cos(πk/20000))^12)sin(6πk/10000),
B(k)=−cos((π/2)(k/10000)^7)∗(1+(3/2)(cos(πk/20000)cos(3πk/20000))^6)∗((cos(41πk/10000))^6)+(1/2)(cos(3πk/100000)cos(9πk/100000)cos(18πk/100000))^10,
R(k)=(1/50)+(1/10)((sin(41πk/10000)sin(9πk/100000))^2)+(1/20)((cos(41πk/10000))^2)((cos(πk/20000))^10)";


//  the foci of the k-th ellipse are, and the eccentricity of the k-th ellipse is D(k), where
pub const ELLIPSE_1:&str="ellipse,
1..2500,
(A(k)+iB(k)+C(k)e^68πik/2500, A(k)+iB(k)−C(k)e^68πik/2500),
A(k)=(−3/2)((sin(2πk/2500))3)+(3/10)((sin(2πk/2500))^7),
B(k)=sin((2πk/1875)+(π/6))+(1/4)(sin((2πk/1875)+(π/6)))^3,
C(k)=(2/15)−(1/8)cos(πk/625),
D(k)=(49/50)−(1/7)(sin(4πk/2500))^4";