use std::cmp;

pub struct Tree {
    root: Link,
}

struct Node {
    min_value: i32,
    max_value: i32,
    left: Tree,
    right: Tree,
}

impl Node {
    fn update_bounces(&mut self) {
        let (min_left, max_left) = get_bouncies(&self.left);
        let (min_right, max_right) = get_bouncies(&self.right);

        self.min_value = cmp::min(min_left, min_right);
        self.max_value = cmp::max(max_left, max_right);
    }
}

enum Link {
    Empty,
    Number(i32),
    Node(Box<Node>),
}

fn get_bouncies(tree: &Tree) -> (i32, i32) {
    match &tree.root {
        Link::Empty => {
            return (i32::MAX, i32::MIN);
        }
        Link::Number(number) => {
            return (*number, *number);
        }
        Link::Node(node) => {
            return (node.min_value, node.max_value);
        }
    }
}

fn insert_value_into_tree_volume(new_value: i32, insert_tree: &Tree, other_tree: &Tree) -> f32 {
    let (mut min_insert_tree, mut max_insert_tree) = get_bouncies(insert_tree);
    let (min_other_tree, max_other_tree) = get_bouncies(other_tree);

    min_insert_tree = cmp::min(new_value, min_insert_tree);
    max_insert_tree = cmp::max(new_value, max_insert_tree);

    return ((max_insert_tree - min_insert_tree) + (max_other_tree - min_other_tree)) as f32;
}

fn combine_tree_volume(left: &Tree, right: &Tree, other: &Tree) -> f32 {
    let (min_left, max_left) = get_bouncies(left);
    let (min_right, max_right) = get_bouncies(right);
    let (min_other, max_other) = get_bouncies(other);

    return (cmp::max(max_left, max_right) - cmp::min(min_left, min_right) + max_other - min_other)
        as f32;
}

impl Tree {
    pub fn new() -> Tree {
        Tree { root: Link::Empty }
    }
    fn private_insert(&mut self, arg: i32) -> Tree {
        match &mut self.root {
            Link::Empty => {
                self.root = Link::Number(arg);
                return Tree::new();
            }
            Link::Number(_number) => {
                let mut new_tree = Tree::new();
                new_tree.root = Link::Number(arg);
                return new_tree;
            }
            Link::Node(node) => {
                //if there is no right son, I insert ret value of inserting in left there
                if let Link::Empty = node.right.root {
                    node.right = node.left.private_insert(arg);
                    let (potential_min, potential_max) = get_bouncies(&node.right);
                    node.min_value = cmp::min(potential_min, node.min_value);
                    node.max_value = cmp::max(potential_max, node.max_value);

                    return Tree::new();
                }

                //find best child
                let mut new_child = if insert_value_into_tree_volume(arg, &node.left, &node.right)
                    < insert_value_into_tree_volume(arg, &node.right, &node.left)
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
    pub fn insert(&mut self, arg: i32) {
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
            min_value: cmp::min(min_old_value, min_new_value),
            max_value: cmp::max(max_old_value, max_new_value),
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
            Link::Number(number) => println!("{}", number),
            Link::Node(node) => {
                println!("[{} {}]", node.min_value, node.max_value);
                node.left.print(Some(offset + 1));
                node.right.print(Some(offset + 1));
            }
        }
    }
}
