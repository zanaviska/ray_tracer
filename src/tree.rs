use crate::vec3::{max_coor, min_coor, Vec3};

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

impl Tree {
    pub fn new() -> Tree {
        Tree { root: Link::Empty }
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
                    let (potential_min, potential_max) = get_bouncies(&node.right);
                    node.min_value = min_coor(potential_min, node.min_value);
                    node.max_value = max_coor(potential_max, node.max_value);

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
        match &new_child.root {
            Link::Empty => {
                return;
            }
            _ => {}
        }

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
