use crate::vec3::{cross_product, dot_product, max_coor, min_coor, Vec3};

pub type Triangle = [Vec3; 3];

#[derive(Debug)]
pub struct Tree {
    root: Link,
}

#[derive(Debug)]
struct Node {
    min_value: Vec3,
    max_value: Vec3,
    left: Tree,
    right: Tree,
}

impl Node {
    fn update_bounces(&mut self) {
        let (min_left, max_left) = get_bouncies(&self.left);
        let (min_right, max_right) = get_bouncies(&self.right);

        self.min_value = min_coor(min_left, min_right);
        self.max_value = max_coor(max_left, max_right);
    }
}

#[derive(Debug)]
enum Link {
    Empty,
    Triangle(Triangle),
    Node(Box<Node>),
}

fn get_bouncies(tree: &Tree) -> (Vec3, Vec3) {
    match &tree.root {
        Link::Empty => {
            return (Vec3::MAX, Vec3::MIN);
        }
        Link::Triangle(triangle) => {
            return (
                min_coor((*triangle)[2], min_coor((*triangle)[0], (*triangle)[1])),
                max_coor((*triangle)[2], max_coor((*triangle)[0], (*triangle)[1])),
            );
        }
        Link::Node(node) => {
            return (node.min_value, node.max_value);
        }
    }
}

fn vec_volume(vec: Vec3) -> f32 {
    vec.x * vec.y * vec.z
}

fn insert_value_into_tree_volume(
    new_value: Triangle,
    insert_tree: &Tree,
    other_tree: &Tree,
) -> f32 {
    let (mut min_insert_tree, mut max_insert_tree) = get_bouncies(insert_tree);
    let (min_other_tree, max_other_tree) = get_bouncies(other_tree);

    min_insert_tree = min_coor(
        min_coor(new_value[0], min_insert_tree),
        min_coor(new_value[1], new_value[2]),
    );
    max_insert_tree = max_coor(
        max_coor(new_value[0], max_insert_tree),
        max_coor(new_value[1], new_value[2]),
    );

    return vec_volume(max_insert_tree - min_insert_tree)
        + vec_volume(max_other_tree - min_other_tree);
}

fn combine_tree_volume(left: &Tree, right: &Tree, other: &Tree) -> f32 {
    let (min_left, max_left) = get_bouncies(left);
    let (min_right, max_right) = get_bouncies(right);
    let (min_other, max_other) = get_bouncies(other);

    return vec_volume(max_coor(max_left, max_right) - min_coor(min_left, min_right))
        + vec_volume(max_other - min_other);
}

// https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
fn triangle_intersection(orig: Vec3, dir: Vec3, triangle: &Triangle) -> Option<Vec3> {
    let e1 = triangle[1] - triangle[0];
    let e2 = triangle[2] - triangle[0];

    //get normal line
    let pvec = cross_product(dir, e2);
    let det = dot_product(e1, pvec);

    //ray is parallel to the plane
    if det < 1e-8 && det > -1e-8 {
        return None;
    }

    let inv_det = 1. / det;
    let tvec = orig - triangle[0];
    let u = dot_product(tvec, pvec) * inv_det;
    if u < 0. || u > 1. {
        return None;
    }

    let qvec = cross_product(tvec, e1);
    let v = dot_product(dir, qvec) * inv_det;
    if v < 0. || u + v > 1. {
        return None;
    }

    let t = dot_product(e2, qvec) * inv_det;
    return Some(orig + dir*t);
}

fn ray_cube_intersect(
    origin: Vec3,
    direction: Vec3,
    min_values: Vec3,
    max_values: Vec3,
) -> bool {
    let mut txmin = (min_values.x - origin.x) / direction.x;
    let mut txmax = (max_values.x - origin.x) / direction.x;

    if txmin > txmax {
        std::mem::swap(&mut txmin, &mut txmax);
    }

    let mut tymin = (min_values.y - origin.y) / direction.y;
    let mut tymax = (max_values.y - origin.y) / direction.y;

    if tymin > tymax {
        std::mem::swap(&mut tymin, &mut tymax);
    }

    if (txmin > tymax) || (tymin > txmax) {
        return false;
    }

    if tymin > txmin {
        txmin = tymin;
    }
    if tymax < txmax {
        txmax = tymax;
    }
    let mut tzmin = (min_values.z - origin.z) / direction.z;
    let mut tzmax = (max_values.z - origin.z) / direction.z;

    if tzmin > tzmax {
        std::mem::swap(&mut tzmin, &mut tzmax);
    }

    if (txmin > tzmax) || (tzmin > txmax) {
        return false;
    }
    return true;
}

