use std::path::Path;
use std::sync::Arc;
use std::thread;

pub mod file_io;
pub mod tree;
pub mod vec3;

use file_io::{read_file, write_to_file};
use tree::Tree;
use vec3::Vec3;

fn get_image_part(
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
    height: i32,
    width: i32,
    tree: &Tree,
) -> Vec<Vec<Vec3>> {
    let mut image: Vec<Vec<Vec3>> = Vec::new();
    let mut x = x_min;

    while x < x_max {
        let mut y = y_min;
        let mut line: Vec<Vec3> = Vec::new();
        while y < y_max {
            let intersect = tree.does_intersect(
                Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 2.,
                },
                Vec3 { x, y, z: 1. },
            );

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
    let shape = read_file(Path::new("cow.obj"));

    let mut tr = Tree::new();

    for i in &shape {
        tr.insert(i);
        // tr.print(None);
        // println!();
    }

    let height = 1080;
    let width = 1920;

    const THREAD_COUNT: usize = 11;
    // let intersect = tr.does_intersect(
    //     Vec3 {
    //         x: 0.,
    //         y: 0.,
    //         z: 2.,
    //     },
    //     Vec3 {
    //         x: -0.1,
    //         y: 0.05,
    //         z: 1.,
    //     },
    // );
    // println!("{:?}", intersect);
    // (-0.1, 0.05)

    let x_min = -0.5;
    let x_max = 0.5;
    let y_min = -0.5;
    let y_max = 0.5;

    let tr_arc = Arc::new(tr);
    let threads: Vec<std::thread::JoinHandle<Vec<Vec<vec3::Vec3>>>> = (0..THREAD_COUNT)
        .into_iter()
        .map(|idx| {
            let tree = Arc::clone(&tr_arc);
            thread::spawn(move || {
                let x_diff = (x_max - x_min) / THREAD_COUNT as f32;
                let image = get_image_part(
                    x_min + x_diff * idx as f32,
                    x_min + x_diff * (idx as f32 + 1.),
                    y_min,
                    y_max,
                    height,
                    width,
                    &*tree,
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
