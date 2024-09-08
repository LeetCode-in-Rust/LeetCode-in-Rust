// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Array #Hash_Table #Tree #Binary_Tree
// #Divide_and_Conquer #Data_Structure_II_Day_15_Tree #Big_O_Time_O(N)_Space_O(N)
// #2024_09_08_Time_2_ms_(84.72%)_Space_2.9_MB_(25.00%)

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
	pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
		let mut map = HashMap::new(); // Creating map
		let n = inorder.len(); 
		for i in 0..n {
			map.insert(inorder[i], i); // storing index of inorder value for later purpose
		}
		return Self::f(0, n-1, 0, n-1, &preorder, &inorder, &map);  
	}
	fn f(pre_st:usize,pre_en:usize,in_st:usize,in_en:usize,preorder:&Vec<i32>,
		inorder:&Vec<i32>,map:&HashMap<i32,usize>)->Option<Rc<RefCell<TreeNode>>> {
			if pre_st > pre_en || in_st > in_en {return None;} // if there is no node to make return none
			let root = Rc::new(RefCell::new(TreeNode{ val : preorder[pre_st], left : None, right : None}));
			let ind = map.get(&root.borrow().val); // using the map to get the index of root in inorder vector
			let len = ind.unwrap() - in_st; // lenght of left subtree
			root.borrow_mut().left = Self::f(pre_st+1,pre_st+len,in_st,ind.unwrap()-1,preorder,inorder,map);
			root.borrow_mut().right = Self::f(pre_st+len+1,pre_en,ind.unwrap()+1,in_en,preorder,inorder,map);
			return Some(root);
	}
}
