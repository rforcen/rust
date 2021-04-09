// transformations.rs

use crate::flag::{i4_min, i4_min_3, to_int4, to_int4_2, to_int4_3, to_int4_4, Flag};
use crate::polyhedron::Polyhedron;
use crate::vertex::{add, midpoint, mulc, neg, one_third, sub, tween, unit};

pub fn kis_n(poly: &Polyhedron, n: u32, apexdist: f32) -> Polyhedron {
    // 0, 0.1
    let mut flag = Flag::new(0);

    let normals = poly.calc_normals();
    let centers = poly.calc_centers();

    for (nface, face) in poly.faces.iter().enumerate() {
        let fname = to_int4_2('k' as u32, nface as u32);

        let mut v1 = face.last().unwrap();

        for v2 in face {
            let iv2 = to_int4(*v2);

            flag.add_v(&iv2, &poly.vertexes[*v2 as usize]);

            if face.len() == 0 || n == 0 {
                flag.add_v(
                    &fname,
                    &add(
                        &centers[nface as usize],
                        &mulc(&normals[nface as usize], apexdist),
                    ),
                );
                flag.add_face_f(&vec![to_int4(*v1), iv2, fname])
            } else {
                flag.add_face_m(&to_int4(nface as u32), &to_int4(*v1), &to_int4(*v2))
            }
            v1 = v2
        }
    }

    flag.to_poly();

    Polyhedron {
        name: format!(
            "k{}{}",
            if n == 0 {
                "".to_string()
            } else {
                n.to_string()
            },
            poly.name
        ),
        vertexes: flag.vertexes,
        faces: flag.faces,
    }
}

pub fn ambo(poly: &Polyhedron) -> Polyhedron {
    let mut flag = Flag::new(0);
    let fdwn_name = 'd' as u32;

    for face in &poly.faces {
        let (mut v1, mut v2) = (face[face.len() - 2], face[face.len() - 1]);

        let mut f_orig = vec![];

        for v3 in face {
            let (m12, m23) = (i4_min(v1, v2), i4_min(v2, *v3));

            if v1 < v2 {
                // vertices are the midpoints of all edges of original poly
                flag.add_v(
                    &m12,
                    &midpoint(&poly.vertexes[v1 as usize], &poly.vertexes[v2 as usize]),
                )
            }

            // two new flags: One whose face corresponds to the original f:
            f_orig.push(m12);

            // Another flag whose face  corresponds to (the truncated) v2:
            flag.add_face_m(&to_int4_2(fdwn_name, v2), &m23, &m12);

            // shift over one
            v1 = v2;
            v2 = *v3;
        }
        flag.fcs.push(f_orig)
    }

    flag.to_poly();

    Polyhedron {
        name: format!("a{}", poly.name),
        vertexes: flag.vertexes,
        faces: flag.faces,
    }
}

pub fn gyro(poly: &Polyhedron) -> Polyhedron {
    const FACE_ID: u32 = 'c' as u32;
    let centers = poly.calc_centers(); // new vertices in center of each face

    let mut flag = Flag::new(0).with_vertexes(&poly.vertexes);

    for (nface, face) in poly.faces.iter().enumerate() {
        let (mut v1, mut v2) = (face[face.len() - 2], face[face.len() - 1]);

        flag.add_v(&to_int4_2(FACE_ID, nface as u32), &centers[nface]);

        for v3 in face {
            flag.add_v(
                &to_int4_2(v1, v2),
                &one_third(&poly.vertexes[v1 as usize], &poly.vertexes[v2 as usize]),
            ); // new v in face

            // 5 new faces to fcs
            flag.add_face_f(&vec![
                to_int4_2(FACE_ID, nface as u32),
                to_int4_2(v1, v2),
                to_int4_2(v2, v1),
                to_int4(v2),
                to_int4_2(v2, *v3),
            ]);

            // shift over one
            v1 = v2;
            v2 = *v3;
        }
    }

    flag.to_poly();

    Polyhedron {
        name: format!("g{}", poly.name),
        vertexes: flag.vertexes,
        faces: flag.faces,
    }
}

