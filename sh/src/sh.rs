/*
    Spherical Harmonics
*/

use rayon::prelude::*;
use std::f32::consts::PI;
use nalgebra as na;
use na::{Point2, Point3, Vector3};

const SPHERICAL_HARMONICS_CODES: [i32; 647] = [
    1222412, 1410121, 1420121, 1434301, 1441324, 1444401, 1444421, 2222222, 2240322, 2420214,
    2441224, 4026442, 4032241, 4240412, 4310132, 4322142, 4323242, 4410112, 4422122, 4422133,
    4422242, 11111212, 11112242, 11121314, 11121442, 11121443, 11132444, 11134321, 11142241,
    11143234, 11214244, 11223344, 11224224, 11232334, 11242234, 11244141, 11244224, 11244444,
    11311232, 11314442, 11321224, 11321242, 11331442, 11334422, 11344234, 11413142, 11421122,
    11421133, 11421244, 11422233, 11434241, 11441111, 11442211, 12121224, 12123222, 12123244,
    12124232, 12141212, 12221422, 12222212, 12222242, 12223242, 12244424, 12320124, 12321244,
    12322141, 12341234, 12414244, 12420224, 12420244, 12421442, 12422232, 12431424, 12442124,
    13121242, 13134224, 13142244, 13224424, 13243234, 13312222, 13313342, 13324143, 13332424,
    13342114, 13422421, 13422421, 13434243, 13443212, 13443244, 13444124, 14032211, 14122442,
    14126211, 14131214, 14142242, 14222231, 14222414, 14234211, 14234214, 14241424, 14242414,
    14243444, 14322212, 14333242, 14344432, 14414232, 14422143, 14431243, 14432424, 14434241,
    14444122, 14444232, 21022212, 21023122, 21030324, 21142223, 21142424, 21210412, 21212121,
    21213434, 21214422, 21222222, 21222422, 21224212, 21234314, 21332321, 21333444, 21344422,
    21412441, 21413214, 21413434, 21422122, 21422241, 21442221, 22023304, 22024402, 22041224,
    22113231, 22124144, 22133212, 22141344, 22144344, 22212414, 22222244, 22223232, 22224231,
    22224242, 22232442, 22243224, 22243442, 22314442, 22323222, 22323322, 22334334, 22344234,
    22344404, 22411232, 22411432, 22420214, 22424222, 22424224, 22431442, 22432424, 22442212,
    22442344, 22443232, 23112442, 23124422, 23124443, 23134234, 23142213, 23142314, 23143212,
    23214221, 23224442, 23230324, 23232322, 23242441, 23244133, 23312441, 23324424, 23332244,
    23344241, 23412342, 23414421, 23424144, 23432332, 23434423, 23442443, 23444233, 23444312,
    24024442, 24112332, 24124442, 24133441, 24134314, 24144342, 24213423, 24222224, 24222422,
    24222442, 24224422, 24234422, 24241212, 24242142, 24242412, 24243434, 24244224, 24313124,
    24324433, 24330324, 24330324, 24333333, 24341423, 24412424, 24422214, 24422222, 24423423,
    24431212, 24442231, 24444222, 31112444, 31124442, 31132324, 31142224, 31214244, 31221122,
    31234431, 31244224, 31313422, 31323222, 31331234, 31342434, 31344234, 31414234, 31422241,
    31432221, 31434111, 31434321, 31443224, 32111242, 32120214, 32123441, 32132224, 32144244,
    32220144, 32221214, 32224222, 32224244, 32231242, 32243234, 32314222, 32321442, 32343222,
    32412124, 32424232, 32424242, 32432124, 32432222, 32441232, 33141232, 33221322, 33244232,
    33333333, 33412244, 33421234, 33422432, 33423121, 33441233, 34111244, 34124244, 34134243,
    34143141, 34143144, 34210144, 34223221, 34223244, 34224224, 34234324, 34241214, 34243131,
    34243212, 34314242, 34322112, 34334242, 34342414, 34343434, 34414442, 34422142, 34423242,
    34424334, 34431243, 34432241, 34441441, 34442122, 34443234, 34444122, 41112442, 41122442,
    41124122, 41132432, 41142244, 41144141, 41144442, 41212121, 41213244, 41213422, 41224124,
    41224224, 41224334, 41231242, 41242214, 41244432, 41311222, 41313222, 41313442, 41324211,
    41334223, 41341222, 41341222, 41342214, 41344441, 41412121, 41421442, 41422334, 41434144,
    41442434, 42000024, 42024232, 42111412, 42123241, 42131212, 42142244, 42212412, 42221124,
    42221222, 42222232, 42223432, 42232414, 42233223, 42241212, 42313422, 42323244, 42323422,
    42324244, 42333422, 42333442, 42342341, 42344241, 42412444, 42413121, 42421424, 42422424,
    42423232, 42424141, 42424444, 42433124, 42441111, 42441222, 42441232, 42622462, 42624422,
    43114443, 43122224, 43124114, 43131324, 43134144, 43142212, 43144344, 43214321, 43221432,
    43232442, 43244322, 43313443, 43323212, 43323212, 43324224, 43334413, 43342222, 43342432,
    43344334, 43414422, 43421121, 43424242, 43434142, 43434144, 43434442, 43444422, 44004400,
    44112412, 44113231, 44121224, 44134122, 44134324, 44143322, 44213242, 44221144, 44234124,
    44234232, 44243422, 44314123, 44322124, 44334242, 44334343, 44342232, 44342412, 44414224,
    44421242, 44421421, 44421424, 44431421, 44432424, 44441212, 44444242, 12345678, 13287282,
    26345664, 26722884, 27282827, 27382738, 27384856, 34567812, 36178242, 36377284, 36383836,
    36546644, 37483847, 41828446, 42273881, 42428822, 42646246, 45226644, 45434666, 45544256,
    45565254, 45634566, 46266464, 46352226, 46466433, 46514416, 46544346, 46544654, 46545253,
    46611454, 46636546, 46727861, 46848126, 47484748, 47626684, 48422614, 48424841, 51144446,
    51263462, 51325455, 51446454, 51546634, 51563652, 51616151, 51644243, 51644633, 52145236,
    52222553, 52344664, 52465354, 52466446, 52545256, 52564464, 52566465, 52664654, 52824574,
    52828252, 53164266, 53261146, 53364463, 53426426, 53464345, 53536564, 53623456, 53634434,
    53665364, 53816273, 54354662, 54365636, 54424262, 54445464, 54466344, 54546444, 54613546,
    54633426, 54644554, 54647484, 55266536, 55446446, 55546256, 55555555, 55556666, 56266411,
    56344624, 56366644, 56434663, 56645264, 56646264, 57356365, 57386575, 61144246, 61243256,
    61345524, 61366442, 61446256, 61452663, 61465462, 61465642, 61487462, 61515162, 61546264,
    61555464, 61626364, 61644644, 61645245, 62246654, 62446264, 62544564, 62545366, 62546455,
    62624554, 62648628, 62666461, 62726574, 63266454, 63286212, 63364224, 63366254, 63446264,
    62545564, 63626263, 63636266, 64162446, 64252546, 64354462, 64365636, 64415264, 64436544,
    64446264, 64446534, 64534244, 64636261, 64644554, 64668571, 64828241, 65345261, 65432884,
    65436543, 65446264, 65526244, 65533264, 65536266, 65464838, 65784231, 65837244, 66162444,
    66226644, 66245544, 66344661, 66365254, 66444264, 66446264, 66446644, 66526652, 66566424,
    66576658, 66635246, 66644624, 66665656, 66666868, 66872244, 67184718, 67442786, 67822674,
    68166264, 68284821, 68426842, 68626448, 68628448, 71288472, 71528364, 72484846, 72527252,
    72727474, 72737475, 72747678, 72774848, 72816384, 73747526, 73836283, 74737271, 74846484,
    75227641, 75318642, 75717472, 75737274, 76677484, 76737321, 77447722, 77665544, 77784846,
    78167264, 78332364, 78767684, 78787274, 81417181, 81828281, 81828384, 82222534, 82246116,
    82264224, 82624242, 82645731, 82727282, 82747816, 82828484, 82848688, 83325375, 83737383,
    83828482, 83848483, 84622884, 84627181, 84627531, 84644221, 84682866, 84822221, 84838281,
    84841111, 85243642, 85737583, 85847372, 85848182, 85858686, 85868283, 86442221, 86838321,
    87766554, 88228822, 88646261, 88824442, 88888888, 44444444,
];

