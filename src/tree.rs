pub struct Tree {
    root: Link
}

struct Node {
    min_value: i32,
    max_value: i32,
    left: Tree,
    right: Tree
}

enum Link {
    Empty,
    Number(i32),
    Node(Box<Node>)
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            root: Link::Empty
        }
    }
    pub fn insert(&mut self, arg: i32) {
        match &mut self.root {
            Link::Empty => {self.root = Link::Number(arg);},
            Link::Number(value) => {
                let mut new_current = Node {
                    min_value: arg,
                    max_value: *value,
                    left: Tree::new(),
                    right: Tree::new()
                };
                new_current.left.insert(arg);
                new_current.right.insert(*value);
                self.root = Link::Node(Box::new(new_current)); 
            }
            Link::Node(node) => {
                node.left.insert(arg);
            }
        }
    }
    pub fn print(&self, default_offset: Option<i32>) {
        let offset = default_offset.unwrap_or(0);
        for _i in 0..offset-1 {
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
                node.left.print(Some(offset+1));
                node.right.print(Some(offset+1));
            }
        }
    }
}