pub fn propellor(poly: &Polyhedron) -> Polyhedron {
    let mut flag = Flag::new(0).with_vertexes(&poly.vertexes);

    for (nface, face) in poly.faces.iter().enumerate() {
        let (mut v1, mut v2) = (face[face.len() - 2], face[face.len() - 1]);

        for v3 in face {
            flag.add_v(
                &to_int4_2(v1, v2),
                &one_third(&poly.vertexes[v1 as usize], &poly.vertexes[v2 as usize]),
            ); // new v in face, 1/3rd along edge

            flag.add_face_m(
                &to_int4(nface as u32),
                &to_int4_2(v1, v2),
                &to_int4_2(v2, *v3),
            ); // five new flags
            flag.add_face_f(&vec![
                to_int4_2(v1, v2),
                to_int4_2(v2, v1),
                to_int4(v2),
                to_int4_2(v2, *v3),
            ]);

            // shift over one
            v1 = v2;
            v2 = *v3;
        }
    }

    flag.to_poly();

    Polyhedron {
        name: format!("p{}", poly.name),
        vertexes: flag.vertexes,
        faces: flag.faces,
    }
}

pub fn reflect(poly: &Polyhedron) -> Polyhedron {
    let mut poly = poly.clone();

    // reflect each point through origin v=-v
    poly.vertexes.iter_mut().for_each(|v| *v = neg(&v));

    // repair clockwise-ness of faces
    poly.faces.iter_mut().for_each(|face| face.reverse());
    poly.name = format!("p{}", poly.name);
    poly
}

pub fn dual(poly: &Polyhedron) -> Polyhedron {
    let face_map = Flag::gen_face_map(&poly);
    let centers = poly.calc_centers();

    let mut flag = Flag::new(0);

    for (nface, face) in poly.faces.iter().enumerate() {
        let mut v1 = face.last().unwrap(); // previous vertex
        flag.add_v(&to_int4(nface as u32), &centers[nface]);

        for v2 in face {
            flag.add_face_m(
                &to_int4(*v1),
                &to_int4(
                    face_map[face_map
                        .binary_search_by(|_v| _v._i4.cmp(&to_int4_2(*v2, *v1)))
                        .unwrap()]
                    .i,
                ),
                &to_int4(nface as u32),
            );
            v1 = v2; // current becomes previous
        }
    }
    flag.to_poly();
    Polyhedron {
        name: format!("d{}", poly.name),
        vertexes: flag.vertexes,
        faces: flag.faces,
    }
}

pub fn chamfer(poly: &Polyhedron, dist: f32) -> Polyhedron // = 0.05
{
    let normals = poly.calc_normals();
    let mut flag = Flag::new(0);

    for (nface, face) in poly.faces.iter().enumerate() {
        let mut v1 = *face.last().unwrap();
        let mut v1new = to_int4_2(nface as u32, v1);

        for v2 in face {
            // TODO: figure out what distances will give us a planar hex face.
            // Move each old vertex further from the origin.
            flag.add_v(
                &to_int4(*v2),
                &mulc(&poly.vertexes[*v2 as usize], 1. + dist),
            );
            // Add a new vertex, moved parallel to normal.
            let v2new = to_int4_2(nface as u32, *v2);

            flag.add_v(
                &v2new,
                &add(
                    &poly.vertexes[*v2 as usize],
                    &mulc(&normals[nface], dist * 1.5),
                ),
            );

            // Four new flags:
            // One whose face corresponds to the original face:
            flag.add_face_m(&to_int4_2('o' as u32, nface as u32), &v1new, &v2new);

            // And three for the edges of the new hexagon:
            let facename = if v1 < *v2 {
                to_int4_3('h' as u32, v1, *v2)
            } else {
                to_int4_3('h' as u32, *v2, v1)
            };
            flag.add_face_m(&facename, &to_int4(*v2), &v2new);
            flag.add_face_m(&facename, &v2new, &v1new);
            flag.add_face_m(&facename, &v1new, &to_int4(v1));

            v1 = *v2;
            v1new = v2new;
        }
    }

    flag.to_poly();
    Polyhedron {
        name: format!("c{}", poly.name),
        vertexes: flag.vertexes,
        faces: flag.faces,
    }
}

