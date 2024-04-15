use std::cmp::Ordering;

type Link = Box<Node>;
struct Node {
    elem: i32,
    right: Option<Link>,
    left: Option<Link>,
    count: u32,
}

pub struct Three {
    root: Option<Link>,
}

impl Three {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn add(&mut self, elem: i32) {
        match self.root.as_mut() {
            Some(root) => add_recursive(root, elem),
            None => self.root = Node::new(elem),
        };
    }

    pub fn print(&self) {
        self.print_recursive(self.root.as_ref())
    }

    fn print_recursive(&self, nodo: Option<&Box<Node>>) {
        match nodo {
            Some(nodo) => {
                println!("elem: {}, count: {}", nodo.elem, nodo.count);
                if let Some(right) = nodo.right.as_ref() {
                    self.print_recursive(Some(right));
                }

                if let Some(left) = nodo.left.as_ref() {
                    self.print_recursive(Some(left));
                }
            }
            None => println!("End"),
        }
    }
}

fn add_recursive(nodo: &mut Box<Node>, elem: i32) {
    match elem.cmp(&nodo.elem) {
        Ordering::Greater => match nodo.right.as_mut() {
            Some(right_nodo) => add_recursive(right_nodo, elem),
            None => nodo.right = Node::new(elem),
        },
        Ordering::Less => match nodo.left.as_mut() {
            Some(left_nodo) => add_recursive(left_nodo, elem),
            None => nodo.left = Node::new(elem),
        },
        Ordering::Equal => {
            nodo.count += 1;
        }
    }
}

impl Node {
    fn new(elem: i32) -> Option<Box<Self>> {
        Some(Box::new(Self {
            elem,
            left: None,
            right: None,
            count: 1,
        }))
    }
}
