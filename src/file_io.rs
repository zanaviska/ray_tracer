use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub use crate::vec3::{cross_product, dot_product, Vec3};

type Triangle = [Vec3; 3];

pub fn read_file(p: &Path) -> Vec<Triangle> {
    let f = File::open(p).expect("Unable to open file");
    let f = BufReader::new(f);

    let mut vertexes: Vec<Vec3> = Vec::new();
    let mut shape: Vec<Triangle> = Vec::new();

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let mut it = line.split(' ');
        match it.next() {
            Some("v") => {
                let coor = Vec3 {
                    x: it.next().unwrap().parse::<f32>().unwrap(),
                    y: it.next().unwrap().parse::<f32>().unwrap(),
                    z: it.next().unwrap().parse::<f32>().unwrap(),
                };
                vertexes.push(coor);
            }
            Some("f") => {
                let c1 = vertexes[it
                    .next()
                    .unwrap()
                    .split('/')
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
                    - 1];
                let c2 = vertexes[it
                    .next()
                    .unwrap()
                    .split('/')
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
                    - 1];
                while let Some(ind) = it.next() {
                    let c3 = vertexes[ind.split('/').next().unwrap().parse::<usize>().unwrap() - 1];
                    shape.push([c1, c2, c3]);
                }
            }
            _ => {}
        }
    }
    return shape;
}

// https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
pub fn triangle_intersection(orig: Vec3, dir: Vec3, triangle: &Triangle) -> f32 {
    let e1 = triangle[1] - triangle[0];
    let e2 = triangle[2] - triangle[0];

    //get normal line
    let pvec = cross_product(dir, e2);
    let det = dot_product(e1, pvec);

    //ray is parallel to the plane
    if det < 1e-8 && det > -1e-8 {
        return 0.;
    }

    let inv_det = 1. / det;
    let tvec = orig - triangle[0];
    let u = dot_product(tvec, pvec) * inv_det;
    if u < 0. || u > 1. {
        return 0.;
    }

    let qvec = cross_product(tvec, e1);
    let v = dot_product(dir, qvec) * inv_det;
    if v < 0. || u + v > 1. {
        return 0.;
    }

    dot_product(e2, qvec) * inv_det
}

pub fn write_to_file(image: Vec<Vec<Vec3>>) -> std::io::Result<()> {
    let height = image.len();
    let width = image[0].len();

    let filesize = 54 + 3 * height * width;

    let mut bmp_file_header: [u8; 14] = [0; 14];
    bmp_file_header[0] = b'B';
    bmp_file_header[1] = b'M';
    bmp_file_header[2] = filesize as u8;
    bmp_file_header[3] = (filesize >> 8) as u8;
    bmp_file_header[4] = (filesize >> 16) as u8;
    bmp_file_header[5] = (filesize >> 24) as u8;
    bmp_file_header[10] = 54u8;

    let mut bmp_info_header: [u8; 40] = [0; 40];
    bmp_info_header[0] = 40u8;

    bmp_info_header[4] = width as u8;
    bmp_info_header[5] = (width >> 8) as u8;
    bmp_info_header[6] = (width >> 16) as u8;
    bmp_info_header[7] = (width >> 24) as u8;
    bmp_info_header[8] = height as u8;
    bmp_info_header[9] = (height >> 8) as u8;
    bmp_info_header[10] = (height >> 16) as u8;
    bmp_info_header[11] = (height >> 24) as u8;

    bmp_info_header[12] = 1u8;
    bmp_info_header[14] = 24u8;

    let mut file = File::create("image.bmp")?;
    file.write_all(&bmp_file_header)?;
    file.write_all(&bmp_info_header)?;

    for i in image {
        let len = i.len();
        for j in i {
            file.write_all(&[j.x as u8, j.y as u8, j.z as u8])?;
        }
        file.write_all(&vec![0u8; 3 - (len * 3 - 1) % 4])?;
    }

    Ok(())
}