pub fn whirl(poly: &Polyhedron) -> Polyhedron {
    // new vertices around center of each face
    let centers = poly.calc_centers();
    let mut flag = Flag::new(0);

    for (nface, face) in poly.faces.iter().enumerate() {
        let (mut v1, mut v2) = (face[face.len() - 2], face[face.len() - 1]);

        for v3 in face {
            // New vertex along edge
            let v12 = one_third(&poly.vertexes[v1 as usize], &poly.vertexes[v2 as usize]);
            flag.add_v(&to_int4_2(v1, v2), &v12);

            // New vertices near center of face
            let cv1name = to_int4_3('n' as u32, nface as u32, v1);
            let cv2name = to_int4_3('n' as u32, nface as u32, v2);

            flag.add_v(&cv1name, &unit(&one_third(&centers[nface], &v12)));

            // New hexagon for each original edge
            flag.add_face_f(&vec![
                cv1name,
                to_int4_2(v1, v2),
                to_int4_2(v2, v1),
                to_int4(v2),
                to_int4_2(v2, *v3),
                cv2name,
            ]);

            // New face in center of each old face
            flag.add_face_m(&to_int4_2('e' as u32, nface as u32), &cv1name, &cv2name);

            v1 = v2; // shift over one
            v2 = *v3;
        }
    }

    flag.to_poly();
    Polyhedron {
        name: format!("w{}", poly.name),
        vertexes: flag.vertexes,
        faces: flag.faces,
    }
}

pub fn quinto(poly: &Polyhedron) -> Polyhedron {
    let centers = poly.calc_centers();
    let mut flag = Flag::new(0);

    for (nface, face) in poly.faces.iter().enumerate() {
        // For each face f in the original poly

        let centroid = &centers[nface];

        // walk over face vertex-triplets
        let (mut v1, mut v2) = (face[face.len() - 2], face[face.len() - 1]); //  [v1, v2,f.slice(-2);

        let mut vto_int4_ = vec![];
        for v3 in face {
            let t12 = i4_min(v1, v2);
            let ti12 = i4_min_3(nface as u32, v1, v2);
            let t23 = i4_min(v2, *v3);
            let ti23 = i4_min_3(nface as u32, v2, *v3);
            let iv2 = to_int4(v2);

            // for each face-corner, we make two new points:
            let midpt = midpoint(&poly.vertexes[v1 as usize], &poly.vertexes[v2 as usize]);
            let innerpt = midpoint(&midpt, &centroid);

            flag.add_v(&t12, &midpt);
            flag.add_v(&ti12, &innerpt);

            // and add the old corner-vertex
            flag.add_v(&iv2, &poly.vertexes[v2 as usize]);

            // pentagon for each vertex in original face
            flag.add_face_f(&vec![ti12, t12, iv2, t23, ti23]);

            // inner rotated face of same vertex-number as original
            vto_int4_.push(ti12);

            // shift over one
            v1 = v2;
            v2 = *v3;
        }
        flag.fcs.push(vto_int4_);
    }

    flag.to_poly();
    Polyhedron {
        name: format!("q{}", poly.name),
        vertexes: flag.vertexes,
        faces: flag.faces,
    }
}

pub fn insetn(poly: &Polyhedron, n: u32, inset_dist: f32, popout_dist: f32) -> Polyhedron // 0, 0.3, -0.1
{
    let mut flag = Flag::new(0).with_vertexes(&poly.vertexes);

    let normals = poly.calc_normals();
    let centers = poly.calc_centers();

    let mut found_any = false; // alert if don't find any
    for (nface, face) in poly.faces.iter().enumerate() {
        let mut v1 = *face.last().unwrap();

        for v2 in face {
            if face.len() == n as usize || n == 0 {
                found_any = true;

                flag.add_v(
                    &to_int4_3('f' as u32, nface as u32, *v2),
                    &add(
                        &tween(&poly.vertexes[*v2 as usize], &centers[nface], inset_dist),
                        &mulc(&normals[nface], popout_dist),
                    ),
                );

                flag.add_face_f(&vec![
                    to_int4(v1),
                    to_int4(*v2),
                    to_int4_3('f' as u32, nface as u32, *v2),
                    to_int4_3('f' as u32, nface as u32, v1),
                ]);
                // new inset, extruded face
                flag.add_face_m(
                    &to_int4_2('x' as u32, nface as u32),
                    &to_int4_3('f' as u32, nface as u32, v1),
                    &to_int4_3('f' as u32, nface as u32, *v2),
                );
            } else {
                // same old flag, if non-n
                flag.add_face_m(&to_int4(nface as u32), &to_int4(v1), &to_int4(*v2));
            }

            v1 = *v2; // current becomes previous
        }
    }

    if !found_any {
        println!("No {} - fold components were found.", n)
    }

    flag.to_poly();
    Polyhedron {
        name: format!("n{}", poly.name),
        vertexes: flag.vertexes,
        faces: flag.faces,
    }
}

