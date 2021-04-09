// flag.rs

use crate::polyhedron::Polyhedron;
use rayon::prelude::*;

#[derive(PartialEq, PartialOrd, Clone)]
pub struct Int4int {
    // face map
    pub _i4: Int4,
    pub i: u32,
}

// Int4 type
pub type Int4 = [u32; 4];
pub fn i4_min(v1: u32, v2: u32) -> Int4 {
    if v1 < v2 {
        to_int4_2(v1, v2)
    } else {
        to_int4_2(v2, v1)
    }
}
pub fn i4_min_3(i: u32, v1: u32, v2: u32) -> Int4 {
    if v1 < v2 {
        to_int4_3(i, v1, v2)
    } else {
        to_int4_3(i, v2, v1)
    }
}
pub fn to_int4(v1: u32) -> Int4 {
    [v1 + 1, 0, 0, 0]
}
pub fn to_int4_2(v1: u32, v2: u32) -> Int4 {
    [v1 + 1, v2 + 1, 0, 0]
}
pub fn to_int4_3(v1: u32, v2: u32, v3: u32) -> Int4 {
    [v1 + 1, v2 + 1, v3 + 1, 0]
}
pub fn to_int4_4(v1: u32, v2: u32, v3: u32, v4: u32) -> Int4 {
    [v1 + 1, v2 + 1, v3 + 1, v4 + 1]
}

type Vertex = Vec<f32>;
type MapIndex = [Int4; 3];

#[derive(PartialEq, Clone)]
struct VertexIndex {
    index: u32,
    vertex: Vertex,
}

#[derive(PartialEq, Clone)]
pub struct I4Vix {
    index: Int4,
    vix: VertexIndex,
}

#[derive(Clone)]
pub struct Flag {
    pub index: usize,
    pub vertexes: Vec<Vertex>,
    pub faces: Vec<Vec<u32>>,

    pub v: Vec<I4Vix>,
    pub m: Vec<MapIndex>, // m[i4][i4]=i4 -> m[]<<i4,i4,i4
    pub fcs: Vec<Vec<Int4>>,

    pub v_index: u32,
}

