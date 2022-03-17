use std::path::Path;
use std::sync::Arc;
use std::thread;

pub mod file_io;
pub mod tree;
pub mod vec3;

use file_io::{read_file, triangle_intersection, write_to_file};
use tree::Tree;
use vec3::Vec3;

fn get_image_part(
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
    height: i32,
    width: i32,
    shape: &Vec<[Vec3; 3]>,
) -> Vec<Vec<Vec3>> {
    let mut image: Vec<Vec<Vec3>> = Vec::new();
    let mut x = x_min;

    while x < x_max {
        let mut y = y_min;
        let mut line: Vec<Vec3> = Vec::new();
        while y < y_max {
            let intersect = shape.iter().fold(false, |acc, cur| {
                acc | (triangle_intersection(
                    Vec3 {
                        x: 0.,
                        y: 0.,
                        z: 2.,
                    },
                    Vec3 { x, y, z: 1. },
                    cur,
                ) != 0.)
            });
            line.push(Vec3 {
                x: (intersect as i32 as f32) * 255.,
                y: 1.0,
                z: 0.0,
            });

            y += 1.0 / width as f32;
        }
        image.push(line);
        x += 1.0 / height as f32;
    }

    return image;
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    let shape = read_file(Path::new("cow1.obj"));

    let mut tr = Tree::new();
    tr.print(None);

    for i in &shape {
        tr.insert(i);
        tr.print(None);
        println!();
    }

    let height = 720;
    let width = 720;

    const THREAD_COUNT: usize = 11;

    let x_min = -0.5;
    let x_max = 0.5;
    let y_min = -0.5;
    let y_max = 0.5;

    let shape_arc = Arc::new(shape);
    let threads: Vec<std::thread::JoinHandle<Vec<Vec<vec3::Vec3>>>> = (0..THREAD_COUNT)
        .into_iter()
        .map(|idx| {
            let shape_counter = Arc::clone(&shape_arc);
            thread::spawn(move || {
                let x_diff = (x_max - x_min) / THREAD_COUNT as f32;
                let image = get_image_part(
                    x_min + x_diff * idx as f32,
                    x_min + x_diff * (idx as f32 + 1.),
                    y_min,
                    y_max,
                    height,
                    width,
                    &*shape_counter,
                );
                return image;
            })
        })
        .collect();

    let image = threads
        .into_iter()
        .fold(Vec::<Vec<Vec3>>::new(), |mut acc, cur| {
            let current = cur.join().unwrap();
            acc.splice(acc.len().., current.into_iter());
            return acc;
        });

    let _res = write_to_file(image);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
