use crate::vec3::{cross_product, dot_product, max_coor, min_coor, Vec3};

type Triangle = [Vec3; 3];

pub struct Tree {
    root: Link,
}

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
fn triangle_intersection(orig: Vec3, dir: Vec3, triangle: &Triangle) -> f32 {
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

fn ray_square_intersect(source: Vec3, direction: Vec3, vertexes: [Vec3; 4]) -> bool {
    return triangle_intersection(source, direction, &[vertexes[2], vertexes[0], vertexes[1]])
        != 0.
        || triangle_intersection(source, direction, &[vertexes[2], vertexes[0], vertexes[3]])
            != 0.;
}

/*
( 0.0, 0.0, 2.0 )
( 0.22727275, -0.03333487, 1.0 )
( 0.184701, -0.0625733, 0.141112 )
( 0.45416, 0.06693, 0.295318 )
*/

fn ray_cube_intersect_triangle(
    source: Vec3,
    direction: Vec3,
    min_vertex: Vec3,
    max_vertex: Vec3,
) -> bool {
    let mut result = ray_square_intersect(
        source,
        direction,
        [
            Vec3 {
                x: min_vertex.x,
                y: min_vertex.y,
                z: min_vertex.z,
            },
            Vec3 {
                x: max_vertex.x,
                y: min_vertex.y,
                z: min_vertex.z,
            },
            Vec3 {
                x: max_vertex.x,
                y: max_vertex.y,
                z: min_vertex.z,
            },
            Vec3 {
                x: min_vertex.x,
                y: max_vertex.y,
                z: min_vertex.z,
            },
        ],
    );
    result |= ray_square_intersect(
        source,
        direction,
        [
            Vec3 {
                x: min_vertex.x,
                y: min_vertex.y,
                z: min_vertex.z,
            },
            Vec3 {
                x: min_vertex.x,
                y: max_vertex.y,
                z: min_vertex.z,
            },
            Vec3 {
                x: max_vertex.x,
                y: max_vertex.y,
                z: min_vertex.z,
            },
            Vec3 {
                x: max_vertex.x,
                y: min_vertex.y,
                z: min_vertex.z,
            },
        ],
    );
    result |= ray_square_intersect(
        source,
        direction,
        [
            Vec3 {
                x: min_vertex.x,
                y: min_vertex.y,
                z: min_vertex.z,
            },
            Vec3 {
                x: min_vertex.x,
                y: min_vertex.y,
                z: max_vertex.z,
            },
            Vec3 {
                x: max_vertex.x,
                y: min_vertex.y,
                z: max_vertex.z,
            },
            Vec3 {
                x: max_vertex.x,
                y: min_vertex.y,
                z: min_vertex.z,
            },
        ],
    );
    result |= ray_square_intersect(
        source,
        direction,
        [
            Vec3 {
                x: max_vertex.x,
                y: max_vertex.y,
                z: max_vertex.z,
            },
            Vec3 {
                x: max_vertex.x,
                y: min_vertex.y,
                z: max_vertex.z,
            },
            Vec3 {
                x: max_vertex.x,
                y: min_vertex.y,
                z: min_vertex.z,
            },
            Vec3 {
                x: max_vertex.x,
                y: max_vertex.y,
                z: min_vertex.z,
            },
        ],
    );
    result |= ray_square_intersect(
        source,
        direction,
        [
            Vec3 {
                x: max_vertex.x,
                y: max_vertex.y,
                z: max_vertex.z,
            },
            Vec3 {
                x: max_vertex.x,
                y: min_vertex.y,
                z: max_vertex.z,
            },
            Vec3 {
                x: min_vertex.x,
                y: min_vertex.y,
                z: max_vertex.z,
            },
            Vec3 {
                x: min_vertex.x,
                y: max_vertex.y,
                z: max_vertex.z,
            },
        ],
    );
    result |= ray_square_intersect(
        source,
        direction,
        [
            Vec3 {
                x: max_vertex.x,
                y: max_vertex.y,
                z: max_vertex.z,
            },
            Vec3 {
                x: max_vertex.x,
                y: max_vertex.y,
                z: min_vertex.z,
            },
            Vec3 {
                x: min_vertex.x,
                y: max_vertex.y,
                z: min_vertex.z,
            },
            Vec3 {
                x: min_vertex.x,
                y: max_vertex.y,
                z: max_vertex.z,
            },
        ],
    );

    return result;
}

fn ray_cube_intersect_ray_time(
    source: Vec3,
    direction: Vec3,
    min_vertex: Vec3,
    max_vertex: Vec3,
) -> bool {
    let mut t_min = f32::MIN;
    let mut t_max = f32::MAX;

    let mut update_axis_result = |direction: f32, source: f32, min_vertex: f32, max_vertex: f32| {
        let dir = direction - source;
        if dir != 0. {
            let tx1 = (min_vertex - source) / dir;
            let tx2 = (max_vertex - source) / dir;

            t_min = t_min.max(tx1.min(tx2));
            t_max = t_max.min(tx1.max(tx2));
        }
    };

    update_axis_result(direction.x, source.x, min_vertex.x, max_vertex.x);
    update_axis_result(direction.y, source.y, min_vertex.y, max_vertex.y);
    update_axis_result(direction.z, source.z, min_vertex.z, max_vertex.z);
    return t_min <= t_max && 0. <= t_max;
}

fn ray_cube_intersect_cpp_version(
    source: Vec3,
    direction: Vec3,
    min_vertex: Vec3,
    max_vertex: Vec3,
) -> bool {
    let d = Vec3 {
        x: direction.x - source.x,
        y: direction.y - source.y,
        z: direction.z - source.z,
    };

    let update_projection_result = |source: f32,
                                    min_main: f32,
                                    d_main: f32,
                                    d_rest: f32,
                                    source_rest: f32,
                                    min_rest: f32,
                                    max_rest: f32|
     -> bool {
        let mid = (min_main - source) / d_main * d_rest + source_rest;
        return min_rest <= mid && mid <= max_rest;
    };

    macro_rules! update_projection_result {
        ( $main:ident, $rest:ident ) => {
            update_projection_result(
                source.$main,
                min_vertex.$main,
                d.$main,
                d.$rest,
                source.$rest,
                min_vertex.$rest,
                max_vertex.$rest,
            )
        };
    }

    if update_projection_result! {x, y} && update_projection_result! {x, z} {
        return true;
    }
    if update_projection_result! {y, x} && update_projection_result! {y, z} {
        return true;
    }
    if update_projection_result! {z, y} && update_projection_result! {z, x} {
        return true;
    }

    return false;
}

//  (0.0, 0.0,  2.0  )
//  (0.1, 0.1,  1.0  )
//  (0.494932, 0.0233132,  0.163541)
//  (0.50907, 0.029493,  0.178999 )

//TODO create better solution for intersection ray and cube
fn ray_cube_intersect(source: Vec3, direction: Vec3, min_vertex: Vec3, max_vertex: Vec3) -> bool {
    let triangle_version = ray_cube_intersect_triangle(source, direction, min_vertex, max_vertex);
    let ray_time = ray_cube_intersect_ray_time(source, direction, min_vertex, max_vertex);
    let cpp_version = ray_cube_intersect_cpp_version(source, direction, min_vertex, max_vertex);

    // if working != debug {
    //     println!(
    //         "working: {}, debug: {}\n{:?} {:?} {:?} {:?}",
    //         working, debug, source, direction, min_vertex, max_vertex
    //     );
    //     // panic!();
    // }
    return ray_time;
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
    pub fn does_intersect(&self, source: Vec3, direction: Vec3) -> f32 {
        match &self.root {
            Link::Empty => {
                return 0.;
            }
            Link::Triangle(triangle) => {
                return triangle_intersection(source, direction, triangle);
            }
            Link::Node(node) => {
                // println!("{:?} {:?}", node.min_value, node.max_value);
                if ray_cube_intersect(source, direction, node.min_value, node.max_value) {
                    let left_child_res = node.left.does_intersect(source, direction);
                    if left_child_res != 0. {
                        return left_child_res;
                    }
                    return node.right.does_intersect(source, direction);
                }
                return 0.;
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
