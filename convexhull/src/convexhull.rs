#![allow(dead_code)]
use std::fs::File;
use std::io::BufWriter;
use std::io::*;

// basic 'C' waterman & convexhull interface
#[link(name = "convexhull")] // rustc -L. -lstdc++ rust_test.rs
extern "C" {
    pub fn convex_hull(
        n_vertices: usize,
        vertices: *const f64,
        n_faces: *mut usize,
        hn_vertices: *mut usize,
        o_faces: *mut *mut i32,
        o_vertices: *mut *mut f64,
    );
    pub fn free_ch(o_faces: *mut i32, o_vertices: *mut f64);
}

#[derive(Debug)]
pub struct ConvexHull {
    pub faces: Vec<Vec<i32>>,
    pub vertices: Vec<[f64; 3]>,
}

impl ConvexHull {
    pub fn new(vc: Vec<f64>) -> Self {
        fn fl2fv(face_list: &Vec<i32>) -> Vec<Vec<i32>> {
            let mut faces = vec![];
            let mut i = 0;
            loop {
                faces.push(face_list[i + 1..i + face_list[i] as usize + 1].to_vec());
                i += face_list[i] as usize + 1;
                if i >= face_list.len() {
                    break;
                }
            }
            faces
        }
        fn vl2v3(v: &Vec<f64>) -> Vec<[f64; 3]> {
            v.chunks(3).map(|v| [v[0], v[1], v[2]]).collect()
        }
        let mut n_faces: usize = 0;
        let mut n_vertices: usize = 0;
        let mut _faces: *mut i32 = std::ptr::null_mut();
        let mut _vertices: *mut f64 = std::ptr::null_mut();

        unsafe {
            convex_hull(
                vc.len(),
                vc.as_ptr(),
                &mut n_faces,
                &mut n_vertices,
                &mut _faces,
                &mut _vertices,
            );
        }
        let faces = unsafe { std::slice::from_raw_parts(_faces, n_faces) }.to_vec();
        let vertices = unsafe { std::slice::from_raw_parts(_vertices, n_vertices) }.to_vec();
        unsafe { free_ch(_faces, _vertices) } // now it can be released

        let ch = Self {
            faces: fl2fv(&faces),
            vertices: vl2v3(&vertices),
        };
        assert!(ch.check());
        ch
    }
    pub fn check(&self) -> bool {
        let mut seq = vec![0_usize; self.vertices.len()];
        let mut ok = true;
        let mut max_ix = self.faces[0][0]; // max coord in faces

        'a: for f in &self.faces {
            max_ix = *f.iter().max().expect("").max(&max_ix);
            for ix in f {
                let ix = *ix as usize;
                if ix >= self.vertices.len() {
                    ok = false;
                    break 'a;
                } else {
                    seq[ix] = ix;
                }
            }
        }
        ok && max_ix as usize == self.vertices.len() - 1
            && (0..self.vertices.len()).collect::<Vec<_>>() == seq
    }

    pub fn write_obj(&self, path: &str) {
        let mut buff_write = BufWriter::new(File::create(path).unwrap());

        for v in &self.vertices {
            // vertex
            buff_write
                .write(&format!("v {} {} {}\n", v[0], v[1], v[2]).as_bytes())
                .unwrap();
        }
        for face in &self.faces {
            // faces
            buff_write.write(&format!("f ").as_bytes()).unwrap();
            for ix in face {
                buff_write
                    .write(&format!("{} ", ix + 1).as_bytes())
                    .unwrap();
            }
            buff_write.write(&format!("\n").as_bytes()).unwrap();
        }
        buff_write.flush().unwrap();
    }
}