impl Flag {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            vertexes: vec![],
            faces: vec![],
            v: vec![],
            m: vec![],
            fcs: vec![],
            v_index: 0,
        }
    }
    pub fn with_vertexes(self, vertexes: &Vec<Vec<f32>>) -> Self {
        let mut flag = Flag::new(0);
        flag.set_vertexes(vertexes);
        flag
    }

    pub fn with_offset(mut self, offset: u32) -> Self {
        self.v_index = offset;
        self
    }

    pub fn add_vertex(&mut self, v: &Vertex) -> u32 {
        self.vertexes.push(v.clone());
        self.vertexes.len() as u32 - 1
    }

    pub fn set_vertexes(&mut self, vertexes: &Vec<Vertex>) {
        for (i, v) in vertexes.iter().enumerate() {
            self.v.push(I4Vix {
                index: to_int4(i as u32),
                vix: VertexIndex {
                    index: i as u32,
                    vertex: v.clone(),
                },
            });
        }
        self.v_index = self.v.len() as u32
    }

    pub fn set_vertex(&mut self, i: u32, ix: &Int4, vtx: &Vertex) {
        self.v[i as usize] = I4Vix {
            index: *ix,
            vix: VertexIndex {
                index: 0,
                vertex: vtx.clone(),
            },
        }
    }

    pub fn add_v(&mut self, ix: &Int4, vtx: &Vertex) {
        // v << {ix,{v_index++, vtx}}
        self.v.push(I4Vix {
            index: *ix,
            vix: VertexIndex {
                index: self.v_index,
                vertex: vtx.clone(),
            },
        });
        self.v_index += 1
    }

    pub fn add_face_m(&mut self, i0: &Int4, i1: &Int4, i2: &Int4) {
        self.m.push([*i0, *i1, *i2]);
    }

    pub fn add_face_f(&mut self, vf: &Vec<Int4>) {
        self.fcs.push(vf.clone())
    }
    pub fn add_faces_f(&mut self, vf: &Vec<Vec<Int4>>) {
        for v in vf {
            self.fcs.push(v.clone())
        }
    }

    // pub fn find_vertex(&self, v: &Int4) -> &I4Vix {
    //     self.v.iter().find(|_v| _v.index == *v).unwrap()
    // }

    // should be sorted -> binary search for (v,m)
    pub fn find_vertex_index(&self, v: &Int4) -> u32 {
        let ix = match self.v.binary_search_by(|_v| _v.index.cmp(v)) {
            Ok(ix) | Err(ix) => ix,
        };
        self.v[ix].vix.index
    }

    pub fn find_m(&self, m0: &Int4, m1: &Int4) -> Int4 {
        let mi = &&[*m0, *m1, [0, 0, 0, 0]];
        let ix = match self.m.binary_search_by(|_v| _v.cmp(mi)) {
            Ok(ix) => ix,
            Err(ix) => ix,
        };
        self.m[ix][2]
    }

    pub fn from_to_m(&self) -> Vec<usize> {
        // gen. vector of from index of face change in m
        let mut v_ft = vec![];

        let mut c0 = self.m[0][0];
        let mut from = 0;

        for i in 0..self.m.len() {
            if self.m[i][0] != c0 {
                v_ft.push(from);
                from = i;
                c0 = self.m[i][0]
            }
        }
        v_ft.push(from);

        v_ft
    }

    pub fn sort_unique_v(&mut self) {
        self.v.par_sort_by(|a, b| a.index.cmp(&b.index));
        self.v.dedup_by(|a, b| a.index == b.index);
    }

    pub fn index_vertexes(&mut self) {
        self.sort_unique_v();
        // vertexes = v.vix.vertex, in index order
        self.vertexes = self
            .v
            .par_iter()
            .map(|v| v.vix.vertex.clone())
            .collect::<Vec<Vec<f32>>>();

        // v.vix.index=0..v.len()
        for i in 0..self.v.len() {
            self.v[i].vix.index = i as u32
        }
    }

    pub fn add_offset(&mut self, offset: u32) {
        if offset != 0 {
            // traverse faces and add 'offset' to each item
            self.faces.par_iter_mut().for_each(|face| {
                for f in face {
                    *f += offset
                }
            });
        }
    }

    pub fn process_m(&mut self) {
        // self.m->self.faces
        if !self.m.is_empty() {
            self.m.par_sort();

            self.faces = self
                .m
                .par_iter()
                .map(|m| {
                    let (_v0, mut _v, _m0) = (m[2], m[2], m[0]);

                    let mut face = vec![];
                    let mut cnt = 0;
                    loop {
                        face.push(self.find_vertex_index(&_v));
                        _v = self.find_m(&_m0, &_v);
                        if _v == _v0 {
                            break;
                        }
                        cnt += 1;
                        if cnt > 30 {
                            println!("m={:?}\n_v={:?}\n_v0={:?}\nface:{:?}\n", m, _v, _v0, face);
                            face.resize(3, 0);
                            break;
                        }
                    }
                    face
                })
                .collect();
        }
        // faces <<fcs
        self.faces.extend(
            self.fcs
                .par_iter()
                .map(|face| {
                    face.iter()
                        .map(|_f| self.find_vertex_index(_f))
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>(),
        );
    }

    pub fn to_poly(&mut self) {
        self.index_vertexes();
        self.faces.clear();
        self.process_m();
    }

    pub fn gen_face_map(poly: &Polyhedron) -> Vec<Int4int> {
        // make table of face as fn of edge

        let mut face_map = vec![];
        for (nface, face) in poly.faces.iter().enumerate() {
            let mut v1 = face.last().unwrap(); // previous vertex index
            for v2 in face {
                face_map.push(Int4int {
                    _i4: to_int4_2(*v1, *v2),
                    i: nface as u32,
                });
                v1 = v2; // current becomes previous
            }
        }
        face_map.par_sort_by(|a, b| a._i4.cmp(&b._i4));
        face_map
    }
}
