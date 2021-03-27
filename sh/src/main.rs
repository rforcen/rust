// sh disp

extern crate kiss3d;
extern crate nalgebra as na;

use image::{ImageBuffer, Rgb};

use kiss3d::light::Light;
use kiss3d::resource::{Mesh, Texture, TextureManager};
use kiss3d::window::Window;
use na::{Point2, Point3, Vector3};
use rayon::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

mod sh;
use sh::SpericalHarmonics as SH;

fn main() {
    // generate sh n, code, color_map

    let sh = {
        let n = 256 * 2; // use powers of 2 values (2^n)
        let (code, color_map) = (44, 2);
        SH::new(n, code, color_map)
    };
    let mut window = Window::new("Spherical harmonics");

    // convert sh -> mesh
    let scale = {
        let l = 0.1;
        Vector3::new(l, l, l)
    };
    if sh.size <= (1 << 16) {
        let mut node = window.add_mesh(sh.make_mesh(), scale);
        node.set_texture(sh.make_texture());
        node.enable_backface_culling(false);
    } else {
        for (mesh, texture) in sh.make_meshes() {
            let mut node = window.add_mesh(mesh, scale);
            node.set_texture(texture);
            node.enable_backface_culling(false);
        }
    }

    window.set_light(Light::Absolute(Point3::new(0., 0.4, 0.3)));
    while window.render() {}
}

// kiss3d bindings

impl SH {
    fn get_vertices(&self) -> Vec<Point3<f32>> {
        self.shape.par_iter().map(|v| v.position).collect()
    }
    fn get_faces(&self) -> Vec<Point3<u16>> {
        self.indexes
            .par_iter()
            .chunks(3)
            .map(|ix| Point3::new(*ix[0] as u16, *ix[1] as u16, *ix[2] as u16))
            .collect()
    }
    fn get_normals(&self) -> Vec<Vector3<f32>> { // traverse shape.normal & texture
        self.shape.par_iter().map(|v| v.normal).collect()
    }
    fn get_uvs(&self) -> Vec<Point2<f32>> {
        self.shape.par_iter().map(|v| v.texture).collect()
    }
    // mesh size is limited to u16::MAX (2^16) so a bigger shape should be fragmented to this size
    fn make_meshes(&self) -> Vec<(Rc<RefCell<Mesh>>, Rc<Texture>)> {
        let (vertices, faces, normals, uvs) = (
            self.get_vertices(),
            self.get_faces(),
            self.get_normals(),
            self.get_uvs(),
        );

        let chunk_size = 1 << 16; // 2^16, u16::MAX+1

        (0..self.size)
            .step_by(chunk_size) // size/chunk:size + ramiander
            .map(|i| {
                let (start, end) = (i, (i + chunk_size)); // selected range

                (
                    Rc::new(RefCell::new(Mesh::new(
                        vertices[start..end].to_vec(),
                        faces[start * 2..end * 2].to_vec(), // 2 trigs per quad
                        Some(normals[start..end].to_vec()),
                        Some(uvs[start..end].to_vec()),
                        true,
                    ))),
                    self.make_subtexture(start, end),
                )
            })
            .collect()
    }
    fn make_mesh(&self) -> Rc<RefCell<Mesh>> {
        // make a single mesh for # faces < 2^16
        Rc::new(RefCell::new(Mesh::new(
            self.get_vertices(),
            self.get_faces(),
            Some(self.get_normals()),
            Some(self.get_uvs()),
            true,
        )))
    }

    fn make_subtexture(&self, start: usize, end: usize) -> Rc<Texture> {
        fn v3_u8(c: &Point3<f32>) -> [u8; 3] {
            fn f2b(f: f32) -> u8 {
                (f * 255.) as u8
            }
            [f2b(c[0]), f2b(c[1]), f2b(c[2])]
        }
        let n = ((end - start) as f32).sqrt() as u32;

        TextureManager::new().add_image(
            image::DynamicImage::ImageRgb8(ImageBuffer::from_fn(n, n, |x, y| {
                Rgb(v3_u8(&self.shape[(x + y * n) as usize + start].color))
            })),
            "colors",
        )
    }
    // https://github.com/sebcrozet/kiss3d/issues/227
    fn make_texture(&self) -> Rc<Texture> {
        fn v3_u8(c: &Point3<f32>) -> [u8; 3] {
            fn f2b(f: f32) -> u8 {
                (f * 255.) as u8
            }
            [f2b(c[0]), f2b(c[1]), f2b(c[2])]
        }
        let n = self.n as u32;

        TextureManager::new().add_image(
            image::DynamicImage::ImageRgb8(ImageBuffer::from_fn(n, n, |x, y| {
                Rgb(v3_u8(&self.shape[(x + y * n) as usize].color))
            })),
            "colors",
        )
    }
}