const PI2: f32 = PI * 2.0;



#[derive(Copy, Clone)]
pub struct Vertex {
    //  used in glium
    pub position    : Point3<f32>,
    pub normal      : Vector3<f32>,
    pub color       : Point3<f32>,
    pub texture     : Point2<f32>,
}

pub struct SpericalHarmonics {
    pub n       : u32,
    pub size    : usize,
    code        : Vec<f32>,
    color_map   : u32,
    pub shape   : Vec<Vertex>, // position, normal, color, texture
    pub indexes : Vec<u32>,
}

impl SpericalHarmonics {
    pub fn new(n: u32, n_code: u32, color_map: u32) -> Self {
        fn code_2_vec(n_code: u32) -> Vec<f32> {
            let mut m = SPHERICAL_HARMONICS_CODES[n_code as usize];
            let mut v: Vec<f32> = vec![0_f32; 8];
            for i in 0..8 {
                v[7 - i] = (m % 10) as f32;
                m /= 10
            }
            v
        }
        let n_code = n_code % SPHERICAL_HARMONICS_CODES.len() as u32;

        let s = Self {
            n,
            size    : (n*n) as usize,
            code 	: code_2_vec(n_code),
            color_map,
            shape	: vec![],
            indexes	: vec![],
        };
        s.generate()
    }

