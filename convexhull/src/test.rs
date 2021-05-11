// test

use crate::convexhull::*;
use crate::waterman;

#[test]
fn test_convex_hull() {
    fn fl2fv(face_list: Vec<i32>) -> Vec<Vec<i32>> {
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
    fn vl2v3(v: Vec<f64>) -> Vec<[f64; 3]> {
        v.chunks(3).map(|v| [v[0], v[1], v[2]]).collect()
    }
    let wp = waterman::gen_waterman_poly(23.);

    let wp: Vec<_> = wp.iter().flatten().collect();
    let mut n_faces: usize = 0;
    let mut n_vertices: usize = 0;
    let mut _faces: *mut i32 = std::ptr::null_mut();
    let mut _vertices: *mut f64 = std::ptr::null_mut();

    unsafe {
        convex_hull(
            wp.len(),
            *wp.as_ptr(),
            &mut n_faces,
            &mut n_vertices,
            &mut _faces,
            &mut _vertices,
        );
    }
    let vfaces = unsafe { std::slice::from_raw_parts(_faces, n_faces) }.to_vec();
    let vvertices = unsafe { std::slice::from_raw_parts(_vertices, n_vertices) }.to_vec();
    unsafe { free_ch(_faces, _vertices) } // now it can be released
                                          // (fl2fv(vfaces.clone()), vl2v3(vvertices))
}

#[test]
fn test_convex_hull_case() {
    let v = waterman::gen_waterman_poly(6.);
    let v = v.iter().flatten().cloned().collect::<Vec<f64>>();
    let ch = ConvexHull::new(v);
    println!("{:.1?}\n\nok={}", ch, ch.check())
}
