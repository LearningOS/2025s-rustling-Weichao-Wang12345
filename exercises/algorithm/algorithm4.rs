/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::fmt::Debug;

#[derive(Clone)]
#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord+Clone,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord+Clone,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord+Clone,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T:Clone > BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
       let bo = self.search(value.clone());
       if bo{
        return ;
       }else {
        let new_node = Box::new(TreeNode{
            value:value.clone(),
            right:None,
            left:None,
        });
        if self.root.is_none() {
            self.root=Some(new_node);
            return ;
        }else{
            let mut cur_node_opt=& mut self.root;
            while let Some( node)=cur_node_opt{
                if value>node.value
                {
                    
                    if let Some(_)=node.right{
                        cur_node_opt=&mut node.right;
                    }else{
                        node.right=Some(new_node.clone());
                       break;
                      
                    }
    
                }
                else
                {
                    if let Some(_)=node.left{
                        cur_node_opt=&mut node.left;
                    }else{
                        node.left=Some(new_node.clone());
                         break;
                    }
                   
                }
            }
            
        }
       }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        let mut cur_node_opt=&self.root;
        while let Some(node)=cur_node_opt{
            if value>node.value
            {
                cur_node_opt=&node.right;

            }
            else if value<node.value
            {
                cur_node_opt=&node.left;
            }else if value==node.value {
                return true
            }
        }
         false
    }
}

impl<T> TreeNode<T>
where
    T: Ord+Clone,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


