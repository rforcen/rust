
use crate::vertex::*;
use std::fs::File;
use std::io::BufWriter;
use std::io::*;

pub struct Wrl {}

impl Wrl {
    pub fn write_indexed_faceset(path: &str, shape: &Vec<Vertex>, faces: &Vec<Vec<u32>>) {
        let mut bw = BufWriter::new(File::create(path).unwrap());
        // header
        bw.write(
            "#VRML V2.0 utf8 

# Spherical Harmonics : {file}            

# lights on
DirectionalLight {  direction -.5 -1 0   intensity 1  color 1 1 1 }
DirectionalLight {  direction  .5  1 0   intensity 1  color 1 1 1 }
           
Shape {
    # default material
    appearance Appearance {
        material Material { }
    }
    geometry IndexedFaceSet {
        
        coord Coordinate {
            point [\n".replace("{file}", path)
                .as_bytes(),
        )
        .unwrap();

        // coords
        for s in shape {
            let p = s.position;
            bw.write(format!("{:.3} {:.3} {:.3},\n", p[0], p[1], p[2]).as_bytes())
                .unwrap();
        }
        bw.write(
            "]
        }
        color Color {
            color ["
                .as_bytes(),
        )
        .unwrap();

        // colors
        for s in shape {
            let p = s.color;
            bw.write(format!("{:.3} {:.3} {:.3},\n", p[0], p[1], p[2]).as_bytes())
                .unwrap();
        }
        bw.write(
            "]
        }
        normal Normal {
            vector ["
                .as_bytes(),
        )
        .unwrap();
        // normals
        for s in shape {
            let p = s.normal;
            bw.write(format!("{:.3} {:.3} {:.3},\n", p[0], p[1], p[2]).as_bytes())
                .unwrap();
        }
        bw.write(
            "]
        }
        coordIndex [\n"
                .as_bytes(),
        )
        .unwrap();
        // faces
        for face in faces {
            for ix in face {
                bw.write(format!("{},", ix).as_bytes()).unwrap();
            }
            bw.write(format!("-1,\n",).as_bytes()).unwrap();
        }
        bw.write(
            "]
        colorPerVertex TRUE
        convex TRUE
        solid TRUE
    }
}"
            .as_bytes(),
        )
        .unwrap();
        bw.flush().unwrap();
    }
}
