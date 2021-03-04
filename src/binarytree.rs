type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    value: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node{
            value: value,
            left: None,
            right: None,
        }
    }
}

pub struct BinaryTree<T: Ord> {
    head: Link<T>,
} 

impl<T: Ord> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree {
            head: None,
        }
    }   
    
    pub fn locate(&self, value: &T) -> &Link<T> {
        let mut link = &self.head; 
        loop {
            match link.as_ref() {
                Some(node) => {
                    if node.value < *value {
                        link = &node.right;
                    } else if node.value > *value {
                        link = &node.left;
                    } else {
                        return link;
                    }
                },
                None => {return link;}
            }
        }
    }

    pub fn locate_mut(&mut self, value: &T) -> &mut Link<T> {
        let mut link = &mut self.head; 
        loop {
            match link.as_ref() {

                Some(node) if node.value < *value => {
                    let right_node = &mut link.as_mut().unwrap().right;
                    link = right_node;
                },
                Some(node) if node.value > *value => {
                    let left_node = &mut link.as_mut().unwrap().left;
                    link = left_node;
                },
                _ => {return link;}
            }
        }
    }

    pub fn push(&mut self, value: T) {
        let push_node = self.locate_mut(&value);
        match push_node {
            Some(_) => {},
            None => {
                let new_node = Box::new(Node::new(value));
                *push_node = Some(new_node);
            },
        }
    }

    pub fn has_value(&self, value: &T) -> bool {
        let has_value = match self.locate(value) {
            Some(_) => {true},
            None => false,
        };
        has_value
    }

    pub fn delete(&mut self, value: &T) {
        let delete_link = self.locate_mut(value);
        delete_node(delete_link);
    }

    pub fn iter(&mut self) -> Iter<T> {
        Iter{current: &self.head, stack: vec![]}
    }

    pub fn into_iter(self) -> IntoIter<T>{
        IntoIter{current: self.head, stack: vec![]}
    }
}

fn delete_node<T: Ord>(link: &mut Link<T>) {
    if link.is_none() { 
        return;
    }

    let mut node = link.take().unwrap();
    match (node.left.take(), node.right.take()) {
        // 構造体への参照の場合, rustのボローチェッカーはメンバが1つでも借用中だとエラーになる
        // 構造体そのものの場合, ボローチェッカーは正常に機能する
        (Some(left_node), Some(right_node)) => {
            // ここでnodeを構造体として再構成する
            node.right = Some(right_node);
            node.left = Some(left_node);
            let swap_link = find_successor(& mut node.right);
            // &Box<Node<T>>だと借用中と判断されてswapできない
            std::mem::swap(&mut node.value, &mut swap_link.as_mut().unwrap().value);
            delete_node(swap_link);            
            *link = Some(node); 
        },
        (Some(left_node), None) => {
            *link = Some(left_node);
        },
        (None, Some(right_node)) => {
            *link = Some(right_node);
        },
        (None, None) => { link.take(); },
    }
}

fn find_successor<T: Ord>(mut link: &mut Link<T>) -> &mut Link<T> {
    loop {
        match link.as_ref() {
            Some(node) => {
                if node.left.is_none() {
                    return link;
                } else {
                    link = &mut link.as_mut().unwrap().left;
                }
            }
            None => {return link;}
        }
    }
}

pub struct Iter<'a, T> {
    current: &'a Link<T>,
    stack: Vec<&'a Node<T>>
}

impl<'a, T> Iterator for  Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item>{
        while let Some(node) = self.current {
            self.stack.push(node);
            self.current = &node.left;
        }
        self.stack.pop().map(|node| {
            self.current = &node.right;
            &node.value
        })
    }
}

pub struct IntoIter<T> {
    current: Link<T>,
    stack: Vec<Box<Node<T>>>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(mut node) = self.current.take() {
            self.current = node.left.take();
            self.stack.push(node);
        }
        self.stack.pop().map(|mut node| {
            self.current = node.right.take();
            node.value
        })
    }
}

#[test]
fn binary_tree_test() {
    let mut binary_tree: BinaryTree<i32> = BinaryTree::new();
    binary_tree.push(100);
    binary_tree.push(150);
    binary_tree.push(120);
    binary_tree.push(130);
    binary_tree.push(125);
    binary_tree.push(175);
    binary_tree.push(50);
    binary_tree.push(25);
    
    binary_tree.delete(&100);

    for value in binary_tree.into_iter() {
        println!("{}", value);
    }

}