pub fn extruden(poly: &Polyhedron, n: u32) -> Polyhedron {
    let mut newpoly = insetn(poly, n, 0.0, 0.1);
    newpoly.name = format!(
        "x{}{}",
        if n == 0 {
            "".to_string()
        } else {
            n.to_string()
        },
        poly.name
    );
    newpoly
}

pub fn loft(poly: &Polyhedron, n: u32, alpha: f32) -> Polyhedron // 0,0
{
    let mut newpoly = insetn(poly, n, alpha, 0.0);
    newpoly.name = format!(
        "l{}{}",
        if n == 0 {
            "".to_string()
        } else {
            n.to_string()
        },
        poly.name
    );
    newpoly
}

pub fn hollow(poly: &Polyhedron, inset_dist: f32, thickness: f32) -> Polyhedron // 0.2, 0.1
{
    let mut flag = Flag::new(0).with_vertexes(&poly.vertexes);

    let normals = poly.avg_normals();
    let centers = poly.calc_centers();
    let (fin_name, fdwn_name, v_name) = ('i' as u32, 'd' as u32, 'v' as u32);

    for (nface, face) in poly.faces.iter().enumerate() {
        let mut v1 = *poly.faces[nface].last().unwrap();
        let iface = nface as u32;

        for v2 in face {
            let v2 = *v2;
            // new inset vertex for every vert in face
            flag.add_v(
                &to_int4_4(fin_name, iface, v_name, v2),
                &tween(&poly.vertexes[v2 as usize], &centers[nface], inset_dist),
            );
            flag.add_v(
                &to_int4_4(fdwn_name, iface, v_name, v2),
                &sub(
                    &tween(&poly.vertexes[v2 as usize], &centers[nface], inset_dist),
                    &mulc(&unit(&normals[nface]), thickness),
                ),
            );

            flag.add_face_f(&vec![
                to_int4(v1),
                to_int4(v2),
                to_int4_4(fin_name, iface, v_name, v2),
                to_int4_4(fin_name, iface, v_name, v1),
            ]);

            flag.add_face_f(&vec![
                to_int4_4(fin_name, iface, v_name, v1),
                to_int4_4(fin_name, iface, v_name, v2),
                to_int4_4(fdwn_name, iface, v_name, v2),
                to_int4_4(fdwn_name, iface, v_name, v1),
            ]);
            v1 = v2; // current becomes previous
        }
    }
    flag.to_poly();
    Polyhedron {
        name: format!("H{}", poly.name),
        vertexes: flag.vertexes,
        faces: flag.faces,
    }
}

pub fn perspectiva1(poly: &Polyhedron) -> Polyhedron {
    let centers = poly.calc_centers(); // calculate face centers

    let mut flag = Flag::new(0).with_vertexes(&poly.vertexes);

    // iterate over triplets of faces v1,v2,v3
    for (nface, face) in poly.faces.iter().enumerate() {
        let (mut v1, mut v2) = (face[face.len() - 2], face[face.len() - 1]);
        let (mut vert1, mut vert2) = (&poly.vertexes[v1 as usize], &poly.vertexes[v2 as usize]);

        let mut vi4 = vec![];
        for v3 in face {
            let v3 = *v3;

            let vert3 = &poly.vertexes[v3 as usize];
            let v12 = to_int4_2(v1, v2); // names for "oriented" midpoints
            let v21 = to_int4_2(v2, v1);
            let v23 = to_int4_2(v2, v3);

            // on each Nface, N new points inset from edge midpoints towards
            // center = "stellated" points
            flag.add_v(&v12, &midpoint(&midpoint(vert1, vert2), &centers[nface]));

            // inset Nface made of new, stellated points
            vi4.push(v12);

            // new tri face constituting the remainder of the stellated Nface
            flag.add_face_f(&vec![v23, v12, to_int4(v2)]);

            // one of the two new triangles replacing old edge between v1->v2
            flag.add_face_f(&vec![to_int4(v1), v21, v12]);

            v1 = v2;
            v2 = v3; //  [v1, v2,[v2, v3];  // current becomes previous

            vert1 = vert2;
            vert2 = vert3; // [vert1, vert2,[vert2, vert3];
        }
        flag.fcs.push(vi4);
    }

    flag.to_poly();
    Polyhedron {
        name: format!("P{}", poly.name),
        vertexes: flag.vertexes,
        faces: flag.faces,
    }
}
