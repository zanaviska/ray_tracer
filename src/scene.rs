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

    pub fn render(
        &self,
        distance: f32,
        alfa: f32,
        beta: f32,
        camera_direction: Vec3,
        height: i32,
        width: u32,
    ) -> Vec<Vec<Vec3>> {
        let width_fov = 80. / 360. * std::f32::consts::PI;
        let height_fov = height as f32 * width_fov / width as f32;
        let before_rotate = Vec3 {
            x: distance * (width_fov / 2.).tan().abs(),
            y: distance * (height_fov / 2.).tan().abs(),
            z: 0.,
        };
        let camera = Vec3 {
            x: 0.,
            y: 0.,
            z: distance,
        };
        let dx = Vec3 {
            x: before_rotate.x * 2. / (width - 1) as f32,
            y: 0.,
            z: 0.,
        };
        let dy = Vec3 {
            x: 0.,
            y: before_rotate.y * 2. / (height - 1) as f32,
            z: 0.,
        };

        //todo add rotation

        let mut image: Vec<Vec<Vec3>> = Vec::new();
        let mut y = before_rotate * (-1.);
        for _i in 0..height {
            let mut x = y;
            let mut line: Vec<Vec3> = Vec::new();
            for _j in 0..width {
                let intersect = self.tree.does_intersect(camera, x);
                let color = if intersect { 250 } else { 0 };
                line.push(Vec3 {
                    x: color as f32,
                    y: color as f32,
                    z: color as f32,
                });
                x = x + dx;
            }
            image.push(line);
            y = y + dy
        }
        image
    }
}
