use std::cmp;
use std::sync::Arc;
use std::thread;

use crate::tree::{Tree, Triangle};
use crate::vec3::{min_coor, max_coor, Vec3, dot_product, square_distance};

#[derive(Debug)]
struct Light {
    pos: Vec3,
    color: Vec3,
}
pub struct Scene {
    tree: Tree,
    lights: Vec<Light>,
}

fn cos(first: Vec3, middle: Vec3, second: Vec3) -> f32 {
    let dot = dot_product(first-middle, second-middle);
    let d1 = square_distance(first, middle).sqrt();
    let d2 = square_distance(second, middle).sqrt();
    dot/(d1*d2)
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            tree: Tree::new(),
            lights: vec![Light{
                pos: Vec3 {x: 5., y: 5., z: 5.},
                color: Vec3{x: 255., y: 255., z:255.}
            }],
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
    fn get_pixel(tree: &Tree, lights: &Vec<Light>, source: Vec3, middle: Vec3) -> Vec3 {
        let intersect = tree.does_intersect(source, middle);
        if intersect.is_none() {
            return Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            };
        }
        let mut color = Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        let intersect = intersect.unwrap();
        for light in lights {
            let lighh_point = tree.does_intersect(light.pos, intersect);
            if intersect != lighh_point.unwrap() {
                color = max_coor(
                    color,
                    Vec3 {
                        x: light.color.x.min(100.),
                        y: light.color.y.min(100.),
                        z: light.color.z.min(100.),
                    },
                );
            } else {
                color = max_coor(
                    color,
                    Vec3 {
                        x: 100.,
                        y: 100.,
                        z: 100.,
                    },
                );
                color = color + light.color * cos(source, intersect, light.pos);
                color = min_coor(
                    color,
                    Vec3 {
                        x: 255.,
                        y: 255.,
                        z: 255.,
                    },
                );
            }
        }
        return color;
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
        let mut lights_temp: Vec<Light> = Vec::new();
        std::mem::swap(&mut tree_temp, &mut self.tree);
        std::mem::swap(&mut lights_temp, &mut self.lights);
        let tree_arc = Arc::new(tree_temp);
        let light_arc = Arc::new(lights_temp);

        let threads: Vec<std::thread::JoinHandle<Vec<Vec<Vec3>>>> = (0..thread_count)
            .into_iter()
            .map(|idx| {
                let tree = Arc::clone(&tree_arc);
                let light = Arc::clone(&light_arc);
                let min_height = idx * lines_per_thread;
                let max_height = (idx + 1) * lines_per_thread;
                let mut y = bl + dy * min_height as f32;
                thread::spawn(move || {
                    let mut image: Vec<Vec<Vec3>> = Vec::new();
                    for _i in min_height..max_height {
                        let mut x = y;
                        let mut line: Vec<Vec3> = Vec::new();
                        for _j in 0..width {
                            line.push(Scene::get_pixel(&tree, &light, camera, x));
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
        let _ = std::mem::replace(&mut self.lights, Arc::try_unwrap(light_arc).unwrap());

        return image;
    }
}