impl Tree {
    pub fn new() -> Tree {
        Tree { root: Link::Empty }
    }
    pub fn func(&self) {
        println!(
            "constructor {:?}",
            ray_cube_intersect(
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 2.0
                },
                Vec3 {
                    x: 0.22727275,
                    y: -0.03333487,
                    z: 1.0
                },
                Vec3 {
                    x: 0.184701,
                    y: -0.0625733,
                    z: 0.141112
                },
                Vec3 {
                    x: 0.45416,
                    y: 0.06693,
                    z: 0.295318
                }
            )
        );
    }
    pub fn does_intersect(&self, source: Vec3, middle: Vec3) -> bool {
        match &self.root {
            Link::Empty => {
                return false;
            }
            Link::Triangle(triangle) => {
                return triangle_intersection(source, middle - source, triangle).is_some();
            }
            Link::Node(node) => {
                // println!("{:?} {:?}", node.min_value, node.max_value);
                if ray_cube_intersect(source, middle - source, node.min_value, node.max_value) {
                    let left_child_res = node.left.does_intersect(source, middle);
                    if left_child_res {
                        return left_child_res;
                    }
                    return node.right.does_intersect(source, middle);
                }
                return false;
            }
        }
    }
    fn private_insert(&mut self, arg: &Triangle) -> Tree {
        match &mut self.root {
            Link::Empty => {
                self.root = Link::Triangle(*arg);
                return Tree::new();
            }
            Link::Triangle(_triabgle) => {
                let mut new_tree = Tree::new();
                new_tree.root = Link::Triangle(*arg);
                return new_tree;
            }
            Link::Node(node) => {
                //if there is no right son, I insert ret value of inserting in left there
                if let Link::Empty = node.right.root {
                    node.right = node.left.private_insert(arg);

                    node.update_bounces();
                    return Tree::new();
                }

                //find best child
                let mut new_child = if insert_value_into_tree_volume(*arg, &node.left, &node.right)
                    < insert_value_into_tree_volume(*arg, &node.right, &node.left)
                {
                    node.left.private_insert(arg)
                } else {
                    node.right.private_insert(arg)
                };

                // if we have no new child, update our ranges end finish
                match &new_child.root {
                    Link::Empty => {
                        node.update_bounces();
                        return new_child;
                    }
                    _ => {}
                }

                //find best child combination
                if combine_tree_volume(&new_child, &node.right, &node.left)
                    < combine_tree_volume(&node.left, &node.right, &new_child)
                {
                    std::mem::swap(&mut new_child, &mut node.left);
                }
                if combine_tree_volume(&node.left, &new_child, &node.right)
                    < combine_tree_volume(&node.left, &node.right, &new_child)
                {
                    std::mem::swap(&mut new_child, &mut node.right);
                }

                node.update_bounces();

                //create return value
                let (min_value, max_value) = get_bouncies(&new_child);
                let mut ret_tree = Tree::new();
                ret_tree.root = Link::Node(Box::new(Node {
                    min_value,
                    max_value,
                    left: new_child,
                    right: Tree::new(),
                }));
                return ret_tree;
            }
        }
    }
    pub fn insert(&mut self, arg: &Triangle) {
        let new_child = self.private_insert(arg);

        //if we don't have new branch end execution
        match &new_child.root {
            Link::Empty => {
                return;
            }
            _ => {}
        }

        //create new root, where one branch is old tree and another is new branch
        let (min_old_value, max_old_value) = get_bouncies(&self);
        let (min_new_value, max_new_value) = get_bouncies(&new_child);
        let new_root = Link::Node(Box::new(Node {
            min_value: min_coor(min_old_value, min_new_value),
            max_value: max_coor(max_old_value, max_new_value),
            left: std::mem::replace(self, Tree::new()),
            right: new_child,
        }));
        self.root = new_root;
    }
    pub fn print(&self, default_offset: Option<i32>) {
        let offset = default_offset.unwrap_or(0);
        for _i in 0..offset - 1 {
            print!("|");
        }
        if offset != 0 {
            print!("└");
        }
        match &self.root {
            Link::Empty => println!("Empty"),
            Link::Triangle(triangle) => println!("{:?}", triangle),
            Link::Node(node) => {
                println!("[{:?} {:?}]", node.min_value, node.max_value);
                node.left.print(Some(offset + 1));
                node.right.print(Some(offset + 1));
            }
        }
    }
}
