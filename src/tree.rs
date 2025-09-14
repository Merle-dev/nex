use std::fmt::Debug;

#[derive(Debug)]
pub enum TreeValue<R: PartialEq, T> {
    Twig(Tree<R, T>),
    Leaf(T),
}

impl<R: PartialEq, T> TreeValue<R, T> {
    pub fn leaf(&self) -> Option<&T> {
        match self {
            TreeValue::Twig(_) => None,
            TreeValue::Leaf(leaf) => Some(leaf),
        }
    }
}

#[derive(Debug)]
pub struct Tree<R: PartialEq, T> {
    pub branch: Vec<(R, TreeValue<R, T>)>,
}

impl<R: PartialEq + Debug + Clone, T: Debug + Clone> Tree<R, T> {
    pub fn new() -> Self {
        Self { branch: vec![] }
    }
    pub fn get(&self, path: Vec<R>) -> Option<&TreeValue<R, T>> {
        let (head, tail) = path.split_first()?;
        self.branch.iter().fold(None, |acc, (r, v)| {
            if r == head {
                if tail.is_empty() {
                    Some(v)
                } else {
                    match v {
                        TreeValue::Twig(sub_tree) => sub_tree.get(tail.to_vec()),
                        TreeValue::Leaf(_leaf) => Some(v),
                    }
                }
            } else {
                acc
            }
        })
    }
    pub fn add(&mut self, path: Vec<R>, value: T) {
        path.split_first()
            .and_then(|(head, tail)| {
                self.branch.iter_mut().fold(None, |acc, (r, sub_tree)| {
                    if r == head {
                        match sub_tree {
                            TreeValue::Twig(tree) => Some((tree, tail)),
                            TreeValue::Leaf(_) => None,
                        }
                    } else {
                        acc
                    }
                })
            })
            .and_then(|(tree, tail)| Some(tree.add(tail.to_vec(), value.clone())))
            .unwrap_or(self.add_recursive(path, value))
    }
    fn add_recursive(&mut self, path: Vec<R>, value: T) {
        self.branch.push((
            path[0].clone(),
            if path.len() == 1 {
                TreeValue::Leaf(value)
            } else {
                let (_head, tail) = path.split_first().unwrap();
                let mut new_sub_tree = Self::new();
                new_sub_tree.add_recursive(tail.to_vec(), value);
                TreeValue::Twig(new_sub_tree)
            },
        ));
    }
}
