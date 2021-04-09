// slower than st version
pub fn kiss_n_mt(poly: &Polyhedron, n: u32, apexdist: f32) -> Polyhedron {
    // 0, 0.1
    let normals = poly.calc_normals();
    let centers = poly.calc_centers();

    let chunk_size = 1024 * 2;

    let flags = poly
        .faces
        .par_chunks(chunk_size)
        .enumerate()
        .map(|(ichnk, face_chunk)| {
            let face_offset = ichnk * chunk_size;
            let mut flag = Flag::new(ichnk);

            for (nface, face) in face_chunk.iter().enumerate() {
                let nface = nface + face_offset;
                let fname = ['f' as u32, nface as u32, 0, 0];

                let mut v1 = face.last().unwrap();

                for v2 in face {
                    let iv2 = [*v2, 0, 0, 0];

                    flag.add_v(&iv2, &poly.vertexes[*v2 as usize]);

                    if n == 0 || face.len() == 0 {
                        flag.add_v(
                            &fname,
                            &add(&centers[nface], &mulc(&normals[nface], apexdist)),
                        );
                        flag.add_face_f(&vec![[*v1, 0, 0, 0], iv2, fname])
                    } else {
                        flag.add_face_m(&[nface as u32, 0, 0, 0], &[*v1, 0, 0, 0], &[*v2, 0, 0, 0])
                    }
                    v1 = v2
                }
            }
            flag.to_poly();
            flag
        })
        .collect::<Vec<Flag>>();

    // combine faces & vertexes
    let mut offset = 0;
    let flag = flags.iter().fold(Flag::new(0), |mut flag_acc, flag| {
        let mut flag = flag.clone();
        flag.add_offset(offset);
        offset += flag.v_index;

        flag_acc.faces.extend(flag.faces.clone());
        flag_acc.vertexes.extend(flag.vertexes.clone());
        flag_acc
    });

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
