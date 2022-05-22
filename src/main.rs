use std::path::Path;
use std::time::Instant;

pub mod file_io;
pub mod tree;
pub mod vec3;
pub mod scene;

use file_io::{read_file, write_to_file};
use vec3::Vec3;
use scene::Scene;

fn main() {
    let mut scene = Scene::new();
    let now = Instant::now();
    let shape = read_file(Path::new("c:/Users/user/cow.obj"));
    let elapsed = now.elapsed();
    println!("reading time: {:.2?}", elapsed);

    scene.add_to_tree(shape);

    let height = 720;
    let width = 720;
    
    let now = Instant::now();
    
    let center = Vec3 {x: 0., y: 0., z: 0.};
    
    let image = scene.render(6., std::f32::consts::PI/4., 0.*std::f32::consts::PI/3., center, height, width);
    
    let elapsed = now.elapsed();
    println!("ray tracing time: {:.2?}", elapsed);
    let now = Instant::now();
    let _res = write_to_file(image);
    let elapsed = now.elapsed();
    println!("writing time: {:.2?}", elapsed);
}