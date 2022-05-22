use std::path::Path;
use std::sync::Arc;
use std::thread;

pub mod file_io;
pub mod tree;
pub mod vec3;
pub mod scene;

use file_io::{read_file, write_to_file};
use tree::Tree;
use vec3::Vec3;
use scene::Scene;


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
                    x: 1.,
                    y: 1.,
                    z: 2.,
                },
                Vec3 { x, y, z: 1. },
            );
            let color = if intersect { 250 } else { 0 };
            line.push(Vec3 {
                x: color as f32,
                y: color as f32,
                z: color as f32,
            });

            y += 1.0 / width as f32;
        }
        image.push(line);
        x += 1.0 / height as f32;
    }

    return image;
}


fn main() {
    // main_old();
    let mut scene = Scene::new();
    let shape = read_file(Path::new("c:/Users/user/cow.obj"));
    // let mut tr = Tree::new();
    // for i in &shape {
    //     tr.insert(i);
    // }

    scene.add_to_tree(shape);
    
    // println!("match {}", tr.does_intersect(Vec3 {x: 0., y: 0., z: 2.}, Vec3{x: 0., y: 0., z: 0.}));

    let height = 720;
    let width = 720;
    
    use std::time::Instant;
    let now = Instant::now();
    
    let center = Vec3 {x: 0., y: 0., z: 0.};

    let image = scene.render(2., 0., 0., center, height, width);

    let elapsed = now.elapsed();
    println!("ray tracing time: {:.2?}", elapsed);
    let _res = write_to_file(image);
    let elapsed = now.elapsed();
    println!("writing time: {:.2?}", elapsed);
}