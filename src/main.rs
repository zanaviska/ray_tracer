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
                x: (intersect.abs() - 1.) * 150.,
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
    let shape = read_file(Path::new("c:/Users/user/cow.obj"));
    let mut tr = Tree::new();
    for i in &shape {
        tr.insert(i);
    }
    use std::time::Instant;
    let now = Instant::now();

    let height = 720;
    let width = 720;

    const THREAD_COUNT: usize = 11;

    // tr.func();
    // return;

    // let intersect = tr.does_intersect(
    //     Vec3 {
    //         x: 0.,
    //         y: 0.,
    //         z: 2.,
    //     },
    //     Vec3 {
    //         x: 0.1,
    //         y: 0.1,
    //         z: 1.,
    //     },
    // );
    // println!("{:?}", intersect);
    // return;
    /*
    min {x:0.0122379996, y:-0.000986000057, z: -0.1111111999}
    max {x:0.25218001, y:0.0871800035, z:0.220054001}
    */
    // (-0.1, 0.05)
    // return;

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

    let elapsed = now.elapsed();
    println!("ray tracing time: {:.2?}", elapsed);
    let _res = write_to_file(image);
    let elapsed = now.elapsed();
    println!("writing time: {:.2?}", elapsed);
}
