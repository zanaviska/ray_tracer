use std::sync::Arc;
use std::thread;

use crate::tree::{Tree, Triangle};
use crate::vec3::Vec3;

struct Light {
    pos: Vec3,
    color: Vec3,
}
pub struct Scene {
    tree: Tree,
    lights: Vec<Light>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            tree: Tree::new(),
            lights: vec![],
        }
    }
    pub fn add_to_tree(&mut self, shape: Vec<Triangle>) {
        for i in &shape {
            self.tree.insert(i);
        }
    }
    pub fn add_light(&mut self, pos: Vec3, color: Vec3) {
        self.lights.push(Light { pos, color });
    }
    fn get_pixel(tree: &Tree, source: Vec3, middle: Vec3) -> Vec3 {
        let intersect = tree.does_intersect(source, middle);
        let color = if intersect { 250 } else { 0 };
        Vec3 {
            x: color as f32,
            y: color as f32,
            z: color as f32,
        }
    }

    pub fn render(
        &mut self,
        distance: f32,
        alfa: f32,
        beta: f32,
        camera_direction: Vec3,
        height: i32,
        width: u32,
    ) -> Vec<Vec<Vec3>> {
        let width_fov = 80. / 360. * std::f32::consts::PI;
        let height_fov = height as f32 * width_fov / width as f32;
        let mut tr = Vec3 {
            x: distance * (width_fov / 2.).tan().abs(),
            y: distance * (height_fov / 2.).tan().abs(),
            z: 0.,
        };
        let mut camera = Vec3 {
            x: 0.,
            y: 0.,
            z: distance,
        };
        let mut dx = Vec3 {
            x: tr.x * 2. / (width - 1) as f32,
            y: 0.,
            z: 0.,
        };
        let mut dy = Vec3 {
            x: 0.,
            y: tr.y * 2. / (height - 1) as f32,
            z: 0.,
        };

        tr.rotate(alfa, beta);
        camera.rotate(alfa, beta);
        dx.rotate(alfa, beta);
        dy.rotate(alfa, beta);
        camera = camera + camera_direction;
        let bl = tr * (-1.) + camera_direction;

        let thread_count = 11;
        let lines_per_thread = (height + thread_count - 1) / thread_count;

        let mut tree_temp = Tree::new();
        std::mem::swap(&mut tree_temp, &mut self.tree);
        let tree_arc = Arc::new(tree_temp);

        let threads: Vec<std::thread::JoinHandle<Vec<Vec<Vec3>>>> = (0..thread_count)
            .into_iter()
            .map(|idx| {
                let tree = Arc::clone(&tree_arc);
                let min_height = idx * lines_per_thread;
                let max_height = (idx + 1) * lines_per_thread;
                let mut y = bl + dy * min_height as f32;
                thread::spawn(move || {
                    let mut image: Vec<Vec<Vec3>> = Vec::new();
                    for _i in min_height..max_height {
                        let mut x = y;
                        let mut line: Vec<Vec3> = Vec::new();
                        for _j in 0..width {
                            line.push(Scene::get_pixel(&tree, camera, x));
                            x = x + dx;
                        }
                        image.push(line);
                        y = y + dy;
                    }
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

        let _ = std::mem::replace(&mut self.tree, Arc::try_unwrap(tree_arc).unwrap());

        return image;
    }
}
