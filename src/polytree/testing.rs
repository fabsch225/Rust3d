// Define the TreeNode struct
struct TreeNode<'a, T> {
    data: &'a T,
    left: Option<Box<TreeNode<'a, T>>>,
    right: Option<Box<TreeNode<'a, T>>>,
}

// Define the VectorTree struct
pub struct VectorTree<'a, T> {
    root: Option<Box<TreeNode<'a, T>>>,
}

impl<'a, T: Ord> VectorTree<'a, T> {
    // Create a new VectorTree from a vector
    pub fn from_vector(elements: &'a [T]) -> Self {
        let mut tree = VectorTree { root: None };

        for element in elements {
            tree.insert(element);
        }

        tree
    }

    // Insert a value into the tree
    pub fn insert(&mut self, value: &'a T) {
        if let Some(root) = &mut self.root {
            VectorTree::insert_recursive(root, value);
        } else {
            self.root = Some(Box::new(TreeNode {
                data: value,
                left: None,
                right: None,
            }));
        }
    }

    // Recursive function to insert a value into the tree
    fn insert_recursive(node: &mut Box<TreeNode<'a, T>>, value: &'a T) {
        if value < node.data {
            if let Some(left) = &mut node.left {
                VectorTree::insert_recursive(left, value);
            } else {
                node.left = Some(Box::new(TreeNode {
                    data: value,
                    left: None,
                    right: None,
                }));
            }
        } else {
            if let Some(right) = &mut node.right {
                VectorTree::insert_recursive(right, value);
            } else {
                node.right = Some(Box::new(TreeNode {
                    data: value,
                    left: None,
                    right: None,
                }));
            }
        }
    }
}

// Example usage
fn main() {
    let vector = vec![5, 3, 7, 2, 4, 6, 8];
    let tree = VectorTree::from_vector(&vector);

    // Do something with the tree...

    // Note: The tree now contains references to the original data in the vector.
}