	fn trig_strip4(&mut self) {
        // generate trig strip indexes [0, 1, n+1, n]
        let n = self.n;
        let ix_vect = [0, 1, n, n+1]; // quad to trig-strip order

        self.indexes = (0..(4 * n * n))
            .into_par_iter()
            .map(|index| (ix_vect[(index % 4) as usize] + (index / 4)) as u32)
            .collect();
    }

    fn trig_strip3(&mut self) { // quad -> trig
        let n = self.n;
        let ix_vect = [0, 1, n+1, 0, n+1, n]; // trig order

        self.indexes = (0..6 * self.size)
            .into_par_iter()
            .map(|index| (ix_vect[(index % 6) as usize] + (index / 6) as u32))
            .collect();
    }

    pub fn generate(mut self) -> Self {
        fn calc_vertex(code: &Vec<f32>, theta: f32, phi: f32) -> Point3<f32> {
            let r = (code[0] * phi  ).sin().powf(code[1]) + (code[2] * phi  ).cos().powf(code[3]) +
                    (code[4] * theta).sin().powf(code[5]) + (code[6] * theta).cos().powf(code[7]);

            Point3::new(
                r * phi.sin() * theta.cos(),
                r * phi.cos(),
                r * phi.sin() * theta.sin(),
            )
        }

        fn calc_normal(v0: Point3<f32>, v1: Point3<f32>, v2: Point3<f32>) -> Vector3<f32> {
            ((v2-v0).cross(&(v1-v0))).normalize()
        }

        fn calc_color(v: f32, vmin: f32, vmax: f32, cm: u32) -> Point3<f32> {
            let zv = Point3::new(0., 0., 0.);
            let mut dv: f32;
            let vmid: f32;

            let (mut c, mut c1, mut c2, mut c3) = (Point3::new(1.0, 1.0, 1.0), zv, zv, zv);

            let ratio: f32;
            let mut vmin = vmin;
            let mut vmax = vmax;
            let mut v = v;

            if vmax < vmin {
                dv = vmin;
                vmin = vmax;
                vmax = dv;
            }
            if vmax - vmin < 0.000001 {
                vmin -= 1.;
                vmax += 1.;
            }
            if v < vmin {
                v = vmin;
            }
            if v > vmax {
                v = vmax;
            }
            dv = vmax - vmin;

            match cm {
                0|1 => {
                    if v < (vmin + 0.25 * dv) {
                        c[0] = 0.;
                        c[1] = 4. * (v - vmin) / dv;
                        c[2] = 1.;
                    } else if v < (vmin + 0.5 * dv) {
                        c[0] = 0.;
                        c[1] = 1.;
                        c[2] = 1. + 4. * (vmin + 0.25 * dv - v) / dv;
                    } else if v < (vmin + 0.75 * dv) {
                        c[0] = 4. * (v - vmin - 0.5 * dv) / dv;
                        c[1] = 1.;
                        c[2] = 0.;
                    } else {
                        c[0] = 1.;
                        c[1] = 1. + 4. * (vmin + 0.75 * dv - v) / dv;
                        c[2] = 0.;
                    }
                }

                2 => {
                    c[0] = (v - vmin) / dv;
                    c[1] = 0.;
                    c[2] = (vmax - v) / dv;
                }
                3 => {
                    c[0] = (v - vmin) / dv;
                    c[2] = c[0];
                    c[1] = c[0];
                }
                4 => {
                    if v < (vmin + dv / 6.0) {
                        c[0] = 1.;
                        c[1] = 6. * (v - vmin) / dv;
                        c[2] = 0.;
                    } else if v < (vmin + 2.0 * dv / 6.0) {
                        c[0] = 1. + 6. * (vmin + dv / 6.0 - v) / dv;
                        c[1] = 1.;
                        c[2] = 0.;
                    } else if v < (vmin + 3.0 * dv / 6.0) {
                        c[0] = 0.;
                        c[1] = 1.;
                        c[2] = 6. * (v - vmin - 2.0 * dv / 6.0) / dv;
                    } else if v < (vmin + 4.0 * dv / 6.0) {
                        c[0] = 0.;
                        c[1] = 1. + 6. * (vmin + 3.0 * dv / 6.0 - v) / dv;
                        c[2] = 1.;
                    } else if v < (vmin + 5.0 * dv / 6.0) {
                        c[0] = 6. * (v - vmin - 4.0 * dv / 6.0) / dv;
                        c[1] = 0.;
                        c[2] = 1.;
                    } else {
                        c[0] = 1.;
                        c[1] = 0.;
                        c[2] = 1. + 6. * (vmin + 5.0 * dv / 6.0 - v) / dv;
                    }
                }
                5 => {
                    c[0] = (v - vmin) / (vmax - vmin);
                    c[1] = 1.;
                    c[2] = 0.;
                }
                6 => {
                    c[0] = (v - vmin) / (vmax - vmin);
                    c[1] = (vmax - v) / (vmax - vmin);
                    c[2] = c[0];
                }
                7 => {
                    if v < (vmin + 0.25 * dv) {
                        c[0] = 0.;
                        c[1] = 4. * (v - vmin) / dv;
                        c[2] = 1. - c[1];
                    } else if v < (vmin + 0.5 * dv) {
                        c[0] = 4. * (v - vmin - 0.25 * dv) / dv;
                        c[1] = 1. - c[0];
                        c[2] = 0.;
                    } else if v < (vmin + 0.75 * dv) {
                        c[1] = 4. * (v - vmin - 0.5 * dv) / dv;
                        c[0] = 1. - c[1];
                        c[2] = 0.;
                    } else {
                        c[0] = 0.;
                        c[2] = 4. * (v - vmin - 0.75 * dv) / dv;
                        c[1] = 1. - c[2];
                    }
                }
                8 => {
                    if v < (vmin + 0.5 * dv) {
                        c[0] = 2. * (v - vmin) / dv;
                        c[1] = c[0];
                        c[2] = c[0];
                    } else {
                        c[0] = 1. - 2. * (v - vmin - 0.5 * dv) / dv;
                        c[1] = c[0];
                        c[2] = c[0];
                    }
                }
                9 => {
                    if v < (vmin + dv / 3.) {
                        c[2] = 3. * (v - vmin) / dv;
                        c[1] = 0.;
                        c[0] = 1. - c[2];
                    } else if v < (vmin + 2. * dv / 3.) {
                        c[0] = 0.;
                        c[1] = 3. * (v - vmin - dv / 3.) / dv;
                        c[2] = 1.;
                    } else {
                        c[0] = 3. * (v - vmin - 2. * dv / 3.) / dv;
                        c[1] = 1. - c[0];
                        c[2] = 1.;
                    }
                }
                10 => {
                    if v < (vmin + 0.2 * dv) {
                        c[0] = 0.;
                        c[1] = 5. * (v - vmin) / dv;
                        c[2] = 1.;
                    } else if v < (vmin + 0.4 * dv) {
                        c[0] = 0.;
                        c[1] = 1.;
                        c[2] = 1. + 5. * (vmin + 0.2 * dv - v) / dv;
                    } else if v < (vmin + 0.6 * dv) {
                        c[0] = 5. * (v - vmin - 0.4 * dv) / dv;
                        c[1] = 1.;
                        c[2] = 0.;
                    } else if v < (vmin + 0.8 * dv) {
                        c[0] = 1.;
                        c[1] = 1. - 5. * (v - vmin - 0.6 * dv) / dv;
                        c[2] = 0.;
                    } else {
                        c[0] = 1.;
                        c[1] = 5. * (v - vmin - 0.8 * dv) / dv;
                        c[2] = 5. * (v - vmin - 0.8 * dv) / dv;
                    }
                }
                11 => {
                    c1[0] = 200. / 255.0;
                    c1[1] = 60. / 255.0;
                    c1[2] = 0. / 255.0;
                    c2[0] = 250. / 255.0;
                    c2[1] = 160. / 255.0;
                    c2[2] = 110. / 255.0;
                    c[0] = (c2[0] - c1[0]) * (v - vmin) / dv + c1[0];
                    c[1] = (c2[1] - c1[1]) * (v - vmin) / dv + c1[1];
                    c[2] = (c2[2] - c1[2]) * (v - vmin) / dv + c1[2];
                }
                12 => {
                    c1[0] = 55. / 255.0;
                    c1[1] = 55. / 255.0;
                    c1[2] = 45. / 255.0;
                    /* c2[0] = 200 / 255.0; c2[1] =  60 / 255.0; c2[2] =   0 / 255.0; */
                    c2[0] = 235. / 255.0;
                    c2[1] = 90. / 255.0;
                    c2[2] = 30. / 255.0;
                    c3[0] = 250. / 255.0;
                    c3[1] = 160. / 255.0;
                    c3[2] = 110. / 255.0;
                    ratio = 0.4;
                    vmid = vmin + ratio * dv;
                    if v < vmid {
                        c[0] = (c2[0] - c1[0]) * (v - vmin) / (ratio * dv) + c1[0];
                        c[1] = (c2[1] - c1[1]) * (v - vmin) / (ratio * dv) + c1[1];
                        c[2] = (c2[2] - c1[2]) * (v - vmin) / (ratio * dv) + c1[2];
                    } else {
                        c[0] = (c3[0] - c2[0]) * (v - vmid) / ((1. - ratio) * dv) + c2[0];
                        c[1] = (c3[1] - c2[1]) * (v - vmid) / ((1. - ratio) * dv) + c2[1];
                        c[2] = (c3[2] - c2[2]) * (v - vmid) / ((1. - ratio) * dv) + c2[2];
                    }
                }
                13 => {
                    c1[0] = 0. / 255.0;
                    c1[1] = 255. / 255.0;
                    c1[2] = 0. / 255.0;
                    c2[0] = 255. / 255.0;
                    c2[1] = 150. / 255.0;
                    c2[2] = 0. / 255.0;
                    c3[0] = 255. / 255.0;
                    c3[1] = 250. / 255.0;
                    c3[2] = 240. / 255.0;
                    ratio = 0.3;
                    vmid = vmin + ratio * dv;
                    if v < vmid {
                        c[0] = (c2[0] - c1[0]) * (v - vmin) / (ratio * dv) + c1[0];
                        c[1] = (c2[1] - c1[1]) * (v - vmin) / (ratio * dv) + c1[1];
                        c[2] = (c2[2] - c1[2]) * (v - vmin) / (ratio * dv) + c1[2];
                    } else {
                        c[0] = (c3[0] - c2[0]) * (v - vmid) / ((1. - ratio) * dv) + c2[0];
                        c[1] = (c3[1] - c2[1]) * (v - vmid) / ((1. - ratio) * dv) + c2[1];
                        c[2] = (c3[2] - c2[2]) * (v - vmid) / ((1. - ratio) * dv) + c2[2];
                    }
                }
                14 => {
                    c[0] = 1.;
                    c[1] = 1. - (v - vmin) / dv;
                    c[2] = 0.;
                }
                15 => {
                    if v < (vmin + 0.25 * dv) {
                        c[0] = 0.;
                        c[1] = 4. * (v - vmin) / dv;
                        c[2] = 1.;
                    } else if v < (vmin + 0.5 * dv) {
                        c[0] = 0.;
                        c[1] = 1.;
                        c[2] = 1. - 4. * (v - vmin - 0.25 * dv) / dv;
                    } else if v < (vmin + 0.75 * dv) {
                        c[0] = 4. * (v - vmin - 0.5 * dv) / dv;
                        c[1] = 1.;
                        c[2] = 0.;
                    } else {
                        c[0] = 1.;
                        c[1] = 1.;
                        c[2] = 4. * (v - vmin - 0.75 * dv) / dv;
                    }
                }
                16 => {
                    if v < (vmin + 0.5 * dv) {
                        c[0] = 0.0;
                        c[1] = 2. * (v - vmin) / dv;
                        c[2] = 1. - 2. * (v - vmin) / dv;
                    } else {
                        c[0] = 2. * (v - vmin - 0.5 * dv) / dv;
                        c[1] = 1. - 2. * (v - vmin - 0.5 * dv) / dv;
                        c[2] = 0.0;
                    }
                }
                17 => {
                    if v < (vmin + 0.5 * dv) {
                        c[0] = 1.0;
                        c[1] = 1. - 2. * (v - vmin) / dv;
                        c[2] = 2. * (v - vmin) / dv;
                    } else {
                        c[0] = 1. - 2. * (v - vmin - 0.5 * dv) / dv;
                        c[1] = 2. * (v - vmin - 0.5 * dv) / dv;
                        c[2] = 1.0;
                    }
                }
                18 => {
                    c[0] = 0.;
                    c[1] = (v - vmin) / (vmax - vmin);
                    c[2] = 1.;
                }
                19 => {
                    c[0] = (v - vmin) / (vmax - vmin);
                    c[1] = c[0];
                    c[2] = 1.;
                }
                20 => {
                    c1[0] = 0. / 255.0;
                    c1[1] = 160. / 255.0;
                    c1[2] = 0. / 255.0;
                    c2[0] = 180. / 255.0;
                    c2[1] = 220. / 255.0;
                    c2[2] = 0. / 255.0;
                    c3[0] = 250. / 255.0;
                    c3[1] = 220. / 255.0;
                    c3[2] = 170. / 255.0;
                    ratio = 0.3;
                    vmid = vmin + ratio * dv;
                    if v < vmid {
                        c[0] = (c2[0] - c1[0]) * (v - vmin) / (ratio * dv) + c1[0];
                        c[1] = (c2[1] - c1[1]) * (v - vmin) / (ratio * dv) + c1[1];
                        c[2] = (c2[2] - c1[2]) * (v - vmin) / (ratio * dv) + c1[2];
                    } else {
                        c[0] = (c3[0] - c2[0]) * (v - vmid) / ((1. - ratio) * dv) + c2[0];
                        c[1] = (c3[1] - c2[1]) * (v - vmid) / ((1. - ratio) * dv) + c2[1];
                        c[2] = (c3[2] - c2[2]) * (v - vmid) / ((1. - ratio) * dv) + c2[2];
                    }
                }
                21 => {
                    c1[0] = 255. / 255.0;
                    c1[1] = 255. / 255.0;
                    c1[2] = 200. / 255.0;
                    c2[0] = 150. / 255.0;
                    c2[1] = 150. / 255.0;
                    c2[2] = 255. / 255.0;
                    c[0] = (c2[0] - c1[0]) * (v - vmin) / dv + c1[0];
                    c[1] = (c2[1] - c1[1]) * (v - vmin) / dv + c1[1];
                    c[2] = (c2[2] - c1[2]) * (v - vmin) / dv + c1[2];
                }
                22 => {
                    c[0] = 1. - (v - vmin) / dv;
                    c[1] = 1. - (v - vmin) / dv;
                    c[2] = (v - vmin) / dv;
                }
                23 => {
                    if v < (vmin + 0.5 * dv) {
                        c[0] = 1.;
                        c[1] = 2. * (v - vmin) / dv;
                        c[2] = c[1];
                    } else {
                        c[0] = 1. - 2. * (v - vmin - 0.5 * dv) / dv;
                        c[1] = c[0];
                        c[2] = 1.;
                    }
                }
                24 => {
                    if v < (vmin + 0.5 * dv) {
                        c[0] = 2. * (v - vmin) / dv;
                        c[1] = c[0];
                        c[2] = 1. - c[0];
                    } else {
                        c[0] = 1.;
                        c[1] = 1. - 2. * (v - vmin - 0.5 * dv) / dv;
                        c[2] = 0.;
                    }
                }
                25 => {
                    if v < (vmin + dv / 3.) {
                        c[0] = 0.;
                        c[1] = 3. * (v - vmin) / dv;
                        c[2] = 1.;
                    } else if v < (vmin + 2. * dv / 3.) {
                        c[0] = 3. * (v - vmin - dv / 3.) / dv;
                        c[1] = 1. - c[0];
                        c[2] = 1.;
                    } else {
                        c[0] = 1.;
                        c[1] = 0.;
                        c[2] = 1. - 3. * (v - vmin - 2. * dv / 3.) / dv;
                    }
                }
                _ => {}
            }

            c
        }

        let n = self.n as f32;
        let du = PI2 / n; // Theta
        let dv = PI / n; // Phi
        let code = &self.code;

        let dx = 1. / n;
        self.shape = (0..self.size)
            .into_par_iter()
            .map(|index| {
                let (i, j) = (index / self.n as usize, index % self.n as usize);

                let (u, v) = (du * i as f32, dv * j as f32);
                let color_offset = if (i & 1) == 0 { u } else { u + du };

                let position = calc_vertex(code, u, v);
                let normal = calc_normal(
                    position,
                    calc_vertex(code, u + du, v),
                    calc_vertex(code, u, v + dv),
                );
                let color = calc_color(color_offset, 0., PI2, self.color_map);
                let texture = Point2::new(i as f32 * dx, j as f32 * dx);

                Vertex { position, normal, color,  texture }
            })
            .collect();

        self.trig_strip3();
        self
    }

    /*
    fn triangularize( n_sides : u32, offset : u32 ) -> Vec<u16> { // generate n_sides polygon set of trig index coords
        let mut res = vec![0_u16; (n_sides as usize - 2) * 3];
        for i in 1..n_sides - 1 { // 0, i, i+1 : i=1..ns-1, for quad=4: 0 1 2 0 2 3
          let vi = [0, i, i + 1];
          for j in 0..3 {
            res[((i - 1) * 3 + j) as usize] = vi[j as usize] as u16 + offset as u16
          }
        }
        res
    }
    */
}
